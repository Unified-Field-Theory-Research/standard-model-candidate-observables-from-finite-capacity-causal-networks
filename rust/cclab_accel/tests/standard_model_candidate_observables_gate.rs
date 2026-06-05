use cclab_accel::{
    paper8_skeleton_marker, smc002_finite_candidate_sector_family_catalog_marker,
    FiniteCandidateSectorFamilyCatalogObservable, Paper8SkeletonCertificate, Paper8UpstreamBinding,
    PAPER1_FROZEN_COMMIT, PAPER2_FROZEN_COMMIT, PAPER3_FROZEN_COMMIT, PAPER4_FROZEN_COMMIT,
    PAPER5_FROZEN_COMMIT, PAPER6_FROZEN_COMMIT, PAPER7_FINAL_CERTIFICATE, PAPER7_FROZEN_COMMIT,
};
use std::fs;
use std::path::{Path, PathBuf};

fn project_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crate remains under rust/cclab_accel")
        .to_path_buf()
}

fn read(root: &Path, relative: &str) -> String {
    let path = root.join(relative);
    fs::read_to_string(&path).unwrap_or_else(|err| panic!("can read {}: {err}", path.display()))
}

fn assert_contains(text: &str, token: &str, artifact: &str) {
    assert!(
        text.contains(token),
        "{artifact} is missing required token {token:?}"
    );
}

fn collect_python_artifacts(path: &Path, out: &mut Vec<PathBuf>) {
    let file_name = path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("");
    if matches!(file_name, ".git" | "target" | ".lake") {
        return;
    }

    let metadata = fs::symlink_metadata(path).expect("can stat repository path");
    if metadata.is_dir() {
        if file_name == "__pycache__" {
            out.push(path.to_path_buf());
            return;
        }
        for entry in fs::read_dir(path).expect("can read repository directory") {
            let entry = entry.expect("can read repository directory entry");
            collect_python_artifacts(&entry.path(), out);
        }
        return;
    }

    if matches!(
        path.extension().and_then(|value| value.to_str()),
        Some("py" | "pyc" | "pyo")
    ) {
        out.push(path.to_path_buf());
    }
}

#[test]
fn paper8_skeleton_files_exist() {
    let root = project_root();
    for relative in [
        "AGENTS.md",
        "README.md",
        "UPSTREAM-PAPERS.json",
        "GPD/PROJECT.md",
        "GPD/ROADMAP.md",
        "GPD/STATE.md",
        "GPD/state.json",
        "docs/standard_model_candidate_observables_theorem.md",
        "docs/open_proof_obligations.md",
        "docs/proof_log.md",
        "GPD/formal/FiniteCapacity/StandardModelCandidateObservables.lean",
        "GPD/publication/standard-model-candidate-observables-theorem/manuscript/standard_model_candidate_observables_theorem.tex",
        "GPD/publication/standard-model-candidate-observables-theorem/manuscript/PAPER-CONFIG.json",
        "GPD/publication/standard-model-candidate-observables-theorem/manuscript/ARTIFACT-MANIFEST.json",
        "GPD/publication/standard-model-candidate-observables-theorem/manuscript/reproducibility-manifest.json",
    ] {
        assert!(
            root.join(relative).exists(),
            "missing required file {relative}"
        );
    }
}

#[test]
fn smc001_upstream_binding_consumes_closed_paper7_chain() {
    let binding = Paper8UpstreamBinding::canonical_smc001();

    assert_eq!(binding.paper1_commit, PAPER1_FROZEN_COMMIT);
    assert_eq!(binding.paper2_commit, PAPER2_FROZEN_COMMIT);
    assert_eq!(binding.paper3_commit, PAPER3_FROZEN_COMMIT);
    assert_eq!(binding.paper4_commit, PAPER4_FROZEN_COMMIT);
    assert_eq!(binding.paper5_commit, PAPER5_FROZEN_COMMIT);
    assert_eq!(binding.paper6_commit, PAPER6_FROZEN_COMMIT);
    assert_eq!(binding.paper7_commit, PAPER7_FROZEN_COMMIT);
    assert!(binding.paper7_final_certificate_consumed);
    assert!(binding.closes_smc001());

    let promoted_candidate_observable = Paper8UpstreamBinding {
        standard_model_candidate_observable_recovery_claim: true,
        ..binding
    };
    assert!(!promoted_candidate_observable.closes_smc001());

    let promoted_observed_catalog = Paper8UpstreamBinding {
        observed_particle_catalog_claim: true,
        ..binding
    };
    assert!(!promoted_observed_catalog.closes_smc001());

    let promoted_physical_standard_model = Paper8UpstreamBinding {
        physical_standard_model_claim: true,
        ..binding
    };
    assert!(!promoted_physical_standard_model.closes_smc001());

    let wrong_paper7_commit = Paper8UpstreamBinding {
        paper7_commit: "unapproved-paper7-revision",
        ..binding
    };
    assert!(!wrong_paper7_commit.closes_smc001());

    let mutable_upstream = Paper8UpstreamBinding {
        may_modify_upstreams: true,
        ..binding
    };
    assert!(!mutable_upstream.closes_smc001());
}

