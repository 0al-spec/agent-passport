use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use base64::engine::general_purpose::{STANDARD, STANDARD_NO_PAD};
use base64::Engine;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sha2::{Digest, Sha256, Sha512};

use crate::model::{
    AgentIntegrity, Capability, CapabilitySignature, CodeHash, Document, FileSystemAccess,
    InboundNetwork, Metadata, Network, NetworkRestrictions, OutboundNetwork, Passport, Resources,
    SecurityPolicies, Signature, Spec,
};

#[derive(Debug, Clone)]
pub struct CheckOptions {
    pub integrity: IntegrityMode,
}

impl Default for CheckOptions {
    fn default() -> Self {
        Self {
            integrity: IntegrityMode::StructureOnly,
        }
    }
}

#[derive(Debug, Clone)]
pub enum IntegrityMode {
    StructureOnly,
    VerifyFiles { root: PathBuf },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CheckSeverity {
    Error,
    Warning,
}

impl fmt::Display for CheckSeverity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Error => formatter.write_str("error"),
            Self::Warning => formatter.write_str("warning"),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PassportCheck {
    pub severity: CheckSeverity,
    pub path: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ValidationReport {
    pub path: String,
    pub valid: bool,
    pub checks: Vec<PassportCheck>,
}

impl ValidationReport {
    fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            valid: true,
            checks: Vec::new(),
        }
    }

    fn error(&mut self, path: impl Into<String>, message: impl Into<String>) {
        self.valid = false;
        self.checks.push(PassportCheck {
            severity: CheckSeverity::Error,
            path: path.into(),
            message: message.into(),
        });
    }

    fn warning(&mut self, path: impl Into<String>, message: impl Into<String>) {
        self.checks.push(PassportCheck {
            severity: CheckSeverity::Warning,
            path: path.into(),
            message: message.into(),
        });
    }
}

pub fn validate_file(path: &Path, options: &CheckOptions) -> ValidationReport {
    let mut report = ValidationReport::new(path.display().to_string());
    let source = match fs::read_to_string(path) {
        Ok(source) => source,
        Err(error) => {
            report.error("$", format!("failed to read passport file: {error}"));
            return report;
        }
    };

    validate_source(&source, options, report)
}

pub fn validate_str(source: &str, options: &CheckOptions) -> ValidationReport {
    validate_source(source, options, ValidationReport::new("<memory>"))
}

fn validate_source(
    source: &str,
    options: &CheckOptions,
    mut report: ValidationReport,
) -> ValidationReport {
    let document = match yaml_serde::from_str::<Document>(source) {
        Ok(document) => document,
        Err(error) => {
            report.error(
                "$",
                format!("invalid YAML or schema type mismatch: {error}"),
            );
            return report;
        }
    };

    let Some(passport) = document.passport.as_ref() else {
        report.error(
            "$.passport",
            "required top-level passport object is missing",
        );
        return report;
    };

    validate_passport(passport, options, &mut report);
    report
}

fn validate_passport(passport: &Passport, options: &CheckOptions, report: &mut ValidationReport) {
    validate_api_version(passport.api_version.as_deref(), report);
    validate_kind(passport.kind.as_deref(), report);

    match passport.metadata.as_ref() {
        Some(metadata) => validate_metadata(metadata, report),
        None => report.error("$.passport.metadata", "metadata object is required"),
    }

    match passport.spec.as_ref() {
        Some(spec) => validate_spec(spec, report),
        None => report.error("$.passport.spec", "spec object is required"),
    }

    match passport.agent_integrity.as_ref() {
        Some(agent_integrity) => validate_agent_integrity(agent_integrity, options, report),
        None => {
            if matches!(options.integrity, IntegrityMode::VerifyFiles { .. }) {
                report.error(
                    "$.passport.agentIntegrity",
                    "--check-integrity requires agentIntegrity.codeHashes",
                );
            }
        }
    }

    match passport.signature.as_ref() {
        Some(signature) => validate_signature(signature, report),
        None => report.error("$.passport.signature", "signature object is required"),
    }
}

