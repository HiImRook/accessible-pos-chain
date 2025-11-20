# Security Policy

Thank you for taking the time to responsibly disclose a potential security issue. I take reports seriously and will respond as quickly as possible.

## Supported Versions
This repository is a research / experimental project. Please assume the current `main` branch is the most current working code There are no guaranteed long‑term supported releases unless explicitly tagged. If a vulnerability affects a tagged release, please reference the tag or commit SHA in your report.

## Reporting a Vulnerability (Preferred)
Preferred method (private): create a GitHub Security Advisory for this repository:
https://github.com/HiImRook/accessible-pos-chain/security/advisories/new

Using a GitHub Security Advisory allows me to coordinate privately, track remediation, and produce an advisories/patch timeline.

If you cannot use GitHub Security Advisories, you may contact me via X or Discord. **Do not** open a public issue labeled `security` (**note: public issues are visible to everyone**).

## What to include in your report
Please include as much of the following as you can:
- A short summary of the issue and its impact.
- Steps to reproduce the issue (exact commands, payloads, and environment).
- The commit SHA, branch, or release tag where the issue was observed.
- Any proof‑of‑concept (PoC) code or data that reproduces the behavior.
- Logs, stack traces, or network captures that help diagnose the problem.
- Your contact information and preferred method for follow‑up.
- If you need to share sensitive data (keys, dumps), consider encrypting that data with PGP and including the public key fingerprint or sending it via the GitHub Advisory upload mechanism.

## How I handle reports
- Acknowledgement: I will acknowledge the report within 72 hours of receipt (via the Security Advisory thread or Discord DM).
- Triage & remediation: I will triage the issue, reproduce it, and work on a fix. For high‑severity issues I will prioritize the patch and coordinate disclosure.
- Communication: I will keep you updated on progress in the Security Advisory thread or via the channel you used to contact me.
- Disclosure: I will coordinate any public disclosure with the reporter and request a reasonable embargo to allow a fix to be deployed before public disclosure.

## Safe harbor
If you act in good faith to report a potential vulnerability, I will not pursue legal action against you, provided you:
- Do not access, modify, or exfiltrate data beyond what is necessary to demonstrate the issue;
- Do not intentionally disrupt production systems, and
- Give me a reasonable chance to respond and remediate before disclosing details publicly.

## Immediate mitigations
- Do not post private keys, passwords, or sensitive data in public issues.
- If you believe keys have been exposed, rotate them immediately and notify me via the Advisory channel.
- If you find an exploitable issue in a running network, do not attempt to extract value or perform actions that could cause harm. Report it immediately via the Advisory channel.

## Contact & alternative channels
- Preferred: GitHub Security Advisory (private) — https://github.com/HiImRook/accessible-pos-chain/security/advisories/new
- Alternative: X or Discord DM
- Public issues are **not recommended** for security reports.

Thank you — coordinated disclosure helps keep users safe and helps me fix issues faster.