#[test]
fn initial_skeleton_does_not_close_paper8_theorem() {
    let certificate = Paper8SkeletonCertificate::initial_smc001_only();

    assert!(certificate.smc001_upstream_binding_closed);
    assert!(!certificate.smc002_finite_candidate_sector_family_catalog_closed);
    assert!(!certificate.smc008_final_conditional_certificate_closed);
    assert!(!certificate.paper8_theorem_closed);
    assert!(!certificate.closes_paper8_theorem());
    assert_eq!(
        paper8_skeleton_marker(),
        "paper8-standard-model-candidate-observables-smc001-nonpromoting-skeleton"
    );
}

#[test]
fn smc002_finite_candidate_sector_family_catalog_closes_only_catalog_rung() {
    let catalog = FiniteCandidateSectorFamilyCatalogObservable::canonical_smc002();

    assert!(catalog.smc001_upstream_binding_closed);
    assert!(catalog.closes_smc002());
    assert!(catalog.occupied_family_count <= catalog.family_label_bound);
    assert!(catalog.occupied_excitation_label_count <= catalog.excitation_label_bound);
    assert!(catalog.occupied_charge_signature_count <= catalog.charge_signature_label_bound);
    assert!(catalog.occupied_gauge_signature_count <= catalog.gauge_signature_label_bound);
    assert!(catalog.local_readout_boundary_size <= catalog.local_support_size);
    assert!(catalog.local_support_size <= catalog.finite_capacity_bound);
    assert!(catalog.bounded_transfer_bound <= catalog.finite_capacity_bound);
    assert!(catalog.paper7_excitation_sector_rows_compatible);
    assert!(catalog.paper7_transition_interaction_rows_compatible);
    assert!(catalog.paper6_charge_signature_support_inherited);
    assert!(catalog.paper6_gauge_signature_support_inherited);
    assert!(!catalog.observed_particle_catalog_import);
    assert!(!catalog.physical_standard_model_content_import);
    assert!(!catalog.physical_particle_excitation_import);
    assert!(!catalog.continuum_qft_import);
    assert_eq!(
        smc002_finite_candidate_sector_family_catalog_marker(),
        "smc002-finite-candidate-sector-family-catalog-observable-closed"
    );

    let certificate = Paper8SkeletonCertificate::with_smc002_catalog_closed();
    assert!(certificate.smc001_upstream_binding_closed);
    assert!(certificate.smc002_finite_candidate_sector_family_catalog_closed);
    assert!(!certificate.smc003_finite_candidate_interaction_family_signature_closed);
    assert!(!certificate.smc008_final_conditional_certificate_closed);
    assert!(!certificate.paper8_theorem_closed);
    assert!(!certificate.closes_paper8_theorem());
}

#[test]
fn smc002_catalog_fails_closed_on_nonfinite_or_imported_structure() {
    let catalog = FiniteCandidateSectorFamilyCatalogObservable::canonical_smc002();

    let zero_family_bound = FiniteCandidateSectorFamilyCatalogObservable {
        family_label_bound: 0,
        ..catalog
    };
    assert!(!zero_family_bound.closes_smc002());

    let family_count_exceeds_bound = FiniteCandidateSectorFamilyCatalogObservable {
        occupied_family_count: catalog.family_label_bound + 1,
        ..catalog
    };
    assert!(!family_count_exceeds_bound.closes_smc002());

    let readout_exceeds_support = FiniteCandidateSectorFamilyCatalogObservable {
        local_readout_boundary_size: catalog.local_support_size + 1,
        ..catalog
    };
    assert!(!readout_exceeds_support.closes_smc002());

    let missing_paper7_excitation_rows = FiniteCandidateSectorFamilyCatalogObservable {
        paper7_excitation_sector_rows_compatible: false,
        ..catalog
    };
    assert!(!missing_paper7_excitation_rows.closes_smc002());

    let missing_paper7_transition_rows = FiniteCandidateSectorFamilyCatalogObservable {
        paper7_transition_interaction_rows_compatible: false,
        ..catalog
    };
    assert!(!missing_paper7_transition_rows.closes_smc002());

    let hidden_observed_catalog = FiniteCandidateSectorFamilyCatalogObservable {
        observed_particle_catalog_import: true,
        ..catalog
    };
    assert!(!hidden_observed_catalog.closes_smc002());

    let hidden_physical_standard_model = FiniteCandidateSectorFamilyCatalogObservable {
        physical_standard_model_content_import: true,
        ..catalog
    };
    assert!(!hidden_physical_standard_model.closes_smc002());

    let hidden_physical_particle = FiniteCandidateSectorFamilyCatalogObservable {
        physical_particle_excitation_import: true,
        ..catalog
    };
    assert!(!hidden_physical_particle.closes_smc002());

    let simulation_signal = FiniteCandidateSectorFamilyCatalogObservable {
        simulation_only_signal: true,
        ..catalog
    };
    assert!(!simulation_signal.closes_smc002());
}

