//! Shared runtime vocabulary.

/// The product workflow selected for a runtime session.
///
/// A mode describes the access pattern. It does not authorize a process, select a security
/// level, or bypass identity, session, broker, or policy checks.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RuntimeMode {
    /// A bounded working set for editing and development tools.
    Development,
    /// Explicit resource scopes for an application executing a project.
    Execution,
}

#[cfg(test)]
mod tests {
    use super::RuntimeMode;

    #[test]
    fn runtime_modes_are_explicit_and_distinct() {
        assert_ne!(RuntimeMode::Development, RuntimeMode::Execution);
        assert_eq!(RuntimeMode::Development, RuntimeMode::Development);
    }
}
