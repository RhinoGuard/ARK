# Security Policy

## Project maturity

ARK is under active early-stage development. It is not currently represented as production-ready or independently security-audited. Do not use it as the sole protection for irreplaceable or highly sensitive data until an appropriate release and independent review process exists.

## Reporting a vulnerability

Do not disclose suspected vulnerabilities in public issues, discussions, pull requests, or social media.

Report privately to the repository maintainers through GitHub's private vulnerability reporting feature, if enabled, or contact the maintainers through a verified private channel. Include:
- affected commit, release, component, and platform;
- concise impact and threat assumptions;
- reproducible steps or a minimal proof of concept;
- expected versus actual behavior;
- whether the report may expose user data or secrets.

Do not attach real workspace contents, passwords, private keys, recovery material, or third-party data. Use synthetic test data.

## Triage principles

Maintainers will assess exploitability, affected versions, confidentiality/integrity/availability impact, prerequisites, and mitigations. Critical/high findings must be resolved or explicitly block release. Coordinate disclosure with maintainers; do not assume a fixed response SLA until one is published.

## Security boundaries

ARK aims to protect encrypted data at rest and mediate supported application access. It cannot guarantee confidentiality against a fully compromised OS/kernel, malicious administrator, compromised hypervisor, or an authorized application that receives plaintext and exfiltrates it outside ARK's control. These boundaries must remain explicit in product documentation.