#[test]
fn upstream_json_records_paper7_certificate_and_nonpromotion() {
    let root = project_root();
    let upstream = read(&root, "UPSTREAM-PAPERS.json");

    assert_contains(&upstream, PAPER1_FROZEN_COMMIT, "UPSTREAM-PAPERS.json");
    assert_contains(&upstream, PAPER2_FROZEN_COMMIT, "UPSTREAM-PAPERS.json");
    assert_contains(&upstream, PAPER3_FROZEN_COMMIT, "UPSTREAM-PAPERS.json");
    assert_contains(&upstream, PAPER4_FROZEN_COMMIT, "UPSTREAM-PAPERS.json");
    assert_contains(&upstream, PAPER5_FROZEN_COMMIT, "UPSTREAM-PAPERS.json");
    assert_contains(&upstream, PAPER6_FROZEN_COMMIT, "UPSTREAM-PAPERS.json");
    assert_contains(&upstream, PAPER7_FROZEN_COMMIT, "UPSTREAM-PAPERS.json");
    assert_contains(&upstream, PAPER7_FINAL_CERTIFICATE, "UPSTREAM-PAPERS.json");
    assert_contains(
        &upstream,
        "\"smc002_finite_candidate_sector_family_catalog_closed\": true",
        "UPSTREAM-PAPERS.json",
    );
    assert_contains(
        &upstream,
        "\"standard_model_candidate_observables_theorem_closed\": false",
        "UPSTREAM-PAPERS.json",
    );
    assert_contains(
        &upstream,
        "\"observed_particle_catalog_claim\": false",
        "UPSTREAM-PAPERS.json",
    );
    assert_contains(
        &upstream,
        "\"physical_standard_model_claim\": false",
        "UPSTREAM-PAPERS.json",
    );
    assert_contains(
        &upstream,
        "\"unified_field_theory_claim\": false",
        "UPSTREAM-PAPERS.json",
    );
}

#[test]
fn docs_record_smc002_closed_smc003_active_and_physical_claims_false() {
    let root = project_root();
    let theorem = read(
        &root,
        "docs/standard_model_candidate_observables_theorem.md",
    );
    let state = read(&root, "GPD/STATE.md");
    let readme = read(&root, "README.md");
    let proof_log = read(&root, "docs/proof_log.md");

    for artifact in [
        ("theorem doc", theorem.as_str()),
        ("state", state.as_str()),
        ("readme", readme.as_str()),
        ("proof log", proof_log.as_str()),
    ] {
        assert_contains(artifact.1, "SMC-001", artifact.0);
        assert_contains(artifact.1, "SMC-002", artifact.0);
        assert_contains(artifact.1, "SMC-003", artifact.0);
        assert_contains(artifact.1, "finite candidate sector-family", artifact.0);
        assert_contains(artifact.1, "observed particle", artifact.0);
        assert_contains(artifact.1, "physical Standard Model", artifact.0);
        assert_contains(artifact.1, "continuum quantum field theory", artifact.0);
        assert_contains(artifact.1, "unified", artifact.0);
    }
}

#[test]
fn repository_contains_no_python_artifacts() {
    let root = project_root();
    let mut python_artifacts = Vec::new();
    collect_python_artifacts(&root, &mut python_artifacts);

    assert!(
        python_artifacts.is_empty(),
        "Paper 8 scaffold must remain Rust-only; found Python artifacts: {python_artifacts:?}"
    );
}