fn validate_api_version(value: Option<&str>, report: &mut ValidationReport) {
    let Some(api_version) = required_string(value, "$.passport.apiVersion", report) else {
        return;
    };

    let Some((group, version)) = api_version.split_once('/') else {
        report.error(
            "$.passport.apiVersion",
            "apiVersion must use group/version form, e.g. agent-passport.io/v1alpha1",
        );
        return;
    };

    if group != "agent-passport.io" {
        report.warning(
            "$.passport.apiVersion",
            "apiVersion group is not agent-passport.io; relying parties may reject it",
        );
    }

    if version != "v1alpha1" {
        report.warning(
            "$.passport.apiVersion",
            "this CLI validates the v1alpha1 RFC shape; unknown versions are treated leniently",
        );
    }
}

fn validate_kind(value: Option<&str>, report: &mut ValidationReport) {
    let Some(kind) = required_string(value, "$.passport.kind", report) else {
        return;
    };

    if kind != "AgentPassport" {
        report.error("$.passport.kind", "kind must be AgentPassport");
    }
}

fn validate_metadata(metadata: &Metadata, report: &mut ValidationReport) {
    required_string(metadata.name.as_deref(), "$.passport.metadata.name", report);

    if let Some(uid) = required_string(metadata.uid.as_deref(), "$.passport.metadata.uid", report) {
        validate_uid(uid, report);
    }

    if let Some(version) = required_string(
        metadata.version.as_deref(),
        "$.passport.metadata.version",
        report,
    ) {
        validate_version(version, report);
    }

    let issue_date = metadata
        .issue_date
        .as_deref()
        .and_then(|value| validate_timestamp(value, "$.passport.metadata.issueDate", report));
    let expiry_date = metadata
        .expiry_date
        .as_deref()
        .and_then(|value| validate_timestamp(value, "$.passport.metadata.expiryDate", report));

    if metadata.issue_date.is_none() {
        report.error("$.passport.metadata.issueDate", "issueDate is required");
    }

    if let Some(issue_date) = issue_date {
        let now = Utc::now();
        if issue_date > now {
            report.error(
                "$.passport.metadata.issueDate",
                "passport is not valid yet because issueDate is in the future",
            );
        }
    }

    if let (Some(issue_date), Some(expiry_date)) = (issue_date, expiry_date) {
        if expiry_date <= issue_date {
            report.error(
                "$.passport.metadata.expiryDate",
                "expiryDate must be later than issueDate",
            );
        }

        if expiry_date <= Utc::now() {
            report.error(
                "$.passport.metadata.expiryDate",
                "passport is expired according to expiryDate",
            );
        }
    }

    required_string(
        metadata.issuer.as_deref(),
        "$.passport.metadata.issuer",
        report,
    );
}

fn validate_uid(uid: &str, report: &mut ValidationReport) {
    if uid.starts_with("did:") || is_uuid_like(uid) {
        return;
    }

    if uid.starts_with("uuid-") {
        report.warning(
            "$.passport.metadata.uid",
            "uid uses a non-standard uuid-* placeholder form; use a real UUID or DID for issuance",
        );
        return;
    }

    report.warning(
        "$.passport.metadata.uid",
        "uid should be a UUID or DID to satisfy global uniqueness expectations",
    );
}

fn validate_version(version: &str, report: &mut ValidationReport) {
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() != 3 || parts.iter().any(|part| part.parse::<u64>().is_err()) {
        report.warning(
            "$.passport.metadata.version",
            "metadata.version should use semantic version form MAJOR.MINOR.PATCH",
        );
    }
}

fn validate_spec(spec: &Spec, report: &mut ValidationReport) {
    match spec.entity.as_ref() {
        Some(entity) => {
            if let Some(entity_type) = required_string(
                entity.entity_type.as_deref(),
                "$.passport.spec.entity.type",
                report,
            ) {
                validate_entity_type(entity_type, report);
            }
        }
        None => report.error("$.passport.spec.entity", "entity object is required"),
    }

    match spec.capabilities.as_ref() {
        Some(capabilities) => validate_capabilities(capabilities, report),
        None => report.error(
            "$.passport.spec.capabilities",
            "capabilities array is required",
        ),
    }

    if let Some(resources) = spec.resources.as_ref() {
        validate_resources(resources, report);
    }

    if let Some(security_policies) = spec.security_policies.as_ref() {
        validate_security_policies(security_policies, report);
    }
}

