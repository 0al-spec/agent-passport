# Contributing to Agent Passport RFC

Thank you for considering contributing to this specification!

This repository contains the canonical draft of the **Agent Passport** specification — a declarative YAML-based format for defining, securing, and verifying AI agents in a multi-agent zero-trust environment.

## 📁 Repository Structure

```
agent-passport/
├── drafts/         # Source RFCs written in Markdown (kramdown-rfc2629)
├── schema/         # Optional JSON/YAML Schemas
├── generated/      # Auto-generated XML, TXT, PDF
├── examples/       # Sample agent passports
├── Makefile        # Build script
└── .github/workflows/  # CI pipeline for validation
```

## 💡 How to Contribute

We welcome contributions of all kinds:

- ✍️ Edits or improvements to the RFC text
- 📐 Suggestions on structure or terminology
- 🧪 YAML schema improvements
- 📄 New use cases or examples
- 🛠 Bug fixes in the build process

## 🧑‍💻 Getting Started

1. **Fork** this repository.
2. **Create a new branch** from `main`:
   ```bash
   git checkout -b feature/my-change
   ```
3. **Edit** the RFC in Markdown (`drafts/agent-passport.md`) or add new files in `drafts/`.
5. **Submit a Pull Request** to the `main` branch.

## ✅ Style Guidelines

- Write in clear, formal English using the [RFC Style Guide](https://www.rfc-editor.org/materials/)
- Use normative language (`MUST`, `SHOULD`, etc.) consistently per [RFC 2119](https://www.rfc-editor.org/rfc/rfc2119.html)
- Keep schema and examples in sync with the RFC text

## 📝 License

By contributing, you agree to license your work under the terms of the repository’s license (Creative Commons Attribution 4.0 International).

## 🙏 Thank You

Your feedback, ideas, and improvements help make this a stronger and more secure foundation for agent-based software.
