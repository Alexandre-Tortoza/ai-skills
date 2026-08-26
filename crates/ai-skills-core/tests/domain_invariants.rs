use std::collections::HashSet;

use ai_skills_core::{
    ActivationMode, ChangeProposal, ContentHash, DomainError, Evidence, EvidenceKind, HumanActorId,
    MachineActorId, Ownership, ProposalAuthor, ProposalId, ProposalStatus, ProposedChange,
    RevisionId, RevisionSource, SkillId, SkillName, SkillOrigin, SkillRevision, SourceId,
};

fn machine_proposal() -> ChangeProposal {
    ChangeProposal::new(
        ProposalId::new(),
        ProposalAuthor::Machine(MachineActorId::new()),
        vec![Evidence::new(EvidenceKind::Evaluation, "Regression fixture", None).unwrap()],
        vec![ProposedChange::UpdateSkill {
            skill_id: SkillId::new(),
        }],
    )
    .unwrap()
}

#[test]
fn machine_proposal_requires_evidence_and_human_approval_before_apply() {
    let missing_evidence = ChangeProposal::new(
        ProposalId::new(),
        ProposalAuthor::Machine(MachineActorId::new()),
        Vec::new(),
        vec![ProposedChange::UpdateSkill {
            skill_id: SkillId::new(),
        }],
    );
    assert!(matches!(
        missing_evidence,
        Err(DomainError::InvalidValue { .. })
    ));

    let mut proposal = machine_proposal();
    proposal.submit().unwrap();
    let mut unrelated_proposal = machine_proposal();
    unrelated_proposal.submit().unwrap();
    let unrelated_approval = unrelated_proposal.approve(HumanActorId::new()).unwrap();
    let direct_apply = proposal.apply(&unrelated_approval, HumanActorId::new());
    assert!(matches!(
        direct_apply,
        Err(DomainError::Authorization { .. })
    ));

    let approval = proposal.approve(HumanActorId::new()).unwrap();
    proposal.apply(&approval, HumanActorId::new()).unwrap();
    assert!(matches!(proposal.status(), ProposalStatus::Applied { .. }));
}

#[test]
fn third_party_revisions_preserve_source_provenance() {
    let source_id = SourceId::new();
    let origin = SkillOrigin::new(Ownership::ThirdParty, Some(source_id)).unwrap();
    assert_eq!(origin.source_id(), Some(source_id));
    assert!(SkillOrigin::new(Ownership::ThirdParty, None).is_err());

    let revision_source = RevisionSource::new(
        source_id,
        Some("a1b2c3d4".to_owned()),
        "skills/example/SKILL.md",
    )
    .unwrap();
    let revision = SkillRevision::new(
        RevisionId::new(),
        SkillId::new(),
        ContentHash::new("sha256:fixture").unwrap(),
        revision_source,
        None,
    );

    assert_eq!(revision.source().source_id(), source_id);
    assert_eq!(revision.source().upstream_revision(), Some("a1b2c3d4"));
}

#[test]
fn activation_modes_are_distinct_and_ids_are_hashable() {
    let modes = HashSet::from([
        ActivationMode::Pinned,
        ActivationMode::Active,
        ActivationMode::DiscoveryOnly,
    ]);
    assert_eq!(modes.len(), 3);

    let skill_id = SkillId::new();
    let ids = HashSet::from([skill_id, skill_id]);
    assert_eq!(ids.len(), 1);
    assert_eq!(
        SkillName::new(" "),
        Err(DomainError::InvalidValue {
            field: "skill name",
            reason: "must not be empty".to_owned(),
        })
    );
}
