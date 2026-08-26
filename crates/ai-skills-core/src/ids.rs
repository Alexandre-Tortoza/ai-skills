use std::{fmt, str::FromStr};

use uuid::Uuid;

use crate::{CoreResult, DomainError};

macro_rules! typed_id {
    ($name:ident, $field:literal) => {
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(Uuid);

        impl $name {
            #[must_use]
            pub fn new() -> Self {
                Self(Uuid::now_v7())
            }

            #[must_use]
            pub const fn from_uuid(value: Uuid) -> Self {
                Self(value)
            }

            #[must_use]
            pub const fn as_uuid(self) -> Uuid {
                self.0
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(formatter)
            }
        }

        impl FromStr for $name {
            type Err = DomainError;

            fn from_str(value: &str) -> CoreResult<Self> {
                Uuid::parse_str(value)
                    .map(Self)
                    .map_err(|error| DomainError::InvalidValue {
                        field: $field,
                        reason: error.to_string(),
                    })
            }
        }
    };
}

typed_id!(SkillId, "skill ID");
typed_id!(RevisionId, "revision ID");
typed_id!(ProposalId, "proposal ID");
typed_id!(ProfileId, "profile ID");
typed_id!(SourceId, "source ID");
typed_id!(HumanActorId, "human actor ID");
typed_id!(MachineActorId, "machine actor ID");