fn validate_entity_type(entity_type: &str, report: &mut ValidationReport) {
    const KNOWN_TYPES: &[&str] = &["LLM-agent", "system-agent", "proxy-agent", "human-hybrid"];
    if !KNOWN_TYPES.contains(&entity_type) {
        report.warning(
            "$.passport.spec.entity.type",
            "entity type is not one of the RFC examples; ensure relying parties understand it",
        );
    }
}

fn validate_capabilities(capabilities: &[Capability], report: &mut ValidationReport) {
    if capabilities.is_empty() {
        report.warning(
            "$.passport.spec.capabilities",
            "capabilities array is empty; discovery value may be limited",
        );
    }

    for (index, capability) in capabilities.iter().enumerate() {
        let path = format!("$.passport.spec.capabilities[{index}]");
        required_string(capability.name.as_deref(), format!("{path}.name"), report);

        match capability.signature.as_ref() {
            Some(signature) => validate_capability_signature(signature, &path, report),
            None => report.error(
                format!("{path}.signature"),
                "capability signature object is required",
            ),
        }

        if let Some(access_control) = capability.access_control.as_ref() {
            validate_non_empty_string_array(
                access_control,
                &format!("{path}.accessControl"),
                report,
            );
        }
    }
}

fn validate_capability_signature(
    signature: &CapabilitySignature,
    path: &str,
    report: &mut ValidationReport,
) {
    if let Some(parameters) = signature.parameters.as_ref() {
        for (index, parameter) in parameters.iter().enumerate() {
            let parameter_path = format!("{path}.signature.parameters[{index}]");
            required_string(
                parameter.name.as_deref(),
                format!("{parameter_path}.name"),
                report,
            );
            if let Some(parameter_type) = required_string(
                parameter.parameter_type.as_deref(),
                format!("{parameter_path}.type"),
                report,
            ) {
                validate_data_type(parameter_type, &format!("{parameter_path}.type"), report);
            }
        }
    }

    if let Some(returns) = signature.returns.as_ref() {
        if let Some(return_type) = required_string(
            returns.return_type.as_deref(),
            format!("{path}.signature.returns.type"),
            report,
        ) {
            validate_data_type(
                return_type,
                &format!("{path}.signature.returns.type"),
                report,
            );
        }
    }
}

fn validate_data_type(data_type: &str, path: &str, report: &mut ValidationReport) {
    const KNOWN_TYPES: &[&str] = &[
        "string", "integer", "number", "boolean", "array", "object", "null",
    ];
    if !KNOWN_TYPES.contains(&data_type) {
        report.warning(
            path,
            "type is not one of the RFC scalar/container examples; document custom typing rules",
        );
    }
}

fn validate_resources(resources: &Resources, report: &mut ValidationReport) {
    if let Some(file_system_access) = resources.file_system_access.as_ref() {
        for (index, access) in file_system_access.iter().enumerate() {
            validate_file_system_access(access, index, report);
        }
    }

    if let Some(network) = resources.network.as_ref() {
        validate_network(network, report);
    }

    if let Some(executables) = resources.executables.as_ref() {
        for (index, executable) in executables.iter().enumerate() {
            let path = format!("$.passport.spec.resources.executables[{index}]");
            if required_string(Some(executable), &path, report).is_some()
                && !executable.starts_with('/')
            {
                report.warning(
                    path,
                    "executable should usually be an absolute path for enforceability",
                );
            }
        }
    }
}

fn validate_file_system_access(
    access: &FileSystemAccess,
    index: usize,
    report: &mut ValidationReport,
) {
    let path = format!("$.passport.spec.resources.fileSystemAccess[{index}]");

    if let Some(fs_path) = required_string(access.path.as_deref(), format!("{path}.path"), report) {
        if !fs_path.starts_with('/') {
            report.warning(
                format!("{path}.path"),
                "fileSystemAccess path should usually be absolute for runtime enforcement",
            );
        }
    }

    match access.permissions.as_ref() {
        Some(permissions) => {
            if permissions.is_empty() {
                report.error(
                    format!("{path}.permissions"),
                    "permissions must not be empty",
                );
            }
            for (permission_index, permission) in permissions.iter().enumerate() {
                let permission_path = format!("{path}.permissions[{permission_index}]");
                if required_string(Some(permission), &permission_path, report).is_some()
                    && !matches!(permission.as_str(), "read" | "write" | "execute")
                {
                    report.warning(
                        permission_path,
                        "permission is not one of read, write, execute",
                    );
                }
            }
        }
        None => report.error(
            format!("{path}.permissions"),
            "permissions array is required",
        ),
    }
}

