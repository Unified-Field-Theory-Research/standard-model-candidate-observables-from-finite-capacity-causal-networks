# Standard-Model-Candidate Observables From Finite-Capacity Causal Networks

This repository is the isolated Paper 8 workspace for the finite-capacity
emergent-spacetime program.

Paper 8 target:

> Starting from the closed Paper 7 conditional particle-excitation
> observables theorem, formulate finite network-native
> Standard-Model-candidate catalog/interface observables while preserving
> finite capacity, locality, bounded transfer, causal-cone/no-signaling
> constraints, conservation/coarse-graining stability, and compatibility with
> the upstream geometry, curvature, dynamics, quantum-compatible local
> dynamics, matter/gauge observable, and particle-excitation observable
> packages.

This repo is intentionally separate from the upstream theorem repositories:

- Paper 1/core repo: `https://github.com/Unified-Field-Theory-Research/finite-capacity-causal-geometry`
- Paper 2 geometry repo: `https://github.com/Unified-Field-Theory-Research/higher-dimensional-geometry-recovery`
- Paper 3 curvature repo: `https://github.com/Unified-Field-Theory-Research/curvature-from-finite-capacity-causal-networks`
- Paper 4 dynamics repo: `https://github.com/Unified-Field-Theory-Research/gravitational-dynamics-from-finite-capacity-causal-networks`
- Paper 5 quantum-compatible local dynamics repo: `https://github.com/Unified-Field-Theory-Research/quantum-compatible-local-dynamics-from-finite-capacity-causal-networks`
- Paper 6 matter/gauge observables repo: `https://github.com/Unified-Field-Theory-Research/matter-gauge-observables-from-finite-capacity-causal-networks`
- Paper 7 particle-excitation observables repo: `https://github.com/Unified-Field-Theory-Research/particle-excitation-observables-from-finite-capacity-causal-networks`
- Expected local sibling checkouts: `../finite-capacity-causal-geometry`,
  `../higher-dimensional-geometry-recovery`,
  `../curvature-from-finite-capacity-causal-networks`,
  `../gravitational-dynamics-from-finite-capacity-causal-networks`,
  `../quantum-compatible-local-dynamics-from-finite-capacity-causal-networks`,
  `../matter-gauge-observables-from-finite-capacity-causal-networks`, and
  `../particle-excitation-observables-from-finite-capacity-causal-networks`
- Upstream binding file: `UPSTREAM-PAPERS.json`

The current theorem package is non-promoting. `SMC-001` records the upstream
binding and claim boundary through the closed Paper 7 result. `SMC-002`
closes the finite candidate sector-family catalog observable row. `SMC-003`
closes the finite candidate interaction-family signature row. `SMC-004`
closes compatibility with the closed Paper 7 particle-excitation observable
rows. `SMC-005` closes catalog conservation and coarse-graining stability.
`SMC-006` closes Paper 7 regime consistency and upstream-bypass rejection.
`SMC-007` closes the no-hidden-observed-catalog and physical Standard Model
import audit. `SMC-008` closes the final internal conditional
Standard-Model-candidate observables certificate, and the internal conditional
Paper 8 theorem is closed under those finite hypotheses. This does not prove
observed particle catalog recovery, physical Standard Model content, physical
particle excitations, physical matter fields, physical gauge fields, physical
quantum dynamics, continuum quantum field theory, physical nature
realization, or a unified field theory.

Persistent boundary tokens: observed particle catalogs; physical Standard
Model content; physical particle excitations; physical matter fields;
physical gauge fields; physical quantum dynamics; continuum quantum field
theory; unified-field promotion.

## Open Source, Authorship, And Citation

Primary author: Abraham Greenman.

- [LICENSE](LICENSE): code/tooling under MIT; manuscripts, diagrams,
  documentation, and review packets under CC BY 4.0 unless a file states
  otherwise.
- [CITATION.cff](CITATION.cff): citation metadata for this Paper 8 skeleton.
- [OPEN_SOURCE.md](OPEN_SOURCE.md): provenance, credit, and non-promotion
  framing.

## Layout

- `GPD/publication/standard-model-candidate-observables-theorem/manuscript/`:
  Paper 8 manuscript skeleton and publication metadata.
- `GPD/formal/FiniteCapacity/StandardModelCandidateObservables.lean`:
  Lean scaffold for Paper 8 gates and fail-closed blockers.
- `docs/standard_model_candidate_observables_theorem.md`:
  theorem target, closed `SMC-001` through `SMC-008` rungs, and residual
  promotion boundaries.
- `rust/cclab_accel/tests/standard_model_candidate_observables_gate.rs`:
  Rust guard that keeps the skeleton non-promoting and bound to Papers 1-7.

## Checks

```bash
make test-fast
make lean-build
```
