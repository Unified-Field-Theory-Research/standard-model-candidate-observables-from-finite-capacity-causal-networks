# Open Proof Obligations

## Closed Dependencies: SMC-001 Through SMC-006

`SMC-001` closes the upstream binding and claim-boundary contract through the
recorded Paper 7 final conditional particle-excitation observables
certificate. It does not prove any Paper 8 Standard-Model-candidate
observable theorem.

`SMC-002` closes the finite candidate sector-family catalog observable row.
It binds to `SMC-001`, enforces finite family-label, excitation-label,
charge-signature, gauge-signature, local support, local readout,
finite-capacity, and bounded-transfer witnesses, requires Paper 7
excitation-sector and transition/interaction row compatibility, inherits
Paper 6 charge/gauge signature support, and rejects observed particle
catalogs, physical Standard Model content, physical particle excitations,
external fields, continuum QFT, background Hilbert bundles, simulation-only
signals, fit shortcuts, physical promotion, and unified-field promotion.

`SMC-003` closes finite candidate interaction-family signatures. It binds to
`SMC-001` and `SMC-002`, enforces finite interaction-family, source/target
candidate-family, source/target excitation-label, charge-delta, gauge-delta,
local transition-neighborhood, local transition-readout, finite-capacity, and
bounded-transfer witnesses, requires Paper 7 excitation-sector and
transition/interaction row compatibility, inherits Paper 6 charge/gauge delta
signature support, and rejects physical scattering theory, Standard Model
Lagrangians, continuum gauge groups, asymptotic states, S-matrix structure,
external dynamics, observed particle catalogs, physical Standard Model
content, physical particle excitations, external fields, continuum QFT,
background Hilbert bundles, simulation-only signals, fit shortcuts, physical
promotion, and unified-field promotion.

`SMC-004` closes compatibility with the closed Paper 7 particle-excitation
observable package. It binds `SMC-002` and `SMC-003` to Paper 7 finite
excitation-sector rows, transition/interaction rows, matter/gauge
compatibility rows, conservation/coarse-graining rows, finite label/support
preservation, local readout preservation, finite-capacity compatibility,
bounded-transfer compatibility, and causal-cone/no-signaling preservation,
while rejecting observed particle catalogs, physical Standard Model content,
physical particle excitations, external fields, continuum QFT, background
Hilbert bundles, simulation-only signals, fit shortcuts, physical promotion,
and unified-field promotion.

`SMC-005` closes finite catalog conservation/continuity and intrinsic
coarse-graining stability. It binds to `SMC-001` through `SMC-004`, enforces
finite catalog continuity, finite interaction/transition conservation,
bounded catalog flux/transfer, intrinsic coarse-graining maps, non-growing
coarse family, interaction, local support, and transfer bounds, compatibility
of coarse catalog rows with `SMC-002`, coarse interaction rows with
`SMC-003`, coarse Paper 7 compatibility rows with `SMC-004`, and Paper 7
conservation/coarse-graining row compatibility, while rejecting external
conservation laws, continuum currents, continuum-limit oracles, observed
particle catalogs, physical Standard Model content, physical particle
excitations, external fields, continuum QFT, background Hilbert bundles,
simulation-only signals, fit shortcuts, physical promotion, and unified-field
promotion.

`SMC-006` closes Paper 7 regime consistency and upstream-bypass rejection. It
binds `SMC-001` through `SMC-005` to the recorded Paper 7 final conditional
certificate, frozen Paper 1 through Paper 7 commit-chain consistency, closed
Paper 1 through Paper 7 theorem flags, no upstream mutation, no Paper 7
bypass, no unapproved Paper 7 revision, and no unrecorded upstream revision,
while rejecting observed particle catalogs, physical Standard Model content,
physical particle excitations, external fields, continuum QFT, background
Hilbert bundles, simulation-only signals, fit shortcuts, physical promotion,
and unified-field promotion.

## Active Obligation

`SMC-007` is open.

Required row shape:

- closed `SMC-001` upstream binding;
- closed `SMC-002` finite candidate sector-family catalog observable;
- closed `SMC-003` finite candidate interaction-family signature;
- closed `SMC-004` particle-excitation compatibility;
- closed `SMC-005` catalog conservation/coarse-graining stability;
- closed `SMC-006` Paper 7 regime consistency;
- audit covers `SMC-001` through `SMC-006`;
- audit covers Paper 8 theorem docs, proof log, state, upstream manifest,
  Lean gate, Rust gate, and publication skeleton;
- Rust-only runtime/proof-gate artifacts;
- fail-closed audit certificate emitted;
- no observed particle catalog;
- no physical Standard Model content;
- no physical particle-excitation import;
- no external matter field;
- no external gauge field;
- no continuum QFT import;
- no background Hilbert bundle import;
- no simulation-only signal;
- no fit shortcut;
- no physical promotion;
- no unified-field promotion.

## Future Obligations

- `SMC-008`: final internal conditional certificate, if all prior obligations
  close.

Observed particle catalogs, physical Standard Model content, physical
particle excitations, physical matter fields, physical gauge fields, physical
quantum dynamics, continuum QFT, physical nature promotion, and unified-field
promotion remain out of scope and false.

## Closed SMC-002 Row Shape