fn validate_network(network: &Network, report: &mut ValidationReport) {
    if let Some(outbound) = network.outbound.as_ref() {
        for (index, connection) in outbound.iter().enumerate() {
            validate_outbound_network(connection, index, report);
        }
    }

    if let Some(inbound) = network.inbound.as_ref() {
        for (index, connection) in inbound.iter().enumerate() {
            validate_inbound_network(connection, index, report);
        }
    }
}

fn validate_outbound_network(
    connection: &OutboundNetwork,
    index: usize,
    report: &mut ValidationReport,
) {
    let path = format!("$.passport.spec.resources.network.outbound[{index}]");
    if let Some(protocol) = required_string(
        connection.protocol.as_deref(),
        format!("{path}.protocol"),
        report,
    ) {
        validate_network_protocol(protocol, &format!("{path}.protocol"), report);
    }
    required_string(
        connection.address.as_deref(),
        format!("{path}.address"),
        report,
    );
    if let Some(port) = connection.port {
        validate_port(port, &format!("{path}.port"), report);
    }
}

fn validate_inbound_network(
    connection: &InboundNetwork,
    index: usize,
    report: &mut ValidationReport,
) {
    let path = format!("$.passport.spec.resources.network.inbound[{index}]");
    if let Some(protocol) = required_string(
        connection.protocol.as_deref(),
        format!("{path}.protocol"),
        report,
    ) {
        validate_network_protocol(protocol, &format!("{path}.protocol"), report);
    }
    if connection.port.is_none() {
        report.error(format!("{path}.port"), "inbound port is required");
    } else if let Some(port) = connection.port {
        validate_port(port, &format!("{path}.port"), report);
    }
}

fn validate_network_protocol(protocol: &str, path: &str, report: &mut ValidationReport) {
    if !matches!(protocol, "tcp" | "udp" | "http" | "https") {
        report.warning(path, "protocol is not one of tcp, udp, http, https");
    }
}

fn validate_security_policies(policies: &SecurityPolicies, report: &mut ValidationReport) {
    if let Some(seccomp) = policies.seccomp.as_deref() {
        required_string(
            Some(seccomp),
            "$.passport.spec.securityPolicies.seccomp",
            report,
        );
    }

    if let Some(chroot) = policies.chroot.as_deref() {
        required_string(
            Some(chroot),
            "$.passport.spec.securityPolicies.chroot",
            report,
        );
    }

    if let Some(capabilities) = policies.capabilities.as_ref() {
        for (index, capability) in capabilities.iter().enumerate() {
            let path = format!("$.passport.spec.securityPolicies.capabilities[{index}]");
            if required_string(Some(capability), &path, report).is_some()
                && !capability.starts_with("CAP_")
            {
                report.warning(path, "Linux capability names should use CAP_* form");
            }
        }
    }

    if let Some(restrictions) = policies.network_restrictions.as_ref() {
        validate_network_restrictions(restrictions, report);
    }

    if let Some(environment) = policies.execution_environment.as_ref() {
        if environment.privileged.unwrap_or(false) {
            report.warning(
                "$.passport.spec.securityPolicies.executionEnvironment.privileged",
                "privileged agents violate least-privilege expectations and require review",
            );
        }
    }

    if let Some(policy_engine) = policies.policy_engine.as_ref() {
        required_string(
            policy_engine.name.as_deref(),
            "$.passport.spec.securityPolicies.policyEngine.name",
            report,
        );
    }
}

fn validate_port(port: u16, path: &str, report: &mut ValidationReport) {
    if port == 0 {
        report.error(path, "port must be between 1 and 65535");
    }
}

