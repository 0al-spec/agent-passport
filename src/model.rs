use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Document {
    pub passport: Option<Passport>,
}

#[derive(Debug, Deserialize)]
pub struct Passport {
    #[serde(rename = "apiVersion")]
    pub api_version: Option<String>,
    pub kind: Option<String>,
    pub metadata: Option<Metadata>,
    pub spec: Option<Spec>,
    #[serde(rename = "agentIntegrity")]
    pub agent_integrity: Option<AgentIntegrity>,
    pub signature: Option<Signature>,
}

#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub name: Option<String>,
    pub uid: Option<String>,
    pub version: Option<String>,
    #[serde(rename = "issueDate")]
    pub issue_date: Option<String>,
    #[serde(rename = "expiryDate")]
    pub expiry_date: Option<String>,
    pub issuer: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Spec {
    pub entity: Option<Entity>,
    pub capabilities: Option<Vec<Capability>>,
    pub resources: Option<Resources>,
    #[serde(rename = "securityPolicies")]
    pub security_policies: Option<SecurityPolicies>,
}

#[derive(Debug, Deserialize)]
pub struct Entity {
    #[serde(rename = "type")]
    pub entity_type: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "baseModel")]
    pub base_model: Option<String>,
    pub owner: Option<String>,
    pub contact: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Capability {
    pub name: Option<String>,
    pub description: Option<String>,
    pub signature: Option<CapabilitySignature>,
    #[serde(rename = "accessControl")]
    pub access_control: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct CapabilitySignature {
    pub parameters: Option<Vec<Parameter>>,
    pub returns: Option<ReturnValue>,
}

#[derive(Debug, Deserialize)]
pub struct Parameter {
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub parameter_type: Option<String>,
    pub required: Option<bool>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ReturnValue {
    #[serde(rename = "type")]
    pub return_type: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Resources {
    pub cpu: Option<String>,
    pub memory: Option<String>,
    pub storage: Option<String>,
    #[serde(rename = "fileSystemAccess")]
    pub file_system_access: Option<Vec<FileSystemAccess>>,
    pub network: Option<Network>,
    pub executables: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct FileSystemAccess {
    pub path: Option<String>,
    pub permissions: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Network {
    pub outbound: Option<Vec<OutboundNetwork>>,
    pub inbound: Option<Vec<InboundNetwork>>,
}

#[derive(Debug, Deserialize)]
pub struct OutboundNetwork {
    pub protocol: Option<String>,
    pub address: Option<String>,
    pub port: Option<u16>,
}

#[derive(Debug, Deserialize)]
pub struct InboundNetwork {
    pub protocol: Option<String>,
    pub port: Option<u16>,
}

#[derive(Debug, Deserialize)]
pub struct SecurityPolicies {
    pub seccomp: Option<String>,
    pub chroot: Option<String>,
    pub capabilities: Option<Vec<String>>,
    #[serde(rename = "networkRestrictions")]
    pub network_restrictions: Option<NetworkRestrictions>,
    #[serde(rename = "executionEnvironment")]
    pub execution_environment: Option<ExecutionEnvironment>,
    #[serde(rename = "policyEngine")]
    pub policy_engine: Option<PolicyEngine>,
}

#[derive(Debug, Deserialize)]
pub struct NetworkRestrictions {
    #[serde(rename = "allowList")]
    pub allow_list: Option<Vec<String>>,
    #[serde(rename = "denyList")]
    pub deny_list: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct ExecutionEnvironment {
    pub privileged: Option<bool>,
    pub isolated: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct PolicyEngine {
    pub name: Option<String>,
    pub config: Option<yaml_serde::Value>,
}

#[derive(Debug, Deserialize)]
pub struct AgentIntegrity {
    #[serde(rename = "codeHashes")]
    pub code_hashes: Option<Vec<CodeHash>>,
}

#[derive(Debug, Deserialize)]
pub struct CodeHash {
    pub algorithm: Option<String>,
    pub value: Option<String>,
    #[serde(rename = "sourceFile")]
    pub source_file: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Signature {
    pub algorithm: Option<String>,
    pub value: Option<String>,
    #[serde(rename = "publicKeyRef")]
    pub public_key_ref: Option<String>,
    #[serde(rename = "signedBy")]
    pub signed_by: Option<String>,
}
