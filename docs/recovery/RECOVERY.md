# ARK Recovery Architecture

Status: ACCEPTED three-route recovery direction. Exact cryptographic envelope and user flows require implementation and review.

## 1. Recovery goals

Recover access without silently weakening encryption or making an unsupported promise. Recovery material must be scoped to a workspace and protected against theft, replay, and accidental disclosure.

## 2. Recovery routes

### A. Workspace Recovery Key

- Independently generated random secret, target 256 bits from OS CSPRNG.
- Unique per workspace.
- Not stored raw on ARK servers.
- Independent of the workspace password and third-party login.
- Used to unwrap a protected workspace key wrapper through a reviewed construction.
- Shown during setup with explicit safe-storage guidance and a recovery verification challenge.

### B. Recovery File

- Contains authenticated encrypted wrapper material and required versioned metadata.
- Does not contain a raw Workspace Master Key.
- Is a protected recovery container, not an independent secret factor by itself.
- Must be stored separately from the encrypted workspace where possible.
- Must be validated before use and protected against rollback/substitution.

### C. Trusted Device

- Each device has a device-specific private key generated and protected using an appropriate OS facility.
- Recovery requires proof of possession and authorization for the workspace wrapper.
- Device registration, rotation, and revocation are audited.
- A device must not be trusted solely because it shares an account email or username.

## 3. Account login

External login (for example OIDC) identifies the account; it does not automatically decrypt a workspace. Google or another identity provider is not a workspace key escrow service unless a separate explicit architecture is designed and accepted.

## 4. Rotation and revocation

- Create a new wrapper.
- Verify that the new wrapper can unwrap the intended key.
- Atomically publish the new state.
- Revoke the old wrapper after successful commit.
- Preserve a documented recovery/rollback path during the transition.
- Notify/audit sensitive recovery changes.
- Re-evaluate sessions and credentials according to policy.

## 5. Loss scenarios

If the workspace password, Recovery Key, Recovery File, and all trusted devices are lost, ARK cannot guarantee recovery without an explicitly designed escrow mechanism. Do not invent a backdoor or claim zero-knowledge recovery in that situation.

Enterprise escrow is deferred and must be a separate threat-model and key-management design.

## 6. Testing

Test wrong key, corrupted file, stale wrapper, revoked device, device replacement, interrupted rotation, duplicate recovery attempts, rollback, and cross-device restore. Verify that recovery does not accidentally grant unrelated workspace or package access.