fn validate_network_restrictions(
    restrictions: &NetworkRestrictions,
    report: &mut ValidationReport,
) {
    if let Some(allow_list) = restrictions.allow_list.as_ref() {
        validate_non_empty_string_array(
            allow_list,
            "$.passport.spec.securityPolicies.networkRestrictions.allowList",
            report,
        );
    }

    if let Some(deny_list) = restrictions.deny_list.as_ref() {
        validate_non_empty_string_array(
            deny_list,
            "$.passport.spec.securityPolicies.networkRestrictions.denyList",
            report,
        );
    }
}

fn validate_agent_integrity(
    integrity: &AgentIntegrity,
    options: &CheckOptions,
    report: &mut ValidationReport,
) {
    match integrity.code_hashes.as_ref() {
        Some(code_hashes) => {
            if code_hashes.is_empty() {
                report.error(
                    "$.passport.agentIntegrity.codeHashes",
                    "codeHashes must not be empty when agentIntegrity is present",
                );
                return;
            }

            for (index, code_hash) in code_hashes.iter().enumerate() {
                validate_code_hash(code_hash, index, options, report);
            }
        }
        None => report.error(
            "$.passport.agentIntegrity.codeHashes",
            "codeHashes array is required when agentIntegrity is present",
        ),
    }
}

fn validate_code_hash(
    code_hash: &CodeHash,
    index: usize,
    options: &CheckOptions,
    report: &mut ValidationReport,
) {
    let path = format!("$.passport.agentIntegrity.codeHashes[{index}]");
    let algorithm = required_string(
        code_hash.algorithm.as_deref(),
        format!("{path}.algorithm"),
        report,
    )
    .and_then(|value| normalize_hash_algorithm(value, &format!("{path}.algorithm"), report));

    let expected = required_string(code_hash.value.as_deref(), format!("{path}.value"), report)
        .and_then(|value| parse_hash_value(value, &format!("{path}.value"), report));

    if let (Some(algorithm), Some(expected)) = (algorithm, expected.as_ref()) {
        let expected_len = algorithm.digest_len();
        if expected.len() != expected_len {
            report.error(
                format!("{path}.value"),
                format!(
                    "{} digest must be {expected_len} bytes, got {} bytes",
                    algorithm.name(),
                    expected.len()
                ),
            );
        }
    }

    if let IntegrityMode::VerifyFiles { root } = &options.integrity {
        verify_code_hash_file(
            code_hash,
            index,
            root,
            algorithm,
            expected.as_deref(),
            report,
        );
    }
}

fn verify_code_hash_file(
    code_hash: &CodeHash,
    index: usize,
    root: &Path,
    algorithm: Option<HashAlgorithm>,
    expected: Option<&[u8]>,
    report: &mut ValidationReport,
) {
    let path = format!("$.passport.agentIntegrity.codeHashes[{index}]");
    let Some(source_file) = required_string(
        code_hash.source_file.as_deref(),
        format!("{path}.sourceFile"),
        report,
    ) else {
        return;
    };
    let Some(algorithm) = algorithm else {
        return;
    };
    let Some(expected) = expected else {
        return;
    };

    let source_path = Path::new(source_file);
    let resolved_path = if source_path.is_absolute() {
        source_path.to_path_buf()
    } else {
        root.join(source_path)
    };

    let contents = match fs::read(&resolved_path) {
        Ok(contents) => contents,
        Err(error) => {
            report.error(
                format!("{path}.sourceFile"),
                format!("failed to read integrity source file {resolved_path:?}: {error}"),
            );
            return;
        }
    };

    let actual = algorithm.digest(&contents);
    if actual.as_slice() != expected {
        report.error(
            format!("{path}.value"),
            format!(
                "integrity hash mismatch for {resolved_path:?}; expected {}, got {}",
                to_hex(expected),
                to_hex(actual.as_slice())
            ),
        );
    }
}

fn validate_signature(signature: &Signature, report: &mut ValidationReport) {
    if let Some(algorithm) = required_string(
        signature.algorithm.as_deref(),
        "$.passport.signature.algorithm",
        report,
    ) {
        validate_signature_algorithm(algorithm, report);
    }

    if let Some(value) = required_string(
        signature.value.as_deref(),
        "$.passport.signature.value",
        report,
    ) {
        if decode_base64(value).is_none() {
            report.error(
                "$.passport.signature.value",
                "signature value must be base64 encoded; cryptographic verification requires a trust store",
            );
        }
    }

    required_string(
        signature.public_key_ref.as_deref(),
        "$.passport.signature.publicKeyRef",
        report,
    );
}

