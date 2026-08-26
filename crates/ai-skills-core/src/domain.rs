use std::collections::HashSet;

use crate::{
    CoreResult, DomainError, HumanActorId, MachineActorId, ProfileId, ProposalId, RevisionId,
    SkillId, SourceId,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SkillName(String);

impl SkillName {
    pub fn new(value: impl Into<String>) -> CoreResult<Self> {
        let value = value.into();
        validate_text("skill name", &value)?;
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ProfileName(String);

impl ProfileName {
    pub fn new(value: impl Into<String>) -> CoreResult<Self> {
        let value = value.into();
        validate_text("profile name", &value)?;
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ContentHash(String);

impl ContentHash {
    pub fn new(value: impl Into<String>) -> CoreResult<Self> {
        let value = value.into();
        validate_text("content hash", &value)?;
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Ownership {
    User,
    Organization,
    ThirdParty,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TrustLevel {
    Untrusted,
    Trusted,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ActivationMode {
    Pinned,
    Active,
    DiscoveryOnly,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SourceKind {
    Local,
    Git,
    GitHub,
    Generated,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Source {
    id: SourceId,
    kind: SourceKind,
    uri: String,
}

impl Source {
    pub fn new(id: SourceId, kind: SourceKind, uri: impl Into<String>) -> CoreResult<Self> {
        let uri = uri.into();
        validate_text("source URI", &uri)?;
        Ok(Self { id, kind, uri })
    }

    #[must_use]
    pub const fn id(&self) -> SourceId {
        self.id
    }

    #[must_use]
    pub const fn kind(&self) -> SourceKind {
        self.kind
    }

    #[must_use]
    pub fn uri(&self) -> &str {
        &self.uri
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SkillOrigin {
    ownership: Ownership,
    source_id: Option<SourceId>,
}

impl SkillOrigin {
    pub fn new(ownership: Ownership, source_id: Option<SourceId>) -> CoreResult<Self> {
        if ownership == Ownership::ThirdParty && source_id.is_none() {
            return Err(DomainError::InvalidValue {
                field: "skill origin",
                reason: "third-party skills require source provenance".to_owned(),
            });
        }

        Ok(Self {
            ownership,
            source_id,
        })
    }

    #[must_use]
    pub const fn ownership(&self) -> Ownership {
        self.ownership
    }

    #[must_use]
    pub const fn source_id(&self) -> Option<SourceId> {
        self.source_id
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RevisionSource {
    source_id: SourceId,
    upstream_revision: Option<String>,
    source_path: String,
}

impl RevisionSource {
    pub fn new(
        source_id: SourceId,
        upstream_revision: Option<String>,
        source_path: impl Into<String>,
    ) -> CoreResult<Self> {
        let source_path = source_path.into();
        validate_text("source path", &source_path)?;
        if let Some(revision) = &upstream_revision {
            validate_text("upstream revision", revision)?;
        }

        Ok(Self {
            source_id,
            upstream_revision,
            source_path,
        })
    }

    #[must_use]
    pub const fn source_id(&self) -> SourceId {
        self.source_id
    }

    #[must_use]
    pub fn upstream_revision(&self) -> Option<&str> {
        self.upstream_revision.as_deref()
    }

    #[must_use]
    pub fn source_path(&self) -> &str {
        &self.source_path
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Skill {
    id: SkillId,
    name: SkillName,
    description: String,
    origin: SkillOrigin,
    activation: ActivationMode,
    current_revision_id: RevisionId,
}

impl Skill {
    pub fn new(
        id: SkillId,
        name: SkillName,
        description: impl Into<String>,
        origin: SkillOrigin,
        activation: ActivationMode,
        current_revision_id: RevisionId,
    ) -> CoreResult<Self> {
        let description = description.into();
        validate_text("skill description", &description)?;
        Ok(Self {
            id,
            name,
            description,
            origin,
            activation,
            current_revision_id,
        })
    }

    #[must_use]
    pub const fn id(&self) -> SkillId {
        self.id
    }

    #[must_use]
    pub fn name(&self) -> &SkillName {
        &self.name
    }

    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    #[must_use]
    pub fn origin(&self) -> &SkillOrigin {
        &self.origin
    }

    #[must_use]
    pub const fn activation(&self) -> ActivationMode {
        self.activation
    }

    #[must_use]
    pub const fn current_revision_id(&self) -> RevisionId {
        self.current_revision_id
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SkillRevision {
    id: RevisionId,
    skill_id: SkillId,
    content_hash: ContentHash,
    source: RevisionSource,
    supersedes: Option<RevisionId>,
}

impl SkillRevision {
    #[must_use]
    pub fn new(
        id: RevisionId,
        skill_id: SkillId,
        content_hash: ContentHash,
        source: RevisionSource,
        supersedes: Option<RevisionId>,
    ) -> Self {
        Self {
            id,
            skill_id,
            content_hash,
            source,
            supersedes,
        }
    }

    #[must_use]
    pub const fn id(&self) -> RevisionId {
        self.id
    }

    #[must_use]
    pub const fn skill_id(&self) -> SkillId {
        self.skill_id
    }

    #[must_use]
    pub fn content_hash(&self) -> &ContentHash {
        &self.content_hash
    }

    #[must_use]
    pub fn source(&self) -> &RevisionSource {
        &self.source
    }

    #[must_use]
    pub const fn supersedes(&self) -> Option<RevisionId> {
        self.supersedes
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Profile {
    id: ProfileId,
    name: ProfileName,
    skill_ids: Vec<SkillId>,
}

impl Profile {
    pub fn new(id: ProfileId, name: ProfileName, skill_ids: Vec<SkillId>) -> CoreResult<Self> {
        let unique_skill_ids: HashSet<_> = skill_ids.iter().copied().collect();
        if unique_skill_ids.len() != skill_ids.len() {
            return Err(DomainError::InvalidValue {
                field: "profile skills",
                reason: "a profile cannot include the same skill more than once".to_owned(),
            });
        }

        Ok(Self {
            id,
            name,
            skill_ids,
        })
    }

    #[must_use]
    pub const fn id(&self) -> ProfileId {
        self.id
    }

    #[must_use]
    pub fn name(&self) -> &ProfileName {
        &self.name
    }

    #[must_use]
    pub fn skill_ids(&self) -> &[SkillId] {
        &self.skill_ids
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum EvidenceKind {
    UserFeedback,
    UsageSignal,
    Evaluation,
    Review,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Evidence {
    kind: EvidenceKind,
    summary: String,
    reference: Option<String>,
}

impl Evidence {
    pub fn new(
        kind: EvidenceKind,
        summary: impl Into<String>,
        reference: Option<String>,
    ) -> CoreResult<Self> {
        let summary = summary.into();
        validate_text("evidence summary", &summary)?;
        if let Some(reference) = &reference {
            validate_text("evidence reference", reference)?;
        }

        Ok(Self {
            kind,
            summary,
            reference,
        })
    }

    #[must_use]
    pub const fn kind(&self) -> EvidenceKind {
        self.kind
    }

    #[must_use]
    pub fn summary(&self) -> &str {
        &self.summary
    }

    #[must_use]
    pub fn reference(&self) -> Option<&str> {
        self.reference.as_deref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProposedChange {
    CreateSkill { name: SkillName },
    UpdateSkill { skill_id: SkillId },
    DeleteSkill { skill_id: SkillId },
    UpdateProfile { profile_id: ProfileId },
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ProposalAuthor {
    Human(HumanActorId),
    Machine(MachineActorId),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ProposalStatus {
    Draft,
    PendingApproval,
    Approved { approved_by: HumanActorId },
    Rejected { rejected_by: HumanActorId },
    Applied { applied_by: HumanActorId },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApprovedProposal {
    proposal_id: ProposalId,
    approved_by: HumanActorId,
}

impl ApprovedProposal {
    #[must_use]
    pub const fn proposal_id(&self) -> ProposalId {
        self.proposal_id
    }

    #[must_use]
    pub const fn approved_by(&self) -> HumanActorId {
        self.approved_by
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChangeProposal {
    id: ProposalId,
    author: ProposalAuthor,
    evidence: Vec<Evidence>,
    changes: Vec<ProposedChange>,
    status: ProposalStatus,
}

impl ChangeProposal {
    pub fn new(
        id: ProposalId,
        author: ProposalAuthor,
        evidence: Vec<Evidence>,
        changes: Vec<ProposedChange>,
    ) -> CoreResult<Self> {
        if changes.is_empty() {
            return Err(DomainError::InvalidValue {
                field: "proposal changes",
                reason: "a proposal must contain at least one change".to_owned(),
            });
        }
        if matches!(author, ProposalAuthor::Machine(_)) && evidence.is_empty() {
            return Err(DomainError::InvalidValue {
                field: "proposal evidence",
                reason: "machine-authored proposals require evidence".to_owned(),
            });
        }

        Ok(Self {
            id,
            author,
            evidence,
            changes,
            status: ProposalStatus::Draft,
        })
    }

    #[must_use]
    pub const fn id(&self) -> ProposalId {
        self.id
    }

    #[must_use]
    pub const fn author(&self) -> ProposalAuthor {
        self.author
    }

    #[must_use]
    pub fn evidence(&self) -> &[Evidence] {
        &self.evidence
    }

    #[must_use]
    pub fn changes(&self) -> &[ProposedChange] {
        &self.changes
    }

    #[must_use]
    pub const fn status(&self) -> ProposalStatus {
        self.status
    }

    pub fn submit(&mut self) -> CoreResult<()> {
        if self.status != ProposalStatus::Draft {
            return Err(invalid_transition(self.status, "pending approval"));
        }
        self.status = ProposalStatus::PendingApproval;
        Ok(())
    }

    pub fn approve(&mut self, actor: HumanActorId) -> CoreResult<ApprovedProposal> {
        if self.status != ProposalStatus::PendingApproval {
            return Err(invalid_transition(self.status, "approved"));
        }
        self.status = ProposalStatus::Approved { approved_by: actor };
        Ok(ApprovedProposal {
            proposal_id: self.id,
            approved_by: actor,
        })
    }

    pub fn reject(&mut self, actor: HumanActorId) -> CoreResult<()> {
        if self.status != ProposalStatus::PendingApproval {
            return Err(invalid_transition(self.status, "rejected"));
        }
        self.status = ProposalStatus::Rejected { rejected_by: actor };
        Ok(())
    }

    pub fn apply(&mut self, approval: &ApprovedProposal, actor: HumanActorId) -> CoreResult<()> {
        if approval.proposal_id != self.id {
            return Err(DomainError::Authorization {
                action: "apply proposal",
                reason: "approval belongs to another proposal".to_owned(),
            });
        }

        match self.status {
            ProposalStatus::Approved { approved_by } if approved_by == approval.approved_by => {
                self.status = ProposalStatus::Applied { applied_by: actor };
                Ok(())
            }
            ProposalStatus::PendingApproval => Err(DomainError::Trust {
                operation: "apply proposal",
                reason: "an explicit human approval is required".to_owned(),
            }),
            status => Err(invalid_transition(status, "applied")),
        }
    }

    #[must_use]
    pub fn is_approved_by(&self, approval: &ApprovedProposal) -> bool {
        self.id == approval.proposal_id
            && self.status
                == (ProposalStatus::Approved {
                    approved_by: approval.approved_by,
                })
    }
}

fn validate_text(field: &'static str, value: &str) -> CoreResult<()> {
    if value.trim().is_empty() {
        return Err(DomainError::InvalidValue {
            field,
            reason: "must not be empty".to_owned(),
        });
    }
    if value.contains('\0') {
        return Err(DomainError::InvalidValue {
            field,
            reason: "must not contain a NUL byte".to_owned(),
        });
    }
    Ok(())
}

fn invalid_transition(status: ProposalStatus, target: &'static str) -> DomainError {
    DomainError::InvalidTransition {
        entity: "proposal",
        from: proposal_status_name(status),
        to: target,
    }
}

const fn proposal_status_name(status: ProposalStatus) -> &'static str {
    match status {
        ProposalStatus::Draft => "draft",
        ProposalStatus::PendingApproval => "pending approval",
        ProposalStatus::Approved { .. } => "approved",
        ProposalStatus::Rejected { .. } => "rejected",
        ProposalStatus::Applied { .. } => "applied",
    }
}
