# ADR-0003: Cryptographic Primitive Families and Key Hierarchy

Status: Accepted direction; exact construction and implementation remain gated by independent review.

## Context

ARK needs authenticated encryption at rest, password-based unlock, context-separated derivation, portable packages, and multiple recovery routes.

## Decision

Use established primitive families: XChaCha20-Poly1305, Argon2id, HKDF-SHA-256, and OS CSPRNG. Do not invent cryptographic algorithms.

Workspace password derives a KEK using Argon2id; the KEK unwraps a random Workspace Master Key. Scoped project/file keys are derived or wrapped through a reviewed, versioned hierarchy. Password rotation normally rewraps keys rather than re-encrypting all data.

Each portable package has an independent random Package Key and a package-specific password-derived wrapper.

## Consequences

Exact library/version/features, KDF parameters, key labels, nonce allocation, chunk format, and migration rules must be selected and reviewed before implementation is accepted.

## Security implications

Nonce uniqueness, key separation, authenticated metadata, rollback limitations, memory exposure, and recovery wrappers are release-critical.

## Validation

Published vectors, negative/tamper tests, fuzzing, nonce lifecycle analysis, dependency review, and independent cryptographic review.