fn validate_signature_algorithm(algorithm: &str, report: &mut ValidationReport) {
    const KNOWN_ALGORITHMS: &[&str] = &["RSASSA-PSS-SHA256", "EdDSA"];
    if !KNOWN_ALGORITHMS.contains(&algorithm) {
        report.warning(
            "$.passport.signature.algorithm",
            "signature algorithm is not one of the RFC examples; ensure verifier support",
        );
    }
}

fn validate_timestamp(
    value: &str,
    path: &str,
    report: &mut ValidationReport,
) -> Option<DateTime<Utc>> {
    if value.trim().is_empty() {
        report.error(path, "timestamp must not be empty");
        return None;
    }

    match DateTime::parse_from_rfc3339(value) {
        Ok(timestamp) => Some(timestamp.with_timezone(&Utc)),
        Err(error) => {
            report.error(
                path,
                format!("timestamp must be RFC 3339 / ISO 8601: {error}"),
            );
            None
        }
    }
}

fn required_string<'a>(
    value: Option<&'a str>,
    path: impl Into<String>,
    report: &mut ValidationReport,
) -> Option<&'a str> {
    let path = path.into();
    match value {
        Some(value) if !value.trim().is_empty() => Some(value),
        Some(_) => {
            report.error(path, "required string must not be empty");
            None
        }
        None => {
            report.error(path, "required field is missing");
            None
        }
    }
}

fn validate_non_empty_string_array(values: &[String], path: &str, report: &mut ValidationReport) {
    if values.is_empty() {
        report.warning(path, "array is empty");
        return;
    }

    for (index, value) in values.iter().enumerate() {
        required_string(Some(value), format!("{path}[{index}]"), report);
    }
}

fn normalize_hash_algorithm(
    value: &str,
    path: &str,
    report: &mut ValidationReport,
) -> Option<HashAlgorithm> {
    match value.to_ascii_uppercase().replace('_', "-").as_str() {
        "SHA-256" | "SHA256" => Some(HashAlgorithm::Sha256),
        "SHA-512" | "SHA512" => Some(HashAlgorithm::Sha512),
        _ => {
            report.error(path, "hash algorithm must be SHA-256 or SHA-512");
            None
        }
    }
}

fn parse_hash_value(value: &str, path: &str, report: &mut ValidationReport) -> Option<Vec<u8>> {
    if let Some(bytes) = decode_hex(value) {
        return Some(bytes);
    }

    if let Some(bytes) = decode_base64(value) {
        return Some(bytes);
    }

    report.error(path, "hash value must be hex or base64 encoded");
    None
}

fn decode_hex(value: &str) -> Option<Vec<u8>> {
    let value = value.trim();
    if value.is_empty() || !value.len().is_multiple_of(2) {
        return None;
    }

    let mut bytes = Vec::with_capacity(value.len() / 2);
    for chunk in value.as_bytes().chunks_exact(2) {
        let high = hex_digit(chunk[0])?;
        let low = hex_digit(chunk[1])?;
        bytes.push((high << 4) | low);
    }
    Some(bytes)
}

fn hex_digit(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

fn decode_base64(value: &str) -> Option<Vec<u8>> {
    let value = value.trim();
    STANDARD
        .decode(value)
        .or_else(|_| STANDARD_NO_PAD.decode(value))
        .ok()
}

fn is_uuid_like(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.len() != 36 {
        return false;
    }

    for (index, byte) in bytes.iter().enumerate() {
        match index {
            8 | 13 | 18 | 23 => {
                if *byte != b'-' {
                    return false;
                }
            }
            _ => {
                if !byte.is_ascii_hexdigit() {
                    return false;
                }
            }
        }
    }
    true
}

#[derive(Debug, Clone, Copy)]
enum HashAlgorithm {
    Sha256,
    Sha512,
}

impl HashAlgorithm {
    fn name(self) -> &'static str {
        match self {
            Self::Sha256 => "SHA-256",
            Self::Sha512 => "SHA-512",
        }
    }

    fn digest_len(self) -> usize {
        match self {
            Self::Sha256 => 32,
            Self::Sha512 => 64,
        }
    }

    fn digest(self, contents: &[u8]) -> Vec<u8> {
        match self {
            Self::Sha256 => Sha256::digest(contents).to_vec(),
            Self::Sha512 => Sha512::digest(contents).to_vec(),
        }
    }
}

