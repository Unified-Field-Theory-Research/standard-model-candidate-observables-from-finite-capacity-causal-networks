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

## Active Next Obligation

`SMC-002`: define a finite intrinsic candidate sector-family catalog
observable without importing observed particle catalogs, physical Standard
Model content, physical particle excitations, continuum QFT, external Hilbert
bundles, simulation-only signals, fit shortcuts, physical quantum dynamics,
or unified-field promotion.
