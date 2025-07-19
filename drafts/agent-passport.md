# Agent Passport Specification

Request for Comments

## Authors' Contact Information

* Egor Merkushev
* Organization: Individual
* Email: gorkaedeep@gmail.com
* Published: 20 July 2025

## Status of this Memo

This document is an **Experimental** Request for Comments. It represents a draft proposal for an "Agent Passport" specification and is submitted to the community for discussion, feedback, critique, and suggestions for improvement. This document is a "work in progress" and is not yet a finalized standard. Distribution of this memo is unlimited.

Comments should be submitted as GitHub issues in the original repository hosting this RFC.

## Copyright Notice and Licensing

Copyright © 2025 Egor Merkushev. All rights reserved.

This document is released under the [Creative Commons Attribution 4.0 International License (CC BY 4.0)](https://creativecommons.org/licenses/by/4.0/). You are free to copy, distribute, and modify this specification, even for commercial purposes, provided that attribution is given to the original author(s).

To contribute to this document, please submit an issue or pull request to the original [GitHub repository](https://github.com/0al-spec/agent-passport). See [CONTRIBUTING.md](https://github.com/0al-spec/agent-passport/blob/main/CONTRIBUTING.md) for details.

*Note: This is not an IETF document and is not subject to BCP 78 or the IETF Trust. However, it follows similar principles of openness and community participation.*

## Abstract

This document outlines the specification for an **Agent Passport**, a standardized framework designed to describe the capabilities, resource needs, and security measures of AI agents. By providing a uniform way to define and manage AI agents, the Agent Passport aims to enhance interoperability and trust within multi-agent systems. Similar to a human passport, it verifies an agent's identity, capabilities, and constraints, while safeguarding against unauthorized alterations. This specification addresses the current gap in secure and standardized AI agent management, promoting a more cohesive and secure AI ecosystem.

## Table of Contents

1.  Introduction
    1. Problem Statement
    2. Purpose and Scope
    3. Relationship to 0AL
    4. Analogy to Human Passports
2.  Motivation
    1. Need for Standardization in AI Agent Description
    2. Addressing Security and Trust Concerns
    3. Limitations of Existing Approaches (eg. A2A Agent Card)
    4. Enabling Interoperability and an Agent Ecosystem
3.  Terminology and Definitions
    1. Agent
    2. Agent Passport
    3. Agent Card
    4. Issuing Authority
    5. Verification
    6. Capability
    7. Security Policy
    8. Agent Integrity
    9. Digital Signature
    10. Other relevant terms
4.  Requirements
    1. Functional Requirements
    2. Non-Functional Requirements
5.  Agent Passport Specification
    1. Core Structure (YAML Schema)
      1. Identification
      2. Agent Entity Description
      3. Capabilities
      4. Resource Requirements
      5. Security Policies
      6. Agent Integrity Verification
      7. Digital Signature and Verification
      8. Versioning and Extensibility
      9. Full YAML Examples
6.  Issuance and Verification Process
    1. Issuing Authorities
    2. Passport Issuance Process
    3. Passport Verification Process
    4. Lifecycle Management
7.  Security Considerations
    1. Threats to AI Agent Systems (General)
    2. Risks to Passport Integrity and Authenticity
    3. Risks during Issuance and Verification
    4. Mitigating Risks through Digital Signatures and Verification
    5. Role of Policies in Mitigating Agent-Specific Risks
    6. Encryption Considerations
    7. Audit and Compliance
8.  Extensibility
    1. Mechanisms for Extending the Passport Schema
    2. Versioning Strategy
9.  Use Cases
    1. Agent Discovery and Trust Establishment
    2. Policy Enforcement
    3. Multi-Agent System Deployment
    4. Auditing and Compliance
10. IANA Considerations
11. Acknowledgements
12. References
    1. Normative References
    2. Informative References
13. Version History
14. Appendix A. Comparison with A2A Agent Card
15. Appendix B. Open Issues and Future Considerations

## 1. Introduction

The rapid proliferation of AI agents in various domains necessitates a robust and standardized approach to their definition, management, and secure operation. As AI agents become more sophisticated and interconnected, operating within complex multi-agent systems, the need for clear identification, capability declaration, and, paramountly, enforced security policies becomes critical. This RFC proposes the "Agent Passport" specification to fulfill these requirements.

### 1.1. Problem Statement

The current landscape of AI agent development often lacks a standardized and secure method for describing agents. This leads to a fragmented ecosystem where interoperability is hindered, and ensuring trust and security is challenging. Existing approaches, such as the Google A2A Agent Card, may not provide the comprehensive security and verification mechanisms required for complex and sensitive agent operations. Without a universally recognized and verifiable declaration of an agent's identity, capabilities, and security posture, the risks of unauthorized access, malicious behavior, and system compromise are significantly elevated.

### 1.2. Purpose and Scope

The purpose of this RFC is to define a comprehensive, human-readable, and machine-parsable specification for the Agent Passport, primarily using YAML. The scope includes:

  * Defining the core structure of an Agent Passport, including identification, agent entity description, capabilities, resource requirements, and security policies.
  * Outlining the mechanisms for issuance, digital signing, and verification of Agent Passports by "Issuing Authorities".
  * Establishing requirements for security, versioning, and extensibility of the Agent Passport.
  * Providing a framework for how Agent Passports enable interoperability, policy enforcement, and trust establishment within a multi-agent ecosystem.

This specification focuses on the conceptual framework and declarative aspects of the Agent Passport, laying the groundwork for its broad adoption across various AI agent runtimes and frameworks.

### 1.3. Relationship to 0AL

The Agent Passport is a central and integral concept within the Zero-trust Agentification Layer (0AL) project. While 0AL was initially positioned as an execution environment (`zeroald`), the Agent Passport specification and its associated tooling are intended to become the foundational open agent specification — with a strong emphasis on identity, integrity, and security. It defines the core trust framework that enables secure interaction, authentication, and delegation between agents in decentralized and hybrid environments. The `zeroald` runtime will serve as a reference implementation of this standard, but the goal is to promote widespread adoption of the Agent Passport specification by other AI agent runtimes and frameworks (e.g., LangChain, AIOS, AutoGen) and cloud providers. 0AL aims to become the "standard keeper" and a key contributor to the development of this standard, promoting it through communities and organizations.

### 1.4. Analogy to Human Passports

The Agent Passport draws a strong analogy to a human passport, serving as a verifiable identification document for an AI agent. Just as a human passport confirms identity, nationality, and legal rights, an Agent Passport verifies an agent's "identity," its declared capabilities (methods, resources), and, most importantly, its limitations through explicit security policies (e.g., seccomp, chroot, cap-lists). This analogy highlights the need for the Agent Passport to be protected against forgery and unauthorized modification, and to be issued and verified by trusted "Issuing Authorities". This concept elevates the discussion of agent security and trust, particularly for enterprise users and scenarios requiring high security and reliability.

## 2. Motivation

The motivation behind the Agent Passport specification stems from critical needs within the evolving AI agent ecosystem.

### 2.1. Need for Standardization in AI Agent Description

The current lack of a standardized way to describe AI agents leads to significant fragmentation and inefficiency in multi-agent systems. Without a common language for defining agent capabilities and requirements, interoperability remains a significant hurdle. The Agent Passport addresses this by providing a universal, declarative YAML-based format that can be understood and processed across different platforms and frameworks. This standardization will streamline agent development, deployment, and integration, fostering a more cohesive and efficient AI agent ecosystem.

### 2.2. Addressing Security and Trust Concerns

As AI agents gain more autonomy and access to sensitive resources, security and trust become paramount. Traditional security models are often insufficient for dynamic agent environments. The Agent Passport directly addresses these concerns by:

  * **Declarative Security Policies:** Explicitly defining security policies such as seccomp, chroot, cap-lists, network restrictions, and access control policies within the passport itself. This allows for fine-grained control over an agent's permissible actions and resource access.
  * **Digital Signatures and Verification:** Requiring cryptographic signatures by Issuing Authorities to ensure the integrity and authenticity of the passport, preventing tampering and spoofing. This provides a chain of trust, verifying not only what an agent claims to be but also who attested to its identity and capabilities.
  * **Lifecycle Management:** Incorporating a lifecycle for passports, including issuance, expiration, renewal, and revocation, similar to human passports, to manage agent trust over time.

### 2.3. Limitations of Existing Approaches (e.g., A2A Agent Card)

While existing mechanisms like the A2A Agent Card provide some level of agent description, they often fall short in providing the comprehensive security and verifiable identity features necessary for robust multi-agent systems. The Agent Passport distinguishes itself by emphasizing:

  * **Verifiable Identity:** Going beyond a simple description to offer an official, cryptographically verifiable "document" that confirms an agent's identity and attested characteristics.
  * **Enforceable Security Policies:** Directly embedding granular security policies within the passport, allowing for runtime enforcement by environments like `agentifyd`.
  * **Issuance and Verification Framework:** Establishing a clear model for Issuing Authorities and verification processes, which is often absent or less formalized in simpler "agent card" concepts.

### 2.4. Enabling Interoperability and an Agent Ecosystem

The Agent Passport specification is designed to enable seamless interoperability across diverse AI agent frameworks and environments. By providing a standardized, machine-parsable format for agent description, it facilitates:

  * **Agent Discovery and Communication:** Agents can easily discover each other's capabilities and understand their operational constraints, fostering more effective communication and collaboration.
  * **Cross-Platform Compatibility:** The specification aims to be adopted by various runtimes and frameworks, allowing agents to be defined once and deployed across different compatible systems.
  * **Development of a Unified Ecosystem:** By providing a common foundation for agent description and security, the Agent Passport can act as a catalyst for the growth of a healthy, secure, and truly interoperable AI agent ecosystem.

## 3. Terminology and Definitions

This section defines key terms used throughout this Request for Comments (RFC).

### 3.1. Agent

An autonomous software entity, often powered by Large Language Models (LLMs), designed to perform specific tasks or interact with systems and other agents. Agents operate within a multi-agent ecosystem and require clear identification, capability declaration, and enforced security policies for secure and interoperable operation.

### 3.2. Agent Passport

A declarative, YAML-based specification for describing the capabilities, resource requirements, and, critically, the security policies of AI agents. It serves as an official, verifiable "document" that confirms an agent's identity, declared functionalities, and limitations, protected against forgery and unauthorized changes. The Agent Passport is a central concept within the Agentification Layer (AL) project.

### 3.3. Agent Card

A more limited concept compared to the Agent Passport, often used as a public "business card" for an agent, describing its general capabilities. The Agent Passport expands upon this by including enhanced security, access control, and cryptographic verification. An example is the "Agent Card" in the A2A Protocol.

### 3.4. Issuing Authority

A trusted entity or organization responsible for issuing and digitally signing Agent Passports. This can include the company deploying the agents, the Agentification Layer (AL) via AgentHub, or third-party organizations specializing in AI agent auditing and certification. Issuing Authorities ensure the integrity and authenticity of the passport.

### 3.5. Verification

The process of confirming the authenticity, integrity, and validity of an Agent Passport. This typically involves checking the digital signature of the passport against the Issuing Authority's public key and validating that the declared capabilities and security policies are consistent and enforceable.

### 3.6. Capability

A declarative description of the functionalities or methods an agent provides. This includes details such as method signatures, data types, and expected parameters, allowing other agents or systems to understand and interact with the agent.

### 3.7. Security Policy

Explicit rules and constraints embedded within an Agent Passport that define an agent's permissible actions, resource access, and operational limitations. Examples include seccomp (secure computing mode), chroot (change root directory), cap-lists (capabilities lists), network restrictions, and access control policies.

### 3.8. Agent Integrity

Agent Integrity is the property that ensures the executable code and associated components of an AI agent have not been altered or tampered with since their approval by the Issuing Authority. It is verified by comparing cryptographic hashes of the agent's key files against reference hashes specified in the agentIntegrity section of its passport.

Distinct from the integrity of the passport itself (which is protected by a digital signature), Agent Integrity guarantees that the code being launched in a runtime environment is the exact code that was vetted and for which the passport was issued. This serves as a form of "biometric" verification for the software, linking the agent's verified identity (the passport) to its physical embodiment (the code).

### 3.9. Digital Signature

A cryptographic mechanism used to ensure the integrity and authenticity of an Agent Passport. The passport is signed by an Issuing Authority, and this signature can be verified by relying parties to confirm that the passport has not been tampered with and originates from a trusted source.

### 3.10. Other relevant terms

  * **Zero-trust:** A security model that assumes that all entities are potentially malicious and that no trust should be implicitly granted to any entity. It requires continuous verification and validation of entities and their actions.
  * **Zero-trust Agentification Layer (0AL):** Originally designed as an execution environment (`zeroald`), the project has evolved to center around the Agent Passport specification and its associated tooling as its core focus. 0AL aspires to be the canonical authority for the Agent Passport standard, fostering its adoption across diverse AI agent platforms and frameworks. Zero-trust is a fundamental principle of the 0AL, ensuring that all interactions are authenticated and authorized.
  * **Agent Hub:** A centralized service within the Agentification Layer that functions as a public registry for discovering and distributing AI agents and their passports. It can also act as a trusted Issuing Authority, responsible for vetting agents and signing their passports to establish a chain of trust.
  * **zeroald:** Reference runtime daemon for executing native 0AL agents with Agent Passport standard.
  * **agentifyd:** Agentification daemon for legacy binaries using Agent Passport.
  * **YAML:** A human-readable data serialization language used as the primary format for defining Agent Passports.
  * **Chain of Trust:** A hierarchical or networked relationship wherein the validity and trustworthiness of a document (like an Agent Passport) is established by verifying the signature of its issuer, which in turn may be verified by another trusted issuer, ultimately leading back to a root of trust (e.g., a self-signed root certificate or a pre-configured trusted authority).
  * **Relying Party**: Any entity that depends on the validity of an Agent Passport and the information within it to make a security or access control decision. In this ecosystem, a relying party could be another agent, an agent runtime (like `agentifyd`), or any system interacting with the agent.

## 4. Requirements

This section outlines the key requirements for the Agent Passport specification, encompassing both its functional and non-functional aspects.

### 4.1. Functional Requirements

The Agent Passport specification SHALL fulfill the following functional requirements:

  * **Human-Readable:** The Agent Passport MUST be easily understandable by humans, facilitating review, auditing, and manual inspection.
  * **Machine-Parsable:** The Agent Passport MUST be structured in a way that allows for efficient and unambiguous parsing by machines, enabling automated processing and validation. This will primarily be achieved through a YAML-based schema.
  * **Declarative Description of Agent:** It MUST provide a declarative mechanism to describe an agent's identity, type (e.g., LLM-agent, system agent, proxy agent), purpose, and capabilities/methods.
  * **Resource Requirements Declaration:** The passport MUST allow for the declaration of an agent's resource requirements, including CPU, RAM, file system access, network ports, and specific utilities/binaries.
  * **Security Policies Declaration:** It MUST enable the explicit declaration of fine-grained security policies, such as seccomp, chroot, cap-lists, network restrictions, and access control policies for methods and resources.
  * **Unique Identification:** The passport MUST contain a unique identifier for the agent (e.g., UUID, DID, reversed domain name), along with its name and version.
  * **Support for Lifecycle Management:** The specification SHOULD accommodate concepts of issuance date, validity period, renewal, and revocation.

### 4.2. Non-functional Requirements

The Agent Passport specification SHALL adhere to the following non-functional requirements:

  * **Security:**
      * **Integrity and Authenticity:** The Agent Passport MUST be protected against forgery and unauthorized modification through cryptographic digital signatures by trusted Issuing Authorities.
      * **Verifiability:** There MUST be a clear process for verifying the digital signature and the declared attributes of the passport against the Issuing Authority's credentials.
      * **Trust Chain:** The specification SHOULD support building a chain of trust for passports, potentially through integration with Public Key Infrastructure (PKI), Decentralized Identifiers (DIDs), or Verifiable Credentials (VCs).
      * **Mitigation of Security Risks:** The passport SHOULD explicitly contribute to mitigating common security risks in multi-agent systems, such as unauthorized access and malicious behavior.
  * **Versioning:** The Agent Passport MUST support clear versioning to manage evolution of the specification and agent definitions over time without breaking compatibility, possibly indicating a v0.1 or v0 vs v1 structure.
  * **Extensibility:** The specification MUST provide mechanisms for future extensions to the passport schema without requiring a complete re-definition or breaking backward compatibility.
  * **Interoperability:** The specification SHOULD enable seamless interoperability between agents defined using the passport across different AI agent runtimes and frameworks (e.g., LangChain, AIOS, AutoGen).
  * **LLM-Agnostic:** The design SHOULD NOT be tied to a specific Large Language Model, allowing for broad applicability.
  * **Low Entry Threshold:** The specification SHOULD provide a low entry threshold for developers. The use of YAML helps achieve this.

## 5. Agent Passport Specification

This section defines the core structure and schema of the Agent Passport, primarily specified in YAML, outlining the essential fields and their purpose. The Agent Passport is designed to be a comprehensive, declarative, and verifiable document for AI agents.

### 5.1. Core Structure (YAML Schema)

The Agent Passport is structured as a YAML document, facilitating both human readability and machine parsability. The top-level structure of the Agent Passport includes critical metadata and distinct sections for agent identification, entity description, capabilities, resource requirements, security policies, and digital signature information.

A high-level representation of the Agent Passport YAML schema:

```yaml
passport:
  apiVersion: "agent-passport.io/v1alpha1" # API version of the passport specification
  kind: "AgentPassport"                   # Type of the document
  metadata:
    name: "my-llm-agent"
    uid: "a1b2c3d4-e5f6-7890-1234-567890abcdef"
    version: "1.0.0"                      # Version of this specific agent passport
    issueDate: "2025-05-27T10:00:00Z"     # ISO 8601 timestamp
    expiryDate: "2026-05-27T10:00:00Z"    # Optional: ISO 8601 timestamp for validity
    issuer: "AgentHub-Development-CA"     # Name of the Issuing Authority
  spec:
    entity:
      # Agent entity description details (see 5.1.2)
    capabilities:
      # Agent's declared functionalities (see 5.1.3)
    resources:
      # Agent's resource requirements (see 5.1.4)
    securityPolicies:
      # Agent's security policies (see 5.1.5)
  agentIntegrity:
    # Agent's integrity policies (see 5.1.6)
  signature:
    # Digital signature and verification details (see 5.1.7)
```

#### 5.1.1. Identification

This section provides unique identifiers and metadata for the Agent Passport itself and the agent it describes.

  * `apiVersion` (string, required): Specifies the version of the Agent Passport API. This follows a group/version format (e.g., `agent-passport.io/v1alpha1`).
  * `kind` (string, required): Denotes the type of the YAML document, which must be `AgentPassport`.
  * `metadata` (object, required): Contains metadata about the specific instance of the Agent Passport.
      * `name` (string, required): A human-readable name for the agent.
      * `uid` (string, required): A universally unique identifier (UUID) or Decentralized Identifier (DID) for the agent, ensuring global uniqueness.
      * `version` (string, required): The version of this particular agent's passport definition. This is distinct from `apiVersion` and reflects updates to the agent's definition.
      * `issueDate` (string, required): The timestamp (ISO 8601 format) when the Agent Passport was issued.
      * `expiryDate` (string, optional): The timestamp (ISO 8601 format) indicating when the Agent Passport ceases to be valid, similar to human passports. This supports lifecycle management.
      * `issuer` (string, required): The name or identifier of the Issuing Authority that digitally signed and issued this passport.

#### 5.1.2. Agent Entity Description

This section provides details about the agent itself, beyond just its identification.

  * `entity` (object, required):
      * `type` (string, required): The classification of the agent (e.g., `LLM-agent`, `system-agent`, `proxy-agent`, `human-hybrid`).
      * `description` (string, optional): A brief textual description of the agent's purpose and functionality.
      * `baseModel` (string, optional): For LLM-based agents, the underlying language model used (e.g., `GPT-4`, `Gemini Pro`, `Llama 3`).
      * `owner` (string, optional): Identifier of the individual or organization responsible for the agent.
      * `contact` (string, optional): Contact information for the agent's owner or support.

#### 5.1.3. Capabilities

This section declaratively lists the functions, methods, or services an agent provides, enabling other agents or systems to understand and interact with it.

  * `capabilities` (array of objects, required): A list of capabilities the agent exposes.
      * Each capability object:
          * `name` (string, required): The unique name of the capability or method (e.g., `send_email`, `query_database`).
          * `description` (string, optional): A description of what the capability does.
          * `signature` (object, required): Defines the input parameters and expected output.
              * `parameters` (array of objects, optional): List of input parameters. Each parameter object:
                  * `name` (string, required): Parameter name.
                  * `type` (string, required): Data type (e.g., `string`, `integer`, `boolean`, `array`, `object`).
                  * `required` (boolean, optional): Whether the parameter is mandatory.
                  * `description` (string, optional): Description of the parameter.
              * `returns` (object, optional): Defines the return value.
                  * `type` (string, required): Data type of the return value.
                  * `description` (string, optional): Description of the return value.
          * `accessControl` (array of strings, optional): Specifies which entities (e.g., other agents, roles, users) are authorized to invoke this capability (e.g., `["internal-agents", "admin-users"]`).

#### 5.1.4. Resource Requirements

This section specifies the system resources and external access that the agent requires to operate, facilitating proper environment provisioning and policy enforcement.

  * `resources` (object, optional):
      * `cpu` (string, optional): CPU requirements (e.g., `"2 cores"`, `"500m"`).
      * `memory` (string, optional): Memory requirements (e.g., `"1GB"`, `"512Mi"`).
      * `storage` (string, optional): Storage requirements (e.g., `"10GB"`).
      * `fileSystemAccess` (array of objects, optional): Specifies required file system paths and permissions. Each object:
          * `path` (string, required): The file system path.
          * `permissions` (array of strings, required): List of allowed operations (e.g., `["read", "write", "execute"]`).
      * `network` (object, optional): Network access requirements.
          * `outbound` (array of objects, optional): List of allowed outbound connections. Each object:
              * `protocol` (string, required): `tcp`, `udp`, `http`, `https`.
              * `address` (string, required): IP address or hostname (e.g., `"api.example.com"`).
              * `port` (integer, optional): Specific port number.
          * `inbound` (array of objects, optional): List of allowed inbound connections (e.g., if the agent provides a local service). Each object:
              * `protocol` (string, required): `tcp`, `udp`, `http`, `https`.
              * `port` (integer, required): Specific port number.
      * `executables` (array of strings, optional): List of external binaries or utilities the agent is allowed to execute (e.g., `["/usr/bin/python3", "/usr/bin/curl"]`).

#### 5.1.5. Security Policies

This crucial section defines the granular security policies and constraints under which the agent must operate. These policies are intended to be enforced by the underlying runtime environment (e.g., `agentifyd`).

  * `securityPolicies` (object, optional):
      * `seccomp` (string, optional): Path to a seccomp profile (e.g., `"default-seccomp-profile.json"`) or inline YAML/JSON for syscall filtering.
      * `chroot` (string, optional): Specifies a chroot jail path for the agent's execution environment, limiting its file system view.
      * `capabilities` (array of strings, optional): Linux capabilities (e.g., `CAP_NET_BIND_SERVICE`, `CAP_SYS_ADMIN`) that the agent is allowed to possess.
      * `networkRestrictions` (object, optional): More specific network policy definitions than `resources.network`.
          * `allowList` (array of strings, optional): List of allowed IP ranges or domain names.
          * `denyList` (array of strings, optional): List of denied IP ranges or domain names.
      * `executionEnvironment` (object, optional): Constraints on the execution environment.
          * `privileged` (boolean, optional): Whether the agent requires privileged access (defaults to `false`). Highly discouraged.
          * `isolated` (boolean, optional): Whether the agent should run in a strongly isolated environment (e.g., sandbox, VM) (defaults to `true`).
      * `policyEngine` (object, optional): Configuration for an external policy engine.
          * `name` (string, required): Name of the policy engine (e.g., `Open Policy Agent`, `AL Policy Engine`).
          * `config` (object, optional): Policy engine specific configuration.

#### 5.1.6. Agent Integrity Verification

This optional section is designed to hold information that allows for the verification of an agent's executable code integrity. This is particularly useful for locally deployed agents, serving as a form of "biometric" identifier for the agent's codebase.

  * `agentIntegrity` (object, optional): Contains data for verifying the agent's code integrity.
    * `codeHashes` (array of objects, required if agentIntegrity is present): A list of one or more cryptographic hashes of the agent's executable files or its key components.
      * Each `codeHash` object:
        * `algorithm` (string, required): The hashing algorithm used to compute the hash value (e.g., SHA-256, SHA-512).
        * `value` (string, required): The hash value, typically encoded in HEX or base64.
        * `sourceFile` (string, optional): A relative path or identifier for the file/component for which the hash was computed, to facilitate verification.

The process assumes that during passport issuance, the Issuing Authority or developer computes hashes for specified agent components. These hashes are then included in the passport. Before execution, the agent's runtime environment must re-compute the hashes of the relevant local files and compare them against the values in the passport. A mismatch indicates that the agent's code may have been tampered with or is an incorrect version.

#### 5.1.7. Digital Signature and Verification

This section contains information necessary for cryptographically verifying the integrity and authenticity of the Agent Passport.

  * `signature` (object, required):
      * `algorithm` (string, required): The cryptographic algorithm used for signing (e.g., `RSASSA-PSS-SHA256`, `EdDSA`).
      * `value` (string, required): The base64-encoded digital signature of the entire Agent Passport document (excluding the `signature` field itself).
      * `publicKeyRef` (string, required): A reference to the public key used to verify the signature. This could be a URL to a JWKS endpoint, a DID, or a direct base64-encoded public key.
      * `signedBy` (string, optional): Identifier of the entity that performed the signing (e.g., `AL-AgentHub-Prod-Signer`).

#### 5.1.8. Versioning and Extensibility

The Agent Passport specification supports versioning at two levels:

  * **API Versioning (`apiVersion`):** Indicates significant changes to the overall Agent Passport schema. Backward compatibility will be maintained where possible, but major changes will result in a new `apiVersion` (e.g., `v1alpha1`, `v1beta1`, `v1`).
  * **Passport Versioning (`metadata.version`):** Reflects updates to a specific agent's passport definition, allowing agents to evolve without requiring a new specification version.

Extensibility is supported by allowing additional fields to be added to existing sections or new top-level sections for future features, provided they do not conflict with defined schema elements. Implementations should be designed to gracefully ignore unknown fields.

#### 5.1.9. Full YAML Examples

This subsection provides complete examples of Agent Passport YAML files to illustrate their practical application.

**Example 1: Simple LLM Agent with Network Access**

```yaml
passport:
  apiVersion: "agent-passport.io/v1alpha1"
  kind: "AgentPassport"
  metadata:
    name: "weather-reporter-agent"
    uid: "uuid-weather-agent-12345"
    version: "1.0.1"
    issueDate: "2025-05-28T10:00:00Z"
    expiryDate: "2026-05-28T10:00:00Z"
    issuer: "TrustedAgentIssuers Inc."
  spec:
    entity:
      type: "LLM-agent"
      description: "An agent that reports the weather for a given city using an external API."
      baseModel: "Generic-LLM-Small"
      owner: "Weather Services Ltd."
      contact: "support@weatherservices.example.com"
    capabilities:
      - name: "get_weather"
        description: "Fetches the current weather for a specified city."
        signature:
          parameters:
            - name: "city"
              type: "string"
              required: true
              description: "The name of the city."
          returns:
            type: "object"
            description: "An object containing weather information (temperature, condition)."
        accessControl: ["public-query-access"]
    resources:
      cpu: "200m"
      memory: "256Mi"
      network:
        outbound:
          - protocol: "https"
            address: "api.weatherprovider.example.com"
            port: 443
    securityPolicies:
      seccomp: "default-llm-agent-profile.json" # Assumed profile allowing basic operations + networking
      networkRestrictions:
        allowList:
          - "api.weatherprovider.example.com" # Only allow calls to this specific domain
      executionEnvironment:
        isolated: true
  agentIntegrity:
    codeHashes:
      - algorithm: "SHA-256"
        value: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        sourceFile: "main_agent_binary"
      - algorithm: "SHA-256"
        value: "a665a45920422f9d417e4867efdc4fb8a04a1f3fff1fa07e998e86f7f7a27ae3"
        sourceFile: "utils/helper_library.so"
  signature:
    algorithm: "RSASSA-PSS-SHA256"
    value: "ewo...SflK=" # Placeholder for actual signature
    publicKeyRef: "https://trustedagentissuers.example.com/keys/weather-agent-key.jwk"
    signedBy: "WeatherServicesSigner001"
```

**Example 2: System Agent for File Processing (Restricted)**

```yaml
passport:
  apiVersion: "agent-passport.io/v1alpha1"
  kind: "AgentPassport"
  metadata:
    name: "log-processor-agent"
    uid: "uuid-logproc-agent-67890"
    version: "0.9.0"
    issueDate: "2025-05-20T14:30:00Z"
    issuer: "InternalCorpCA"
  spec:
    entity:
      type: "system-agent"
      description: "Processes log files from a specific directory and archives them."
      owner: "IT Operations - CorpX"
      contact: "it-ops@corpX.example.com"
    capabilities:
      - name: "process_logs"
        description: "Scans for new log files, processes them, and moves to archive."
        signature:
          parameters: [] # No external parameters, triggered internally or by schedule
          returns:
            type: "object"
            description: "Status of the log processing (e.g., files processed, errors)."
    resources:
      cpu: "500m"
      memory: "512Mi"
      fileSystemAccess:
        - path: "/var/logs/app_A/"
          permissions: ["read", "write"] # Read logs, write/delete processed ones
        - path: "/mnt/archive/logs_app_A/"
          permissions: ["write"]          # Write to archive
      executables:
        - "/usr/bin/grep"
        - "/usr/bin/gzip"
    securityPolicies:
      chroot: "/srv/agent-sandboxes/log-processor/" # Confine to a specific root
      capabilities: [] # No special Linux capabilities needed
      networkRestrictions:
        denyList: ["0.0.0.0/0"] # Deny all network access
      executionEnvironment:
        isolated: true
        privileged: false
  agentIntegrity:
      codeHashes:
        - algorithm: "SHA-512"
          value: "3c9c2a...a95572" # Placeholder for the agent binary's actual hash
          sourceFile: "/usr/local/bin/log-processor"
  signature:
    algorithm: "EdDSA"
    value: "aBc...XyZ=" # Placeholder for actual signature
    publicKeyRef: "did:example:issuer:InternalCorpCA:key1"
    signedBy: "InternalCorpCASigner"
```

## 6. Issuance and Verification Process

This section describes the mechanisms by which Agent Passports are issued, digitally signed, and subsequently verified, establishing a chain of trust within the AI agent ecosystem.

### 6.1. Issuing Authorities

An Issuing Authority is a trusted entity responsible for generating, signing, and distributing Agent Passports. These authorities play a critical role in establishing the authenticity and integrity of an agent's declared identity and policies. Potential Issuing Authorities include:

  * **Internal Organizational CAs:** The company or organization deploying the agents can act as its own internal Certificate Authority (CA) to issue passports for its agents, ensuring internal trust.
  * **Agentification Layer (AL) / AgentHub:** The Agentification Layer, particularly through its AgentHub component, can serve as an Issuing Authority. This could involve vetting agents (e.g., through static code analysis, security checks) before signing their passports, thereby providing a level of public trust or certification.
  * **Third-Party Certification Bodies:** Independent third-party organizations specializing in AI agent auditing, security assessment, and certification could emerge as trusted Issuing Authorities, akin to traditional certificate authorities.

Issuing Authorities are responsible for managing their signing keys securely and maintaining a verifiable record of issued and revoked passports.

### 6.2. Passport Issuance Process

The process of issuing an Agent Passport involves several key steps to ensure its integrity and authenticity:

1.  **Passport Creation:** The agent developer or an automated system creates the Agent Passport document in YAML format, specifying all required fields as per Section 5, including identification, entity description, capabilities, resource requirements, and security policies.
2.  **Submission to Issuing Authority:** The created (unsigned) passport is submitted to a designated Issuing Authority.
3.  **Vetting and Validation:** The Issuing Authority performs necessary checks to validate the information within the passport. This may include:
      * Syntactic validation of the YAML structure against the Agent Passport schema.
      * Semantic validation of declared capabilities and resource requirements (e.g., do they align with expected agent behavior?).
      * Security policy review to ensure compliance with organizational or industry standards.
      * Identity verification of the agent developer or requesting entity.
      * Potentially, static analysis or dynamic testing of the agent's code/binary if applicable.
4.  **Digital Signing:** Upon successful validation, the Issuing Authority computes a cryptographic hash of the entire Agent Passport document (excluding the `signature` field itself) and digitally signs this hash using its private key. The resulting digital signature, along with the algorithm used and a reference to the public key, is embedded into the `signature` field of the passport.
5.  **Passport Distribution:** The signed Agent Passport is then made available for use by the agent and relying parties.

### 6.3. Passport Verification Process

Any entity (e.g., another agent, an agent runtime, a policy engine) that needs to trust an Agent Passport will perform a verification process:

1. **Retrieval of Passport and Public Key:** The verifier obtains the Agent Passport and the corresponding public key of the Issuing Authority from a trusted source (e.g., a public key registry, a DID resolver, or a pre-configured trust anchor).
2. **Signature Validation:** The verifier re-computes the cryptographic hash of the Agent Passport document, ensuring a consistent, canonical representation of the passport data (excluding the `signature` field) is used for hashing. It then uses the Issuing Authority’s public key to verify the provided digital signature (`signature.value`) against the re-computed hash. If they match, the integrity and authenticity of the passport are confirmed.
3. **Schema and Content Validation:** Even after signature validation, the verifier must validate the passport against the Agent Passport schema (Section 5) and check for semantic consistency of its contents. This includes verifying the `apiVersion`, `kind`, and the structure of all declared fields.
4. **Lifecycle Validation:** The verifier checks the `issueDate` and `expiryDate` (if present) to ensure the passport is currently valid. It may also check for revocation status against a Certificate Revocation List (CRL) or Online Certificate Status Protocol (OCSP) equivalent maintained by the Issuing Authority.
5. **Policy Enforcement Check:** For runtimes or policy engines, the verification process extends to evaluating whether the declared security policies can be enforced and if the agent's requested actions comply with those policies.

### 6.4. Lifecycle Management

Similar to human passports, Agent Passports require a lifecycle management framework to ensure their ongoing trustworthiness:

  * **Issuance:** The initial creation and signing of the passport as described in 6.2
  * **Expiration:** Passports can be issued with an `expiryDate` (see 5.1.1) after which they are no longer considered valid. This encourages regular re-validation and updates.
  * **Renewal:** Before expiration, a passport can be renewed by the Issuing Authority, potentially after re-validation of the agent and its properties. A renewal typically results in a new passport with an updated `issueDate` and `expiryDate` (and potentially `metadata.version`).
  * **Revocation:** An Agent Passport can be revoked by the Issuing Authority before its `expiryDate` if the agent's trust is compromised (e.g., due to malicious behavior, security vulnerability discovery, or change in ownership). Issuing Authorities must provide a mechanism (e.g., a revocation list or status service) for relying parties to check the revocation status of a passport.

## 7. Security Considerations

The Agent Passport specification is designed with security as a paramount concern, addressing inherent vulnerabilities in AI agent systems and ensuring the trustworthiness of agent operations.

### 7.1. Threats to AI Agent Systems (General)

AI agent systems face a range of security threats that necessitate robust protective measures. These include:

  * Unauthorized Access
  * Malicious Agents
  * Impersonation/Spoofing
  * Data Tampering
  * Resource Exhaustion
  * Vulnerability Exploitation

The Agent Passport aims to mitigate many of these threats by providing a secure and verifiable foundation for agent identity, capabilities, and enforced policies.

### 7.2. Risks to Passport Integrity and Authenticity

Specific risks related to the Agent Passport itself include:

  * **Forgery**: This refers to the creation of entirely fake Agent Passports by malicious actors. A forged passport might falsely claim an agent has certain capabilities, permissions, or a legitimate identity, with the aim of deceiving other agents or systems into granting unauthorized access or trusting a malicious entity.
  * **Tampering**: This involves the unauthorized modification of a legitimate, signed Agent Passport. An attacker might attempt to alter existing fields, such as an agent's declared capabilities, resource requirements, security policies, or even its identity information, after it has been legitimately issued and signed.
  * **Replay Attacks**: This risk involves a malicious actor capturing a valid, signed Agent Passport and re-using it in an unauthorized context or after it should no longer be considered valid (e.g., after its expiration or revocation, or in a different transaction than intended). The passport itself is legitimate, but its use in the replay scenario is fraudulent.
  * **Disclosure of Sensitive Information**: If an Agent Passport contains sensitive data (e.g., internal network endpoints, proprietary capability details, specific configurations that could reveal vulnerabilities), there is a risk of this information being exposed to unauthorized parties if the passport is not adequately protected during transmission or storage.

### 7.3. Risks during Issuance and Verification

Vulnerabilities can arise during the lifecycle of the Agent Passport:

  * **Compromised Issuing Authority**: This occurs if an attacker gains control over an Issuing Authority's private signing key(s). With a compromised key, an attacker could sign and issue fraudulent Agent Passports that would appear legitimate to relying parties, or they could falsely revoke valid passports, undermining the entire chain of trust.
  * **Weak Verification Procedures**: This risk arises if entities verifying an Agent Passport do not perform all necessary checks thoroughly. Examples include not properly validating the digital signature, failing to check the passport's expiration date or revocation status, not validating the passport against its schema, or accepting passports from untrusted Issuing Authorities. Such weaknesses can lead to the acceptance of invalid, revoked, or forged passports.
  * **Man-in-the-Middle Attacks**: These attacks can occur during the transmission of an Agent Passport or during the retrieval of an Issuing Authority's public key. An attacker could intercept and alter the passport data in transit or substitute a legitimate public key with their own, potentially leading to the acceptance of a modified passport or the validation of a forged passport signed with the attacker's key.

### 7.4. Mitigating Risks through Digital Signatures and Verification

Digital signatures are fundamental to securing the Agent Passport:

  * **Integrity**: This ensures that the Agent Passport's content has not been altered or tampered with since it was signed by the Issuing Authority. Any modification to the passport data, however minor, will invalidate the digital signature, immediately alerting verifiers to the change.
  * **Authenticity**: This confirms that the Agent Passport was indeed issued by the declared Issuing Authority and not by an impostor. The digital signature can only be successfully verified using the legitimate Issuing Authority's public key, thus proving its origin.
  * **Non-Repudiation**: This provides strong evidence that the Issuing Authority did, in fact, sign and approve the specific content of the Agent Passport. Once signed, the Issuing Authority cannot later deny having issued that particular passport with those specific details, assuming their private key was not compromised.
  * **Chain of Trust**: This refers to the ability to establish a verifiable link from the Agent Passport and its Issuing Authority back to a root of trust. By relying on trusted Issuing Authorities and potentially secure public key distribution mechanisms (e.g., Public Key Infrastructure (PKI), trusted DID registries), a verifier can be confident in the legitimacy of the passport.

### 7.5. Role of Policies in Mitigating Agent-Specific Risks

The declarative security policies embedded within the Agent Passport are crucial for runtime risk mitigation:

  * **Principle of Least Privilege**: This security design principle dictates that an agent should only be granted the exact permissions and access rights necessary to perform its intended tasks, and no more. By explicitly defining allowed system calls (e.g., via `seccomp`), file system access, network connections, and capabilities within the passport, runtimes can enforce this principle, significantly reducing the potential damage if an agent is compromised or behaves unexpectedly.
  * **Containment**: This refers to mechanisms that restrict an agent's operational environment to limit its potential impact on the wider system, even if it becomes malicious or faulty. Policies like `chroot` (restricting file system view to a specific directory) and `seccomp` (filtering allowed system calls) provide strong containment, isolating the agent and preventing it from accessing or affecting unauthorized resources.
  * **Networkу Security**: This involves controlling an agent's ability to communicate over the network. By declaring allowed inbound and outbound connections (e.g., specific IP addresses, domains, ports, and protocols) in the passport's `networkRestrictions` or `resources.network` sections, unauthorized network access, data exfiltration, or participation in distributed attacks can be prevented.
  * **Resource Control**: This ensures that an agent does not consume excessive system resources (like CPU, memory, storage, or network bandwidth), which could lead to denial of service for other agents or the host system. Declaring resource requirements and limits in the passport allows the runtime environment to allocate resources appropriately and enforce these limits, preventing resource exhaustion attacks or accidental overconsumption.

### 7.6. Encryption Considerations

While the primary focus of the Agent Passport is on integrity and authenticity through signing, encryption may be considered for specific use cases:

  * **Confidentiality of Sensitive Passport Data**: This addresses the need to protect specific information within an Agent Passport from unauthorized viewing. If a passport contains data that is considered sensitive (e.g., proprietary algorithms, internal resource locators, or specific security configurations that could be exploited if known), parts of the passport or the entire document could be encrypted. This would ensure that only authorized entities with the correct decryption key can access this sensitive information, even if they possess the passport.
  * **Transport Layer Security (TLS)**: This refers to the use of standard cryptographic protocols, like TLS (or its predecessor SSL), to secure the communication channels used during the lifecycle of an Agent Passport. This includes encrypting the connection when a passport is being submitted to an Issuing Authority, distributed to relying parties, or when public keys are being retrieved. TLS protects passport data against eavesdropping and tampering while it is in transit over a network.
  * **Key Management**: This encompasses the secure generation, storage, distribution, rotation, and revocation of cryptographic keys used for encrypting passport data or sections. Robust key management practices are essential if encryption is employed, as the security of the encrypted data directly depends on the security of the encryption keys. Compromise of these keys would lead to a loss of confidentiality.

### 7.7. Audit and Compliance

The Agent Passport facilitates auditing and compliance:

  * **Accountability**: This refers to the ability to trace an agent's actions back to its verified identity and its authorized set of capabilities and policies as defined in its passport. If an agent performs an undesirable action, the passport provides a clear, attested record of what the agent should have been capable of and who (the Issuing Authority) vouched for its definition. This, combined with runtime logs, helps in determining responsibility.
  * **Regulatory Compliance**: Many industries and jurisdictions have regulations regarding data protection, security, and operational conduct (e.g., GDPR, HIPAA, financial regulations). The Agent Passport helps organizations demonstrate compliance by providing a verifiable and auditable record of an agent's identity, its explicitly declared security policies, and its operational constraints, showing due diligence in managing AI agent risks.
  * **Forensics**: In the event of a security incident or system malfunction involving an AI agent, the Agent Passport serves as a crucial piece of evidence. It provides a baseline of the agent's intended design, permissions, and security posture at the time of issuance. This information can significantly aid investigators in understanding the scope of a compromise, how an agent might have been misused, or whether it operated outside its defined parameters.

## 8. Extensibility

The Agent Passport specification is designed to be extensible, allowing for future growth and adaptation to evolving AI agent requirements without invalidating existing implementations. This is achieved through defined mechanisms for schema extension and a clear versioning strategy.

### 8.1. Mechanisms for Extending the Passport Schema

To ensure forward compatibility and flexibility, the Agent Passport specification provides the following extensibility mechanisms:

  * **Addition of New Fields:** New fields CAN be added to existing objects within the Agent Passport schema. Implementations MUST be designed to gracefully ignore any unknown fields they encounter.
  * **Addition of New Top-Level Sections:** Entirely new top-level sections CAN be introduced.
  * **x- Prefixes for Experimental Fields (Proposed):** For experimental or vendor-specific fields, a convention of using an `x-` prefix (e.g., `x-custom-field`) is proposed.

### 8.2. Versioning Strategy

The Agent Passport employs a two-tiered versioning strategy:

  * **API Versioning (`apiVersion`):** This top-level field (e.g., `agent-passport.io/v1alpha1`) indicates the version of the Agent Passport specification schema.
      * `v1alpha1`: Initial, experimental, potentially unstable.
      * `v1beta1`: More stable, breaking changes less likely but possible.
      * `v1` (and subsequent major versions): Stable, backward-compatible.
  * **Passport Versioning (`metadata.version`):** This field (e.g., `metadata.version: "1.0.0"`) denotes the specific version of an individual agent's passport definition. This allows iteration on an agent's definition without changing the API version.

## 9. Use Cases

The Agent Passport specification provides a foundational framework that enables a wide array of critical use cases within dynamic AI agent ecosystems.

### 9.1. Agent Discovery and Trust Establishment

The Agent Passport significantly streamlines how agents discover each other and how trust is established between them:

  * **Verifiable Identity**: This refers to an agent's ability to present its digitally signed passport to cryptographically prove its identity and origin (i.e., who issued the passport). This goes beyond a simple name or ID, offering a secure assertion of "who" the agent is and "who" (the Issuing Authority) vouches for its declared attributes, forming a foundational layer for trust.
  * **Capability Advertisement**: The capabilities section of the passport allows agents to explicitly and transparently declare their available functions, methods, or services in a standardized, machine-readable format. This facilitates automated discovery of services by other agents or systems, enabling them to understand what an agent can do and how to interact with it, which is crucial for dynamic composition of multi-agent workflows.
  * **Trust Negotiation**: Before engaging in significant communication, collaboration, or task delegation, agents (or their hosting environments) can exchange and verify each other's passports. This process involves validating the passport's authenticity, checking the declared security policies, and assessing the trustworthiness based on the Issuing Authority and the passport's content. This forms a verifiable and explicit basis for deciding whether to trust and interact with another agent.

### 9.2. Policy Enforcement

One of the primary use cases for the Agent Passport is to enable robust and dynamic policy enforcement at runtime:

  * **Granular Security Policies**: This refers to the capability of the Agent Passport to define specific, fine-grained security rules and constraints directly within its `securityPolicies` section (e.g., `seccomp` for system call filtering, `chroot` for filesystem isolation, `capabilities` for Linux capabilities, `networkRestrictions`). These declarative policies allow for precise control over what an agent is permitted to do and access.
  * **Runtime Environment Control**: This describes how agent runtimes (like the proposed `agentifyd` or other compatible systems) can parse the security policies declared in an Agent Passport and use them to configure and constrain the agent's execution environment. This ensures that the agent operates strictly within its authorized boundaries, preventing unauthorized actions or access.
  * **Dynamic Policy Adaptation**: This highlights the potential for runtime environments to adjust or apply an agent's operational context based on the policies defined in its passport, potentially adapting to different trust levels, data sensitivities, or operational phases. For example, an agent might be launched with stricter containment if its passport indicates it will handle highly sensitive data, or its network access might be modified based on its current task and declared policies.

### 9.3. Multi-Agent System Deployment

The Agent Passport simplifies the deployment and management of complex multi-agent systems:

  * **Automated Provisioning**: This refers to the ability of orchestration systems (e.g., Kubernetes, custom agent management platforms) to automatically allocate and configure the necessary computational resources (CPU, memory, storage) and set up execution environments (e.g., container settings, network access rules) for an agent based on the `resources` section declared in its passport. This streamlines the deployment process and ensures agents get the environment they need to operate correctly and securely.
  * **Interoperability Across Frameworks**: By providing a common, standardized, LLM-agnostic, and machine-parsable format for describing agents, their capabilities, and security requirements, the Agent Passport aims to foster interoperability. This means agents defined with a passport could potentially be deployed, understood, and interact more seamlessly across different AI agent development frameworks (e.g., LangChain, AIOS, AutoGen) and cloud platforms that adopt the specification.
  * **Simplified Management**: A standardized Agent Passport reduces the complexity of managing diverse fleets of AI agents. Having a consistent, verifiable document that details each agent's core attributes, capabilities, resource needs, and security posture provides a unified view for administrators, simplifying tasks like inventory management, compliance checks, security audits, and updates across the agent ecosystem.

### 9.4. Auditing and Compliance

The verifiable nature of the Agent Passport is crucial for auditing and compliance in regulated industries:

  * **Enhanced Accountability**: This refers to the ability to definitively link an agent's actions to its verified identity and its authorized set of capabilities as attested by a trusted Issuing Authority. The passport provides a robust, non-repudiable foundation for determining responsibility if an agent causes an incident, as there is a clear record of its intended permissions and identity.
  * **Simplified Auditing**: This describes how the Agent Passport provides a standardized, human-readable, and cryptographically verified document for auditors. Instead of inspecting complex codebases or disparate configuration files, auditors can review the passport to quickly understand an agent's permissions, resource access, and security policies, confident that the document itself has not been tampered with since issuance.
  * **Demonstrable Compliance**: This highlights the role of the Agent Passport as tangible evidence for meeting regulatory or organizational requirements (e.g., GDPR, HIPAA). Organizations can present the passports of their agents to prove that security controls are in place, that the principle of least privilege is being enforced, and that agent identities are managed securely, thereby demonstrating due diligence.

## 10. IANA Considerations

This section is reserved for considerations regarding registrations with the Internet Assigned Numbers Authority (IANA). While this is a draft specification, future versions of this RFC may request the registration of a media type (e.g., `application/vnd.agent-passport+yaml`) or a specific file extension for Agent Passport files. Additionally, considerations for URI schemes related to Agent Passports or Issuing Authorities might arise. These will be detailed in future iterations pending further development and community consensus.

## 11. Acknowledgements

The authors would like to thank the following individuals and organizations for their contributions to the ideas, discussions, and reviews during the development of this specification (even at these early stages):

  * Aleksandr Nuikin

The authors also acknowledge the foundational work in related areas that has inspired this specification.

## 12. References

### 12.1. Normative References

  * **YAML Specification:** The official specification for YAML Ain't Markup Language (YAML™) Version 1.2.2.
  * **RFC 2119:** S. Bradner, "Key words for use in RFCs to Indicate Requirement Levels," March 1997.
  * **RFC 8174:** B. Leiba, "Ambiguity of Uppercase vs Lowercase in RFC 2119 Key Words," May 2017 (clarifies RFC 2119 usage).
  * **Cryptographic Signature Standards:** Relevant industry standards for digital signatures (e.g., RSASSA-PSS, EdDSA) and hashing algorithms (e.g., SHA-256) as defined by NIST or other recognized bodies.

### 12.2. Informative References

  * **A2A Protocol:** Reference to existing agent-to-agent communication protocols, specifically where the "Agent Card" concept is introduced. (e.g., [https://a2aprotocol.ai](https://a2aprotocol.ai)).
  * **Decentralized Identifiers (DIDs) v1.0:** W3C Recommendation. (e.g., [https://www.w3.org/TR/did-core/](https://www.w3.org/TR/did-core/)).
  * **Verifiable Credentials Data Model v1.0:** W3C Recommendation. (e.g., [https://www.w3.org/TR/vc-data-model/](https://www.w3.org/TR/vc-data-model/)).
  * **seccomp (Secure Computing Mode):** Linux kernel feature for syscall filtering. (e.g., [https://man7.org/linux/man-pages/man2/seccomp.2.html](https://man7.org/linux/man-pages/man2/seccomp.2.html))
  * **chroot:** Unix operation to change the apparent root directory. (e.g., [https://man7.org/linux/man-pages/man2/chroot.2.html](https://man7.org/linux/man-pages/man2/chroot.2.html))
  * **Linux Capabilities:** System for partitioning superuser privileges. (e.g., [https://man7.org/linux/man-pages/man7/capabilities.7.html](https://man7.org/linux/man-pages/man7/capabilities.7.html))
  * **Agentification Layer (AL) Project Documentation:** Internal or public documentation for the 0AL project, `zeroald` runtime, `agentifyd` daemon, and AgentHub. (e.g., [https://github.com/0al-spec](https://github.com/0al-spec))

## 13. Change Log / Version History

  * **Version 0.1 (2025-07-20):** Initial public draft of the Agent Passport Specification.

## 14. Appendix A. Comparison with A2A Agent Card

The Agent Passport builds upon, and significantly extends, concepts found in simpler agent description mechanisms, such as the A2A Agent Card. Key differences are highlighted below:

| Feature | A2A Agent Card | Agent Passport |
| :----------------------------- | :-------------------------------------- | :----------------------------------------------------------------------------------------------- |
| **Primary Purpose** | Public "business card," basic description | Official, verifiable "document" for identity, capabilities, and security policies |
| **Identity** | Simple identifier, name | Unique ID (UUID/DID), cryptographically verifiable identity |
| **Security Policies** | Limited or implicit | Explicit, granular policies (`seccomp`, `chroot`, `cap-lists`, network restrictions, access control, etc.) |
| **Verifiability** | Minimal or none | Digital signatures by trusted Issuing Authorities; verifiable integrity and authenticity       |
| **Trust Model** | Assumed trust, often implicit | Explicit chain of trust established through Issuing Authorities and cryptographic verification |
| **Protection** | Not typically protected from tampering | Cryptographically protected against forgery and unauthorized changes |
| **Issuance** | Ad-hoc or informal | Formalized process by designated Issuing Authorities |
| **Lifecycle** | Basic or none | Supports issuance, expiration, renewal, and revocation |
| **Scope** | Primarily descriptive | Declarative for runtime enforcement, security, auditing, and interoperability |
| **Underlying Format** | Varies (often JSON) | Primarily YAML |

## 15. Appendix B. Open Issues and Future Considerations

This appendix outlines areas for future work and topics requiring further discussion:

* Standardization Body Engagement
* Registry for Issuing Authorities
* Advanced Policy Expressions
* Telemetry and Monitoring Integration
* Attestation for Agent Behaviors
* Privacy Considerations
* Interoperability with Existing Identity Systems (IAM, OAuth2.0, OpenID Connect)
* Formal Verification
* Decentralized Storage
* Economic Models
