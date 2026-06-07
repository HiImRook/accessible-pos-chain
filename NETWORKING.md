# Network Identity — v0.6.2 Status

> ⚠️ **Read this before running a public testnet or forking for production use.**

---

## Current Architecture (v0.6.2)

Valid Blockchain v0.6.2 introduces validator-aware peer binding and startup quorum gating by carrying validator identity in the direct peer handshake. This ensures correct chain startup and TPI quorum formation on trusted validator networks.

**This is a transitional bootstrap mechanism. It is not the final network identity architecture.**

Validator identity is visible to directly connected peers under the current handshake model. Without transport encryption, a network observer may also learn validator-to-IP correlations depending on network path and deployment.

---

## Testnet Status

Testnets should be treated as **private / trusted-validator networks** until v0.7.0 network identity hardening lands.

- ✅ Private bootstrap testnet with known validators — acceptable
- ✅ Controlled operator network — acceptable
- ❌ Public or adversarial validator testnet — not recommended
- ❌ Anonymous validator participation — not yet supported

---

## Guidance for Forks

Forks building on v0.6.2 should keep validator testnets private until the planned network identity and privacy hardening updates are released. If a fork chooses to run a public testnet before then, it should do so with the explicit understanding that validator identity privacy and peer-level exposure protections are not yet finalized.

---

## Planned Replacement (v0.7.0)

v0.6.2 validator identity in handshake will be replaced in v0.7.0 by a privacy-preserving model based on:

- Ephemeral network identity
- Validator proof/binding without direct identity disclosure in ordinary peer discovery
- Transport hardening

Until that work lands, the current design is intentionally simple and scoped to trusted bootstrap networks only.

---

**"You cannot leak what you never kept."**
