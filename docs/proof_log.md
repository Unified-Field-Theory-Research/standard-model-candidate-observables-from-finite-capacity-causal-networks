# Proof Log

## SMC-001: Upstream Binding And Claim Boundary

Status: closed as a scaffold-level upstream binding and nonpromotion contract.

Artifacts:

- `AGENTS.md`
- `README.md`
- `UPSTREAM-PAPERS.json`
- `GPD/PROJECT.md`
- `GPD/ROADMAP.md`
- `GPD/STATE.md`
- `GPD/state.json`
- `docs/standard_model_candidate_observables_theorem.md`
- `docs/open_proof_obligations.md`
- `GPD/formal/FiniteCapacity/StandardModelCandidateObservables.lean`
- `rust/cclab_accel/src/lib.rs`
- `rust/cclab_accel/tests/standard_model_candidate_observables_gate.rs`

Rust anchors:

- `Paper8UpstreamBinding`
- `Paper8UpstreamBinding::canonical_smc001`
- `Paper8UpstreamBinding::closes_smc001`
- `Paper8SkeletonCertificate::initial_smc001_only`
- `paper8_skeleton_marker`

Lean anchors:

- `SMC001UpstreamBindingContract`
- `SMC001UpstreamBindingContract.closed`
- `smc001_upstream_binding_closed_from_fields`
- `smc001_missing_paper7_particle_excitation_binding_not_closed`
- `smc001_standard_model_candidate_recovery_promotion_not_closed`
- `smc001_observed_particle_catalog_promotion_not_closed`
- `smc001_physical_standard_model_promotion_not_closed`
- `smc001_unified_field_promotion_not_closed`
- `paper8_smc001_skeleton_does_not_close_standard_model_candidate_observables_theorem`

Verification:

- `make test-fast`
- `make lean-build`

Boundary:

`SMC-001` consumes the recorded Paper 7 final conditional
particle-excitation observables certificate and the recorded Paper 1 through
Paper 6 chain. It does not prove Standard-Model-candidate observable recovery,
observed particle catalog recovery, physical Standard Model content, physical
particle excitations, physical matter fields, physical gauge fields, physical
quantum dynamics, continuum quantum field theory, a physical nature-level
claim, or a unified field theory.

## SMC-002: Finite Candidate Sector-Family Catalog Observable

Status: closed as a finite candidate sector-family catalog observable row.

Artifacts:

- `README.md`
- `UPSTREAM-PAPERS.json`
- `GPD/PROJECT.md`
- `GPD/ROADMAP.md`
- `GPD/STATE.md`
- `GPD/state.json`
- `docs/standard_model_candidate_observables_theorem.md`
- `docs/open_proof_obligations.md`
- `docs/proof_log.md`
- `GPD/formal/FiniteCapacity/StandardModelCandidateObservables.lean`
- `rust/cclab_accel/src/lib.rs`
- `rust/cclab_accel/tests/standard_model_candidate_observables_gate.rs`
- `GPD/publication/standard-model-candidate-observables-theorem/manuscript/standard_model_candidate_observables_theorem.tex`
- `GPD/publication/standard-model-candidate-observables-theorem/manuscript/PAPER-CONFIG.json`
- `GPD/publication/standard-model-candidate-observables-theorem/manuscript/ARTIFACT-MANIFEST.json`

Rust anchors:

- `FiniteCandidateSectorFamilyCatalogObservable`
- `FiniteCandidateSectorFamilyCatalogObservable::canonical_smc002`
- `FiniteCandidateSectorFamilyCatalogObservable::closes_smc002`
- `Paper8SkeletonCertificate::with_smc002_catalog_closed`
- `smc002_finite_candidate_sector_family_catalog_marker`

Lean anchors:

- `SMC002FiniteCandidateSectorFamilyCatalogObservableContract`
- `SMC002FiniteCandidateSectorFamilyCatalogObservableContract.closed`
- `smc002_finite_candidate_sector_family_catalog_closed_from_fields`
- `smc002_missing_finite_family_label_bound_not_closed`
- `smc002_missing_paper7_excitation_rows_not_closed`
- `smc002_observed_particle_catalog_import_not_closed`
- `smc002_physical_standard_model_content_import_not_closed`
- `smc002_canonical_finite_candidate_sector_family_catalog_closed`
- `paper8_smc002_catalog_does_not_close_standard_model_candidate_observables_theorem`

