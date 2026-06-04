.PHONY: test-fast rust-test lean-build check

test-fast:
	cargo test --workspace --test standard_model_candidate_observables_gate

rust-test:
	cargo test --workspace

lean-build:
	cd GPD/formal && lake build

check: test-fast lean-build
