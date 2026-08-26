use ai_skills_core::{
    AuditPort, EmbeddingPort, LibraryReadPort, LibraryWritePort, MaterializationPort, ReviewPort,
    SearchPort, SourceSyncPort,
};

#[test]
fn ports_are_object_safe_without_provider_types() {
    let _: Option<&dyn LibraryReadPort> = None;
    let _: Option<&dyn LibraryWritePort> = None;
    let _: Option<&dyn SearchPort> = None;
    let _: Option<&dyn SourceSyncPort> = None;
    let _: Option<&dyn EmbeddingPort> = None;
    let _: Option<&dyn ReviewPort> = None;
    let _: Option<&dyn MaterializationPort> = None;
    let _: Option<&dyn AuditPort> = None;
}