Verification:

- `make test-fast`
- `make lean-build`

Boundary:

`SMC-002` closes only finite candidate sector-family catalog observability:
finite family labels, occupied candidate families, excitation-label support
inherited from Paper 7, charge/gauge signature support inherited from Paper
6, finite local support/readout, finite-capacity compatibility,
bounded-transfer compatibility, and Paper 7 excitation-sector and
transition/interaction row compatibility. It does not close finite candidate
interaction-family signatures, Paper 7 compatibility as a whole,
conservation/coarse-graining stability, Paper 7 regime consistency, the
no-hidden-observed-catalog import audit, the final Paper 8 theorem, observed
particle catalogs, physical Standard Model content, physical particle
excitations, physical matter fields, physical gauge fields, physical quantum
dynamics, continuum QFT, physical nature promotion, or unified-field
promotion.

## SMC-003: Finite Candidate Interaction-Family Signature

Status: closed as finite candidate interaction-family signatures.

Artifacts:

- `README.md`
- `UPSTREAM-PAPERS.json`
- `GPD/PROJECT.md`
- `GPD/ROADMAP.md`
- `GPD/STATE.md`
- `GPD/state.json`
- `docs/standard_model_candidate_observables_theorem.md`
- `docs/open_proof_obligations.md`
- `docs/proof_log.md`
- `GPD/formal/FiniteCapacity/StandardModelCandidateObservables.lean`
- `rust/cclab_accel/src/lib.rs`
- `rust/cclab_accel/tests/standard_model_candidate_observables_gate.rs`
- `GPD/publication/standard-model-candidate-observables-theorem/manuscript/standard_model_candidate_observables_theorem.tex`
- `GPD/publication/standard-model-candidate-observables-theorem/manuscript/PAPER-CONFIG.json`

Rust anchors:

- `FiniteCandidateInteractionFamilySignature`
- `FiniteCandidateInteractionFamilySignature::canonical_smc003`
- `FiniteCandidateInteractionFamilySignature::closes_smc003`
- `Paper8SkeletonCertificate::with_smc003_interaction_signature_closed`
- `smc003_finite_candidate_interaction_family_signature_marker`

Lean anchors:

- `SMC003FiniteCandidateInteractionFamilySignatureContract`
- `SMC003FiniteCandidateInteractionFamilySignatureContract.closed`
- `smc003_finite_candidate_interaction_family_signature_closed_from_fields`
- `smc003_missing_smc002_catalog_not_closed`
- `smc003_physical_scattering_theory_import_not_closed`
- `smc003_standard_model_lagrangian_import_not_closed`
- `smc003_s_matrix_import_not_closed`
- `smc003_canonical_finite_candidate_interaction_family_signature_closed`
- `paper8_smc003_signature_does_not_close_standard_model_candidate_observables_theorem`

Verification:

- `make test-fast`
- `make lean-build`

Boundary:

`SMC-003` closes only finite candidate interaction-family signatures over
local transition neighborhoods: finite interaction-family labels, finite
source/target family labels, finite source/target excitation labels, finite
charge/gauge delta signature support, local transition support/readout,
finite-capacity compatibility, bounded-transfer compatibility, and Paper 7
excitation-sector and transition/interaction row compatibility. It does not
close Paper 7 compatibility as a whole, conservation/coarse-graining
stability, Paper 7 regime consistency, the no-hidden-observed-catalog import
audit, the final Paper 8 theorem, observed particle catalogs, physical
Standard Model content, physical particle excitations, physical matter
fields, physical gauge fields, physical quantum dynamics, continuum QFT,
physical nature promotion, or unified-field promotion.

## SMC-004: Particle-Excitation Observable Compatibility

Status: closed as compatibility with closed Paper 7 particle-excitation
observable rows.

Artifacts:

- `README.md`
- `UPSTREAM-PAPERS.json`
- `GPD/PROJECT.md`
- `GPD/ROADMAP.md`
- `GPD/STATE.md`
- `GPD/state.json`
- `docs/standard_model_candidate_observables_theorem.md`
- `docs/open_proof_obligations.md`
- `docs/proof_log.md`
- `GPD/formal/FiniteCapacity/StandardModelCandidateObservables.lean`
- `rust/cclab_accel/src/lib.rs`
- `rust/cclab_accel/tests/standard_model_candidate_observables_gate.rs`
- `GPD/publication/standard-model-candidate-observables-theorem/manuscript/standard_model_candidate_observables_theorem.tex`
- `GPD/publication/standard-model-candidate-observables-theorem/manuscript/PAPER-CONFIG.json`

