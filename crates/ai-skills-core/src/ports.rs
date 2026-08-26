use async_trait::async_trait;

use crate::{
    ActivationMode, ApprovedProposal, ChangeProposal, CoreResult, Evidence, Profile, ProfileId,
    ProposalAuthor, ProposalId, RevisionId, Skill, SkillId, SkillRevision, Source, SourceId,
    TrustLevel,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SearchRequest {
    query: String,
    activation: Option<ActivationMode>,
    profile_id: Option<ProfileId>,
    tags: Vec<String>,
}

impl SearchRequest {
    pub fn new(query: impl Into<String>) -> CoreResult<Self> {
        let query = query.into();
        if query.trim().is_empty() {
            return Err(crate::DomainError::InvalidValue {
                field: "search query",
                reason: "must not be empty".to_owned(),
            });
        }
        Ok(Self {
            query,
            activation: None,
            profile_id: None,
            tags: Vec::new(),
        })
    }

    #[must_use]
    pub fn with_activation(mut self, activation: ActivationMode) -> Self {
        self.activation = Some(activation);
        self
    }

    #[must_use]
    pub fn with_profile(mut self, profile_id: ProfileId) -> Self {
        self.profile_id = Some(profile_id);
        self
    }

    #[must_use]
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    #[must_use]
    pub fn query(&self) -> &str {
        &self.query
    }

    #[must_use]
    pub const fn activation(&self) -> Option<ActivationMode> {
        self.activation
    }

    #[must_use]
    pub const fn profile_id(&self) -> Option<ProfileId> {
        self.profile_id
    }

    #[must_use]
    pub fn tags(&self) -> &[String] {
        &self.tags
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SearchHit {
    pub skill_id: SkillId,
    pub revision_id: RevisionId,
    pub score: f32,
    pub snippet: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceSyncReport {
    pub source_id: SourceId,
    pub discovered_skills: usize,
    pub updated_skills: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EmbeddingInput {
    pub revision_id: RevisionId,
    pub text: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EmbeddingVector {
    pub revision_id: RevisionId,
    pub values: Vec<f32>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReviewRequest {
    pub skill: Skill,
    pub revision: SkillRevision,
    pub evidence: Vec<Evidence>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MaterializationRequest {
    pub profile_id: ProfileId,
    pub revisions: Vec<RevisionId>,
    pub trust: TrustLevel,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MaterializationReport {
    pub profile_id: ProfileId,
    pub materialized_revisions: Vec<RevisionId>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AuditAction {
    ProposalSubmitted,
    ProposalApproved,
    ProposalApplied,
    SourceSynchronized,
    SkillsMaterialized,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AuditEvent {
    pub action: AuditAction,
    pub actor: ProposalAuthor,
    pub proposal_id: Option<ProposalId>,
}

/// Reads canonical skill and profile state without naming a storage implementation.
#[async_trait]
pub trait LibraryReadPort: Send + Sync {
    async fn skill(&self, id: SkillId) -> CoreResult<Option<Skill>>;
    async fn revision(&self, id: RevisionId) -> CoreResult<Option<SkillRevision>>;
    async fn profile(&self, id: ProfileId) -> CoreResult<Option<Profile>>;
}

/// Applies only proposals carrying a prior human approval.
#[async_trait]
pub trait LibraryWritePort: Send + Sync {
    async fn apply_approved_proposal(
        &self,
        proposal: &ChangeProposal,
        approval: &ApprovedProposal,
    ) -> CoreResult<Vec<SkillRevision>>;
}

#[async_trait]
pub trait SearchPort: Send + Sync {
    async fn search(&self, request: SearchRequest) -> CoreResult<Vec<SearchHit>>;
}

#[async_trait]
pub trait SourceSyncPort: Send + Sync {
    async fn synchronize(&self, source: &Source) -> CoreResult<SourceSyncReport>;
}

#[async_trait]
pub trait EmbeddingPort: Send + Sync {
    async fn embed_documents(
        &self,
        inputs: Vec<EmbeddingInput>,
    ) -> CoreResult<Vec<EmbeddingVector>>;
    async fn embed_query(&self, query: String) -> CoreResult<Vec<f32>>;
}

/// Returns proposals only; applying one belongs to `LibraryWritePort` after human approval.
#[async_trait]
pub trait ReviewPort: Send + Sync {
    async fn review(&self, request: ReviewRequest) -> CoreResult<Vec<ChangeProposal>>;
}

#[async_trait]
pub trait MaterializationPort: Send + Sync {
    async fn materialize(
        &self,
        request: MaterializationRequest,
    ) -> CoreResult<MaterializationReport>;
}

#[async_trait]
pub trait AuditPort: Send + Sync {
    async fn record(&self, event: AuditEvent) -> CoreResult<()>;
}