fn to_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_passport() {
        let report = validate_str(valid_passport(), &CheckOptions::default());

        assert!(
            report.valid,
            "expected valid passport, got: {:?}",
            report.checks
        );
    }

    #[test]
    fn rejects_missing_signature() {
        let source = valid_passport().replace(
            r#"  signature:
    algorithm: "EdDSA"
    value: "YWJjZA=="
    publicKeyRef: "did:example:issuer:key1"
"#,
            "",
        );

        let report = validate_str(&source, &CheckOptions::default());

        assert!(!report.valid);
        assert!(report
            .checks
            .iter()
            .any(|check| check.path == "$.passport.signature"));
    }

    #[test]
    fn rejects_expired_passport() {
        let source = valid_passport().replace(
            r#"expiryDate: "2035-01-01T00:00:00Z""#,
            r#"expiryDate: "2001-01-01T00:00:00Z""#,
        );

        let report = validate_str(&source, &CheckOptions::default());

        assert!(!report.valid);
        assert!(report
            .checks
            .iter()
            .any(|check| check.path == "$.passport.metadata.expiryDate"));
    }

    #[test]
    fn verifies_integrity_hashes() {
        let temp_dir = tempfile::tempdir().expect("temp dir");
        let source_path = temp_dir.path().join("agent.bin");
        fs::write(&source_path, b"hello").expect("write source file");

        let digest = Sha256::digest(b"hello");
        let source = valid_passport().replace(
            "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08",
            &to_hex(&digest),
        );

        let report = validate_str(
            &source,
            &CheckOptions {
                integrity: IntegrityMode::VerifyFiles {
                    root: temp_dir.path().to_path_buf(),
                },
            },
        );

        assert!(
            report.valid,
            "expected valid integrity report, got: {:?}",
            report.checks
        );
    }

    #[test]
    fn rejects_integrity_hash_mismatch() {
        let temp_dir = tempfile::tempdir().expect("temp dir");
        fs::write(temp_dir.path().join("agent.bin"), b"hello").expect("write source file");

        let report = validate_str(
            valid_passport(),
            &CheckOptions {
                integrity: IntegrityMode::VerifyFiles {
                    root: temp_dir.path().to_path_buf(),
                },
            },
        );

        assert!(!report.valid);
        assert!(report
            .checks
            .iter()
            .any(|check| check.message.contains("integrity hash mismatch")));
    }

    fn valid_passport() -> &'static str {
        r#"passport:
  apiVersion: "agent-passport.io/v1alpha1"
  kind: "AgentPassport"
  metadata:
    name: "log-processor-agent"
    uid: "123e4567-e89b-12d3-a456-426614174000"
    version: "1.0.0"
    issueDate: "2025-01-01T00:00:00Z"
    expiryDate: "2035-01-01T00:00:00Z"
    issuer: "InternalCorpCA"
  spec:
    entity:
      type: "system-agent"
      description: "Processes log files."
      owner: "IT Operations"
      contact: "it-ops@example.com"
    capabilities:
      - name: "process_logs"
        signature:
          parameters: []
          returns:
            type: "object"
    resources:
      fileSystemAccess:
        - path: "/var/log/app"
          permissions: ["read"]
      executables:
        - "/usr/bin/grep"
    securityPolicies:
      chroot: "/srv/agent-sandboxes/log-processor"
      capabilities: []
      executionEnvironment:
        isolated: true
        privileged: false
  agentIntegrity:
    codeHashes:
      - algorithm: "SHA-256"
        value: "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08"
        sourceFile: "agent.bin"
  signature:
    algorithm: "EdDSA"
    value: "YWJjZA=="
    publicKeyRef: "did:example:issuer:key1"
"#
    }
}