Rust anchors:

- `CandidateParticleExcitationCompatibility`
- `CandidateParticleExcitationCompatibility::canonical_smc004`
- `CandidateParticleExcitationCompatibility::closes_smc004`
- `Paper8SkeletonCertificate::with_smc004_particle_excitation_compatibility_closed`
- `smc004_particle_excitation_compatibility_marker`

Lean anchors:

- `SMC004ParticleExcitationCompatibilityContract`
- `SMC004ParticleExcitationCompatibilityContract.closed`
- `smc004_particle_excitation_compatibility_closed_from_fields`
- `smc004_missing_smc003_signature_not_closed`
- `smc004_missing_paper7_transition_rows_not_closed`
- `smc004_observed_particle_catalog_import_not_closed`
- `smc004_canonical_particle_excitation_compatibility_closed`
- `paper8_smc004_compatibility_does_not_close_standard_model_candidate_observables_theorem`

Verification:

- `make test-fast`
- `make lean-build`

Boundary:

`SMC-004` closes only compatibility of the closed `SMC-002` catalog rows and
closed `SMC-003` interaction-family signatures with closed Paper 7 finite
excitation-sector, transition/interaction, matter/gauge compatibility, and
conservation/coarse-graining rows. It preserves finite label/support,
charge/gauge signature support, local readout boundaries, finite capacity,
bounded transfer, and causal-cone/no-signaling constraints. It does not close
catalog conservation/coarse-graining stability, Paper 7 regime consistency,
the no-hidden-observed-catalog import audit, the final Paper 8 theorem,
observed particle catalogs, physical Standard Model content, physical
particle excitations, physical matter fields, physical gauge fields, physical
quantum dynamics, continuum QFT, physical nature promotion, or unified-field
promotion.

## SMC-005: Catalog Conservation And Coarse-Graining Stability

Status: closed as finite catalog conservation/continuity and intrinsic
coarse-graining stability.

Artifacts:

- `README.md`
- `UPSTREAM-PAPERS.json`
- `GPD/PROJECT.md`
- `GPD/ROADMAP.md`
- `GPD/STATE.md`
- `GPD/state.json`
- `docs/standard_model_candidate_observables_theorem.md`
- `docs/open_proof_obligations.md`
- `docs/proof_log.md`
- `GPD/formal/FiniteCapacity/StandardModelCandidateObservables.lean`
- `rust/cclab_accel/src/lib.rs`
- `rust/cclab_accel/tests/standard_model_candidate_observables_gate.rs`
- `GPD/publication/standard-model-candidate-observables-theorem/manuscript/standard_model_candidate_observables_theorem.tex`
- `GPD/publication/standard-model-candidate-observables-theorem/manuscript/PAPER-CONFIG.json`

Rust anchors:

- `CandidateCatalogConservationCoarseGrainingStability`
- `CandidateCatalogConservationCoarseGrainingStability::canonical_smc005`
- `CandidateCatalogConservationCoarseGrainingStability::closes_smc005`
- `Paper8SkeletonCertificate::with_smc005_conservation_coarse_graining_closed`
- `smc005_catalog_conservation_coarse_graining_marker`

Lean anchors:

- `SMC005CatalogConservationCoarseGrainingStabilityContract`
- `SMC005CatalogConservationCoarseGrainingStabilityContract.closed`
- `smc005_catalog_conservation_coarse_graining_closed_from_fields`
- `smc005_missing_smc004_compatibility_not_closed`
- `smc005_coarse_family_growth_not_closed`
- `smc005_external_conservation_law_import_not_closed`
- `smc005_canonical_catalog_conservation_coarse_graining_closed`
- `paper8_smc005_conservation_does_not_close_standard_model_candidate_observables_theorem`

Verification:

- `make test-fast`
- `make lean-build`

Boundary:

