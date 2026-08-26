use std::fmt;

/// Errors produced by framework-independent domain validation and ports.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum DomainError {
    InvalidValue {
        field: &'static str,
        reason: String,
    },
    InvalidTransition {
        entity: &'static str,
        from: &'static str,
        to: &'static str,
    },
    Conflict {
        resource: String,
        detail: String,
    },
    Authorization {
        action: &'static str,
        reason: String,
    },
    Trust {
        operation: &'static str,
        reason: String,
    },
    Infrastructure {
        operation: &'static str,
        detail: String,
    },
}

impl fmt::Display for DomainError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidValue { field, reason } => write!(formatter, "invalid {field}: {reason}"),
            Self::InvalidTransition { entity, from, to } => {
                write!(formatter, "cannot transition {entity} from {from} to {to}")
            }
            Self::Conflict { resource, detail } => {
                write!(formatter, "conflict for {resource}: {detail}")
            }
            Self::Authorization { action, reason } => {
                write!(formatter, "not authorized to {action}: {reason}")
            }
            Self::Trust { operation, reason } => {
                write!(formatter, "trust policy rejected {operation}: {reason}")
            }
            Self::Infrastructure { operation, detail } => {
                write!(
                    formatter,
                    "infrastructure failure during {operation}: {detail}"
                )
            }
        }
    }
}

impl std::error::Error for DomainError {}

pub type CoreResult<T> = Result<T, DomainError>;
