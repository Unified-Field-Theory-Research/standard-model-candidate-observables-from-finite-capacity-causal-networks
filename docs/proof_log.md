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

## Active Next Obligation

`SMC-006`: bind the closed `SMC-001` through `SMC-005` rows to the recorded
Paper 7 final conditional certificate and frozen Paper 1 through Paper 7
commit chain while rejecting upstream bypass, upstream mutation, unapproved
Paper 7 revisions, observed particle catalogs, physical Standard Model
content, physical particle excitations, continuum QFT, external Hilbert
bundles, simulation-only signals, fit shortcuts, physical quantum dynamics,
or unified-field promotion.