`SMC-005` closes only finite catalog conservation/continuity and intrinsic
coarse-graining stability for the closed `SMC-002` through `SMC-004` rows. It
records finite catalog continuity, finite interaction conservation, bounded
catalog flux/transfer, intrinsic coarse-graining, non-growing coarse family,
interaction, local support, and transfer bounds, and coarse-row compatibility
with `SMC-002`, `SMC-003`, `SMC-004`, and Paper 7
conservation/coarse-graining rows. It does not close Paper 7 regime
consistency, the no-hidden-observed-catalog import audit, the final Paper 8
theorem, observed particle catalogs, physical Standard Model content,
physical particle excitations, physical matter fields, physical gauge fields,
physical quantum dynamics, continuum QFT, physical nature promotion, or
unified-field promotion.

## SMC-006: Paper 7 Regime Consistency

Status: closed as Paper 7 regime consistency and upstream-bypass rejection.

Artifacts:

- `README.md`
- `UPSTREAM-PAPERS.json`
- `GPD/PROJECT.md`
- `GPD/ROADMAP.md`
- `GPD/STATE.md`
- `GPD/state.json`
- `docs/standard_model_candidate_observables_theorem.md`
- `docs/open_proof_obligations.md`
- `docs/proof_log.md`
- `GPD/formal/FiniteCapacity/StandardModelCandidateObservables.lean`
- `rust/cclab_accel/src/lib.rs`
- `rust/cclab_accel/tests/standard_model_candidate_observables_gate.rs`
- `GPD/publication/standard-model-candidate-observables-theorem/manuscript/standard_model_candidate_observables_theorem.tex`
- `GPD/publication/standard-model-candidate-observables-theorem/manuscript/PAPER-CONFIG.json`

Rust anchors:

- `Paper7RegimeConsistency`
- `Paper7RegimeConsistency::canonical_smc006`
- `Paper7RegimeConsistency::closes_smc006`
- `Paper8SkeletonCertificate::with_smc006_paper7_regime_consistency_closed`
- `smc006_paper7_regime_consistency_marker`

Lean anchors:

- `SMC006Paper7RegimeConsistencyContract`
- `SMC006Paper7RegimeConsistencyContract.closed`
- `smc006_paper7_regime_consistency_closed_from_fields`
- `smc006_missing_smc005_stability_not_closed`
- `smc006_missing_recorded_paper7_commit_not_closed`
- `smc006_upstream_mutation_attempt_not_closed`
- `smc006_canonical_paper7_regime_consistency_closed`
- `paper8_smc006_regime_does_not_close_standard_model_candidate_observables_theorem`

Verification:

- `make test-fast`
- `make lean-build`

Boundary:

`SMC-006` closes only Paper 7 regime consistency and upstream-bypass
rejection for the closed `SMC-001` through `SMC-005` rows. It consumes the
recorded Paper 7 final conditional certificate, enforces the frozen Paper 1
through Paper 7 commit chain and closed upstream theorem flags, and rejects
upstream mutation, Paper 7 bypass, unapproved Paper 7 revisions, and
unrecorded upstream revisions. It does not close the no-hidden-observed
catalog import audit, the final Paper 8 theorem, observed particle catalogs,
physical Standard Model content, physical particle excitations, physical
matter fields, physical gauge fields, physical quantum dynamics, continuum
QFT, physical nature promotion, or unified-field promotion.

## SMC-007: No-Hidden-Observed-Catalog Import Audit

Status: closed as a fail-closed no-hidden-observed-catalog/physical Standard
Model import audit.

Artifacts:

- `README.md`
- `UPSTREAM-PAPERS.json`
- `GPD/PROJECT.md`
- `GPD/ROADMAP.md`
- `GPD/STATE.md`
- `GPD/state.json`
- `docs/standard_model_candidate_observables_theorem.md`
- `docs/open_proof_obligations.md`
- `docs/proof_log.md`
- `GPD/formal/FiniteCapacity/StandardModelCandidateObservables.lean`
- `rust/cclab_accel/src/lib.rs`
- `rust/cclab_accel/tests/standard_model_candidate_observables_gate.rs`
- `GPD/publication/standard-model-candidate-observables-theorem/manuscript/standard_model_candidate_observables_theorem.tex`
- `GPD/publication/standard-model-candidate-observables-theorem/manuscript/PAPER-CONFIG.json`

Rust anchors:

- `NoHiddenObservedCatalogImportAudit`
- `NoHiddenObservedCatalogImportAudit::canonical_smc007`
- `NoHiddenObservedCatalogImportAudit::closes_smc007`
- `Paper8SkeletonCertificate::with_smc007_no_hidden_import_audit_closed`
- `smc007_no_hidden_observed_catalog_import_audit_marker`

Lean anchors:

- `SMC007NoHiddenObservedCatalogImportAuditContract`
- `SMC007NoHiddenObservedCatalogImportAuditContract.closed`
- `smc007_no_hidden_observed_catalog_import_audit_closed_from_fields`
- `smc007_missing_smc006_regime_not_closed`
- `smc007_insufficient_audit_coverage_not_closed`
- `smc007_observed_particle_catalog_import_not_closed`
- `smc007_canonical_no_hidden_observed_catalog_import_audit_closed`
- `paper8_smc007_audit_does_not_close_standard_model_candidate_observables_theorem`

Verification:

- `make test-fast`
- `make lean-build`

Boundary:

`SMC-007` closes only the fail-closed no-hidden-observed-catalog/physical
Standard Model import audit. It covers `SMC-001` through `SMC-006`, theorem
docs, proof log, state files, upstream manifest, Lean gate, Rust gate,
publication skeleton, Rust-only runtime/proof-gate policy, and an emitted
fail-closed audit certificate. It does not close the final Paper 8 theorem,
observed particle catalogs, physical Standard Model content, physical
particle excitations, physical matter fields, physical gauge fields, physical
quantum dynamics, continuum QFT, physical nature promotion, or unified-field
promotion.

## SMC-008: Final Conditional Standard-Model-Candidate Observables Certificate

Status: closed.

Closed artifacts:

- [rust/cclab_accel/src/lib.rs](/Users/charlie/UnifiedFieldTheoryResearch/standard-model-candidate-observables-from-finite-capacity-causal-networks/rust/cclab_accel/src/lib.rs)
- [rust/cclab_accel/tests/standard_model_candidate_observables_gate.rs](/Users/charlie/UnifiedFieldTheoryResearch/standard-model-candidate-observables-from-finite-capacity-causal-networks/rust/cclab_accel/tests/standard_model_candidate_observables_gate.rs)
- [GPD/formal/FiniteCapacity/StandardModelCandidateObservables.lean](/Users/charlie/UnifiedFieldTheoryResearch/standard-model-candidate-observables-from-finite-capacity-causal-networks/GPD/formal/FiniteCapacity/StandardModelCandidateObservables.lean)
- [docs/standard_model_candidate_observables_theorem.md](/Users/charlie/UnifiedFieldTheoryResearch/standard-model-candidate-observables-from-finite-capacity-causal-networks/docs/standard_model_candidate_observables_theorem.md)
- [UPSTREAM-PAPERS.json](/Users/charlie/UnifiedFieldTheoryResearch/standard-model-candidate-observables-from-finite-capacity-causal-networks/UPSTREAM-PAPERS.json)

Rust anchors:

- `FinalStandardModelCandidateObservablesCertificate`
- `FinalStandardModelCandidateObservablesCertificate::canonical_smc008`
- `FinalStandardModelCandidateObservablesCertificate::closes_smc008`
- `Paper8SkeletonCertificate::final_smc008_closed`
- `smc008_final_conditional_certificate_marker`

Lean anchors:

- `SMC008FinalConditionalCertificateContract`
- `SMC008FinalConditionalCertificateContract.closed`
- `smc008_final_conditional_certificate_closed_from_fields`
- `smc008_missing_smc007_audit_not_closed`
- `smc008_observed_particle_catalog_claim_not_closed`
- `smc008_physical_standard_model_claim_not_closed`
- `smc008_canonical_final_conditional_certificate_closed`
- `paper8_smc008_final_conditional_certificate_closes_standard_model_candidate_observables_theorem`

Verification:

- `make test-fast`
- `make lean-build`

Boundary:

`SMC-008` closes the final internal conditional
Standard-Model-candidate observables certificate from closed `SMC-001`
through `SMC-007`. It emits finite candidate sector-family catalog
observables and finite candidate interaction-family signatures, preserves
Paper 7 particle-excitation compatibility, conservation/coarse-graining
stability, finite capacity, locality, bounded transfer, and causal-cone/no
signaling constraints, consumes Paper 7 regime consistency and the
no-hidden-observed-catalog/physical Standard Model import audit, and closes
only the internal conditional Paper 8 theorem. It does not close observed
particle catalogs, physical Standard Model content, physical particle
excitations, physical matter fields, physical gauge fields, physical quantum
dynamics, continuum QFT, physical nature promotion, simulation-only signals,
fit shortcuts, or unified-field promotion.