- closed `SMC-001` upstream binding;
- finite candidate sector-family label bound;
- finite occupied candidate families;
- finite excitation-label support inherited from Paper 7;
- finite charge-signature support inherited from Paper 6;
- finite gauge-signature support inherited from Paper 6;
- finite local support/readout boundary;
- compatibility with finite local capacity;
- compatibility with bounded transfer;
- compatibility with Paper 7 excitation-sector rows;
- compatibility with Paper 7 transition/interaction rows;
- no observed particle catalog;
- no physical Standard Model content;
- no physical particle-excitation import;
- no external matter field;
- no external gauge field;
- no continuum QFT import;
- no background Hilbert bundle import;
- no simulation-only signal;
- no fit shortcut;
- no physical promotion;
- no unified-field promotion.

## Closed SMC-006 Row Shape

- closed `SMC-001` upstream binding;
- closed `SMC-002` finite candidate sector-family catalog observable;
- closed `SMC-003` finite candidate interaction-family signature;
- closed `SMC-004` particle-excitation compatibility;
- closed `SMC-005` catalog conservation/coarse-graining stability;
- recorded Paper 7 final conditional certificate consumed;
- recorded Paper 7 frozen commit equals `UPSTREAM-PAPERS.json`;
- recorded Paper 6 frozen commit equals `UPSTREAM-PAPERS.json`;
- recorded Paper 5 frozen commit equals `UPSTREAM-PAPERS.json`;
- recorded Paper 4 frozen commit equals `UPSTREAM-PAPERS.json`;
- recorded Paper 3 frozen commit equals `UPSTREAM-PAPERS.json`;
- recorded Paper 2 frozen commit equals `UPSTREAM-PAPERS.json`;
- recorded Paper 1 frozen commit equals `UPSTREAM-PAPERS.json`;
- no unapproved Paper 7 revision;
- no upstream repository mutation;
- no Paper 7 bypass;
- no observed particle catalog;
- no physical Standard Model content;
- no physical particle-excitation import;
- no external matter field;
- no external gauge field;
- no continuum QFT import;
- no background Hilbert bundle import;
- no simulation-only signal;
- no fit shortcut;
- no physical promotion;
- no unified-field promotion.

## Closed SMC-005 Row Shape

- closed `SMC-001` upstream binding;
- closed `SMC-002` finite candidate sector-family catalog observable;
- closed `SMC-003` finite candidate interaction-family signature;
- closed `SMC-004` particle-excitation compatibility;
- finite catalog continuity witness;
- finite interaction/transition conservation witness;
- bounded catalog flux or transfer witness;
- intrinsic coarse-graining map over finite candidate families;
- non-growing coarse candidate-family label bound;
- non-growing coarse interaction-family label bound;
- non-growing coarse local support bound;
- non-growing coarse transfer bound;
- coarse-grained catalog rows compatible with `SMC-002`;
- coarse-grained interaction rows compatible with `SMC-003`;
- coarse-grained Paper 7 compatibility rows compatible with `SMC-004`;
- no external conservation-law import;
- no continuum current import;
- no continuum-limit oracle;
- no observed particle catalog;
- no physical Standard Model content;
- no physical particle-excitation import;
- no external matter field;
- no external gauge field;
- no continuum QFT import;
- no background Hilbert bundle import;
- no simulation-only signal;
- no fit shortcut;
- no physical promotion;
- no unified-field promotion.

## Closed SMC-004 Row Shape

- closed `SMC-001` upstream binding;
- closed `SMC-002` finite candidate sector-family catalog observable;
- closed `SMC-003` finite candidate interaction-family signature;
- compatibility with Paper 7 finite excitation-sector observable rows;
- compatibility with Paper 7 finite transition/interaction rows;
- compatibility with Paper 7 matter/gauge compatibility rows;
- compatibility with Paper 7 conservation/coarse-graining stability rows;
- preservation of finite family and excitation-label support;
- preservation of finite charge/gauge signature support;
- preservation of local support and readout boundaries;
- preservation of finite local capacity and bounded transfer;
- preservation of causal-cone/no-signaling constraints;
- no observed particle catalog;
- no physical Standard Model content;
- no physical particle-excitation import;
- no external matter field;
- no external gauge field;
- no continuum QFT import;
- no background Hilbert bundle import;
- no simulation-only signal;
- no fit shortcut;
- no physical promotion;
- no unified-field promotion.

## Closed SMC-003 Row Shape

- closed `SMC-001` upstream binding;
- closed `SMC-002` finite candidate sector-family catalog observable;
- finite candidate interaction-family label bound;
- finite occupied candidate interaction families;
- finite source candidate-family label support;
- finite target candidate-family label support;
- finite source/target excitation-label support inherited from Paper 7;
- finite charge-delta signature support inherited from Paper 6/Paper 7 rows;
- finite gauge-delta signature support inherited from Paper 6/Paper 7 rows;
- finite local transition-neighborhood support;
- finite local transition readout boundary;
- compatibility with finite local capacity;
- compatibility with bounded transfer;
- compatibility with Paper 7 excitation-sector rows;
- compatibility with Paper 7 transition/interaction rows;
- no physical scattering theory import;
- no Standard Model Lagrangian import;
- no continuum gauge-group import;
- no asymptotic-state or S-matrix import;
- no external dynamics import;
- no observed particle catalog;
- no physical Standard Model content;
- no physical particle-excitation import;
- no external matter field;
- no external gauge field;
- no continuum QFT import;
- no background Hilbert bundle import;
- no simulation-only signal;
- no fit shortcut;
- no physical promotion;
- no unified-field promotion.
