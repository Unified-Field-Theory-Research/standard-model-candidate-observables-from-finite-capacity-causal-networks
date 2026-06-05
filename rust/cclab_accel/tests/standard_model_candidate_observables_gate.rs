use cclab_accel::{
    paper8_skeleton_marker, smc002_finite_candidate_sector_family_catalog_marker,
    smc003_finite_candidate_interaction_family_signature_marker,
    smc004_particle_excitation_compatibility_marker,
    smc005_catalog_conservation_coarse_graining_marker, smc006_paper7_regime_consistency_marker,
    smc007_no_hidden_observed_catalog_import_audit_marker,
    CandidateCatalogConservationCoarseGrainingStability, CandidateParticleExcitationCompatibility,
    FiniteCandidateInteractionFamilySignature, FiniteCandidateSectorFamilyCatalogObservable,
    NoHiddenObservedCatalogImportAudit, Paper7RegimeConsistency, Paper8SkeletonCertificate,
    Paper8UpstreamBinding, PAPER1_FROZEN_COMMIT, PAPER2_FROZEN_COMMIT, PAPER3_FROZEN_COMMIT,
    PAPER4_FROZEN_COMMIT, PAPER5_FROZEN_COMMIT, PAPER6_FROZEN_COMMIT, PAPER7_FINAL_CERTIFICATE,
    PAPER7_FROZEN_COMMIT,
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
fn smc003_finite_candidate_interaction_family_signature_closes_only_signature_rung() {
    let interaction = FiniteCandidateInteractionFamilySignature::canonical_smc003();

    assert!(interaction.smc001_upstream_binding_closed);
    assert!(interaction.smc002_finite_candidate_sector_family_catalog_closed);
    assert!(interaction.closes_smc003());
    assert!(
        interaction.occupied_interaction_family_count <= interaction.interaction_family_label_bound
    );
    assert!(interaction.source_candidate_family_label < interaction.candidate_family_label_bound);
    assert!(interaction.target_candidate_family_label < interaction.candidate_family_label_bound);
    assert!(interaction.source_excitation_label < interaction.excitation_label_bound);
    assert!(interaction.target_excitation_label < interaction.excitation_label_bound);
    assert!(
        interaction.occupied_charge_delta_signature_count
            <= interaction.charge_delta_signature_label_bound
    );
    assert!(
        interaction.occupied_gauge_delta_signature_count
            <= interaction.gauge_delta_signature_label_bound
    );
    assert!(
        interaction.local_transition_readout_boundary_size
            <= interaction.local_transition_neighborhood_size
    );
    assert!(interaction.local_transition_neighborhood_size <= interaction.finite_capacity_bound);
    assert!(interaction.bounded_transfer_bound <= interaction.finite_capacity_bound);
    assert!(interaction.paper7_excitation_sector_rows_compatible);
    assert!(interaction.paper7_transition_interaction_rows_compatible);
    assert!(interaction.paper6_charge_delta_signature_support_inherited);
    assert!(interaction.paper6_gauge_delta_signature_support_inherited);
    assert!(!interaction.physical_scattering_theory_import);
    assert!(!interaction.standard_model_lagrangian_import);
    assert!(!interaction.s_matrix_import);
    assert!(!interaction.physical_standard_model_content_import);
    assert_eq!(
        smc003_finite_candidate_interaction_family_signature_marker(),
        "smc003-finite-candidate-interaction-family-signature-closed"
    );

    let certificate = Paper8SkeletonCertificate::with_smc003_interaction_signature_closed();
    assert!(certificate.smc001_upstream_binding_closed);
    assert!(certificate.smc002_finite_candidate_sector_family_catalog_closed);
    assert!(certificate.smc003_finite_candidate_interaction_family_signature_closed);
    assert!(!certificate.smc004_particle_excitation_compatibility_closed);
    assert!(!certificate.smc008_final_conditional_certificate_closed);
    assert!(!certificate.paper8_theorem_closed);
    assert!(!certificate.closes_paper8_theorem());
}

#[test]
fn smc003_interaction_signature_fails_closed_on_nonfinite_or_imported_structure() {
    let interaction = FiniteCandidateInteractionFamilySignature::canonical_smc003();

    let missing_smc002 = FiniteCandidateInteractionFamilySignature {
        smc002_finite_candidate_sector_family_catalog_closed: false,
        ..interaction
    };
    assert!(!missing_smc002.closes_smc003());

    let zero_interaction_bound = FiniteCandidateInteractionFamilySignature {
        interaction_family_label_bound: 0,
        ..interaction
    };
    assert!(!zero_interaction_bound.closes_smc003());

    let source_family_out_of_range = FiniteCandidateInteractionFamilySignature {
        source_candidate_family_label: interaction.candidate_family_label_bound,
        ..interaction
    };
    assert!(!source_family_out_of_range.closes_smc003());

    let readout_exceeds_transition_neighborhood = FiniteCandidateInteractionFamilySignature {
        local_transition_readout_boundary_size: interaction.local_transition_neighborhood_size + 1,
        ..interaction
    };
    assert!(!readout_exceeds_transition_neighborhood.closes_smc003());

    let missing_paper7_transition_rows = FiniteCandidateInteractionFamilySignature {
        paper7_transition_interaction_rows_compatible: false,
        ..interaction
    };
    assert!(!missing_paper7_transition_rows.closes_smc003());

    let physical_scattering = FiniteCandidateInteractionFamilySignature {
        physical_scattering_theory_import: true,
        ..interaction
    };
    assert!(!physical_scattering.closes_smc003());

    let standard_model_lagrangian = FiniteCandidateInteractionFamilySignature {
        standard_model_lagrangian_import: true,
        ..interaction
    };
    assert!(!standard_model_lagrangian.closes_smc003());

    let s_matrix_import = FiniteCandidateInteractionFamilySignature {
        s_matrix_import: true,
        ..interaction
    };
    assert!(!s_matrix_import.closes_smc003());

    let hidden_observed_catalog = FiniteCandidateInteractionFamilySignature {
        observed_particle_catalog_import: true,
        ..interaction
    };
    assert!(!hidden_observed_catalog.closes_smc003());
}

#[test]
fn smc004_particle_excitation_compatibility_closes_only_compatibility_rung() {
    let compatibility = CandidateParticleExcitationCompatibility::canonical_smc004();

    assert!(compatibility.smc001_upstream_binding_closed);
    assert!(compatibility.smc002_finite_candidate_sector_family_catalog_closed);
    assert!(compatibility.smc003_finite_candidate_interaction_family_signature_closed);
    assert!(compatibility.paper7_particle_excitation_observables_closed);
    assert!(compatibility.closes_smc004());
    assert!(compatibility.paper7_finite_excitation_sector_rows_compatible);
    assert!(compatibility.paper7_transition_interaction_rows_compatible);
    assert!(compatibility.paper7_matter_gauge_compatibility_rows_compatible);
    assert!(compatibility.paper7_conservation_coarse_graining_rows_compatible);
    assert!(compatibility.candidate_catalog_preserves_excitation_label_support);
    assert!(compatibility.interaction_signatures_preserve_transition_support);
    assert!(compatibility.charge_gauge_signature_support_preserved);
    assert!(compatibility.local_support_readout_boundaries_preserved);
    assert!(compatibility.finite_capacity_compatible);
    assert!(compatibility.bounded_transfer_compatible);
    assert!(compatibility.causal_cone_no_signaling_preserved);
    assert!(!compatibility.observed_particle_catalog_import);
    assert!(!compatibility.physical_standard_model_content_import);
    assert!(!compatibility.physical_particle_excitation_import);
    assert_eq!(
        smc004_particle_excitation_compatibility_marker(),
        "smc004-particle-excitation-observable-compatibility-closed"
    );

    let certificate =
        Paper8SkeletonCertificate::with_smc004_particle_excitation_compatibility_closed();
    assert!(certificate.smc001_upstream_binding_closed);
    assert!(certificate.smc002_finite_candidate_sector_family_catalog_closed);
    assert!(certificate.smc003_finite_candidate_interaction_family_signature_closed);
    assert!(certificate.smc004_particle_excitation_compatibility_closed);
    assert!(!certificate.smc005_catalog_conservation_coarse_graining_closed);
    assert!(!certificate.smc008_final_conditional_certificate_closed);
    assert!(!certificate.paper8_theorem_closed);
    assert!(!certificate.closes_paper8_theorem());
}

#[test]
fn smc004_particle_excitation_compatibility_fails_closed_on_missing_rows_or_imports() {
    let compatibility = CandidateParticleExcitationCompatibility::canonical_smc004();

    let missing_smc003 = CandidateParticleExcitationCompatibility {
        smc003_finite_candidate_interaction_family_signature_closed: false,
        ..compatibility
    };
    assert!(!missing_smc003.closes_smc004());

    let missing_paper7_excitation_rows = CandidateParticleExcitationCompatibility {
        paper7_finite_excitation_sector_rows_compatible: false,
        ..compatibility
    };
    assert!(!missing_paper7_excitation_rows.closes_smc004());

    let missing_paper7_transition_rows = CandidateParticleExcitationCompatibility {
        paper7_transition_interaction_rows_compatible: false,
        ..compatibility
    };
    assert!(!missing_paper7_transition_rows.closes_smc004());

    let missing_conservation_rows = CandidateParticleExcitationCompatibility {
        paper7_conservation_coarse_graining_rows_compatible: false,
        ..compatibility
    };
    assert!(!missing_conservation_rows.closes_smc004());

    let nonlocal_readout = CandidateParticleExcitationCompatibility {
        local_support_readout_boundaries_preserved: false,
        ..compatibility
    };
    assert!(!nonlocal_readout.closes_smc004());

    let hidden_observed_catalog = CandidateParticleExcitationCompatibility {
        observed_particle_catalog_import: true,
        ..compatibility
    };
    assert!(!hidden_observed_catalog.closes_smc004());

    let hidden_physical_standard_model = CandidateParticleExcitationCompatibility {
        physical_standard_model_content_import: true,
        ..compatibility
    };
    assert!(!hidden_physical_standard_model.closes_smc004());

    let hidden_physical_particle = CandidateParticleExcitationCompatibility {
        physical_particle_excitation_import: true,
        ..compatibility
    };
    assert!(!hidden_physical_particle.closes_smc004());
}

#[test]
fn smc005_catalog_conservation_coarse_graining_closes_only_stability_rung() {
    let stability = CandidateCatalogConservationCoarseGrainingStability::canonical_smc005();

    assert!(stability.smc001_upstream_binding_closed);
    assert!(stability.smc002_finite_candidate_sector_family_catalog_closed);
    assert!(stability.smc003_finite_candidate_interaction_family_signature_closed);
    assert!(stability.smc004_particle_excitation_compatibility_closed);
    assert!(stability.closes_smc005());
    assert!(stability.finite_catalog_continuity_witness);
    assert!(stability.finite_interaction_conservation_witness);
    assert!(stability.bounded_catalog_flux_transfer_witness);
    assert!(stability.intrinsic_coarse_graining_map);
    assert!(stability.coarse_family_label_bound <= stability.family_label_bound);
    assert!(
        stability.coarse_interaction_family_label_bound <= stability.interaction_family_label_bound
    );
    assert!(stability.coarse_local_support_bound <= stability.local_support_bound);
    assert!(stability.coarse_transfer_bound <= stability.transfer_bound);
    assert!(stability.coarse_catalog_rows_compatible_with_smc002);
    assert!(stability.coarse_interaction_rows_compatible_with_smc003);
    assert!(stability.coarse_paper7_compatibility_rows_compatible_with_smc004);
    assert!(stability.paper7_conservation_coarse_graining_rows_compatible);
    assert!(!stability.external_conservation_law_import);
    assert!(!stability.continuum_current_import);
    assert!(!stability.continuum_limit_oracle_import);
    assert_eq!(
        smc005_catalog_conservation_coarse_graining_marker(),
        "smc005-catalog-conservation-coarse-graining-stability-closed"
    );

    let certificate = Paper8SkeletonCertificate::with_smc005_conservation_coarse_graining_closed();
    assert!(certificate.smc001_upstream_binding_closed);
    assert!(certificate.smc002_finite_candidate_sector_family_catalog_closed);
    assert!(certificate.smc003_finite_candidate_interaction_family_signature_closed);
    assert!(certificate.smc004_particle_excitation_compatibility_closed);
    assert!(certificate.smc005_catalog_conservation_coarse_graining_closed);
    assert!(!certificate.smc006_paper7_regime_consistency_closed);
    assert!(!certificate.smc008_final_conditional_certificate_closed);
    assert!(!certificate.paper8_theorem_closed);
    assert!(!certificate.closes_paper8_theorem());
}

#[test]
fn smc005_catalog_conservation_coarse_graining_fails_closed_on_growth_or_imports() {
    let stability = CandidateCatalogConservationCoarseGrainingStability::canonical_smc005();

    let missing_smc004 = CandidateCatalogConservationCoarseGrainingStability {
        smc004_particle_excitation_compatibility_closed: false,
        ..stability
    };
    assert!(!missing_smc004.closes_smc005());

    let coarse_family_growth = CandidateCatalogConservationCoarseGrainingStability {
        coarse_family_label_bound: stability.family_label_bound + 1,
        ..stability
    };
    assert!(!coarse_family_growth.closes_smc005());

    let coarse_transfer_growth = CandidateCatalogConservationCoarseGrainingStability {
        coarse_transfer_bound: stability.transfer_bound + 1,
        ..stability
    };
    assert!(!coarse_transfer_growth.closes_smc005());

    let missing_coarse_catalog_compat = CandidateCatalogConservationCoarseGrainingStability {
        coarse_catalog_rows_compatible_with_smc002: false,
        ..stability
    };
    assert!(!missing_coarse_catalog_compat.closes_smc005());

    let external_conservation = CandidateCatalogConservationCoarseGrainingStability {
        external_conservation_law_import: true,
        ..stability
    };
    assert!(!external_conservation.closes_smc005());

    let continuum_current = CandidateCatalogConservationCoarseGrainingStability {
        continuum_current_import: true,
        ..stability
    };
    assert!(!continuum_current.closes_smc005());

    let continuum_limit_oracle = CandidateCatalogConservationCoarseGrainingStability {
        continuum_limit_oracle_import: true,
        ..stability
    };
    assert!(!continuum_limit_oracle.closes_smc005());

    let hidden_physical_standard_model = CandidateCatalogConservationCoarseGrainingStability {
        physical_standard_model_content_import: true,
        ..stability
    };
    assert!(!hidden_physical_standard_model.closes_smc005());
}

#[test]
fn smc006_paper7_regime_consistency_closes_only_regime_rung() {
    let regime = Paper7RegimeConsistency::canonical_smc006();

    assert!(regime.smc001_upstream_binding_closed);
    assert!(regime.smc002_finite_candidate_sector_family_catalog_closed);
    assert!(regime.smc003_finite_candidate_interaction_family_signature_closed);
    assert!(regime.smc004_particle_excitation_compatibility_closed);
    assert!(regime.smc005_catalog_conservation_coarse_graining_closed);
    assert_eq!(regime.paper1_commit, PAPER1_FROZEN_COMMIT);
    assert_eq!(regime.paper2_commit, PAPER2_FROZEN_COMMIT);
    assert_eq!(regime.paper3_commit, PAPER3_FROZEN_COMMIT);
    assert_eq!(regime.paper4_commit, PAPER4_FROZEN_COMMIT);
    assert_eq!(regime.paper5_commit, PAPER5_FROZEN_COMMIT);
    assert_eq!(regime.paper6_commit, PAPER6_FROZEN_COMMIT);
    assert_eq!(regime.paper7_commit, PAPER7_FROZEN_COMMIT);
    assert_eq!(regime.paper7_final_certificate, PAPER7_FINAL_CERTIFICATE);
    assert!(regime.paper7_final_certificate_consumed);
    assert!(regime.paper7_particle_excitation_observables_closed);
    assert!(regime.closes_smc006());
    assert!(!regime.upstream_mutation_attempt);
    assert!(!regime.paper7_bypass_attempt);
    assert!(!regime.unapproved_paper7_revision);
    assert_eq!(
        smc006_paper7_regime_consistency_marker(),
        "smc006-paper7-regime-consistency-no-upstream-bypass-closed"
    );

    let certificate = Paper8SkeletonCertificate::with_smc006_paper7_regime_consistency_closed();
    assert!(certificate.smc001_upstream_binding_closed);
    assert!(certificate.smc002_finite_candidate_sector_family_catalog_closed);
    assert!(certificate.smc003_finite_candidate_interaction_family_signature_closed);
    assert!(certificate.smc004_particle_excitation_compatibility_closed);
    assert!(certificate.smc005_catalog_conservation_coarse_graining_closed);
    assert!(certificate.smc006_paper7_regime_consistency_closed);
    assert!(!certificate.smc007_no_hidden_observed_catalog_import_audit_closed);
    assert!(!certificate.smc008_final_conditional_certificate_closed);
    assert!(!certificate.paper8_theorem_closed);
    assert!(!certificate.closes_paper8_theorem());
}

#[test]
fn smc006_paper7_regime_consistency_fails_closed_on_bypass_or_revision() {
    let regime = Paper7RegimeConsistency::canonical_smc006();

    let missing_smc005 = Paper7RegimeConsistency {
        smc005_catalog_conservation_coarse_graining_closed: false,
        ..regime
    };
    assert!(!missing_smc005.closes_smc006());

    let wrong_paper7_commit = Paper7RegimeConsistency {
        paper7_commit: "unapproved-paper7-revision",
        ..regime
    };
    assert!(!wrong_paper7_commit.closes_smc006());

    let missing_certificate = Paper7RegimeConsistency {
        paper7_final_certificate_consumed: false,
        ..regime
    };
    assert!(!missing_certificate.closes_smc006());

    let upstream_mutation = Paper7RegimeConsistency {
        upstream_mutation_attempt: true,
        ..regime
    };
    assert!(!upstream_mutation.closes_smc006());

    let paper7_bypass = Paper7RegimeConsistency {
        paper7_bypass_attempt: true,
        ..regime
    };
    assert!(!paper7_bypass.closes_smc006());

    let unapproved_revision = Paper7RegimeConsistency {
        unapproved_paper7_revision: true,
        ..regime
    };
    assert!(!unapproved_revision.closes_smc006());

    let hidden_observed_catalog = Paper7RegimeConsistency {
        observed_particle_catalog_import: true,
        ..regime
    };
    assert!(!hidden_observed_catalog.closes_smc006());
}

#[test]
fn smc007_no_hidden_import_audit_closes_only_audit_rung() {
    let audit = NoHiddenObservedCatalogImportAudit::canonical_smc007();

    assert!(audit.smc001_upstream_binding_closed);
    assert!(audit.smc002_finite_candidate_sector_family_catalog_closed);
    assert!(audit.smc003_finite_candidate_interaction_family_signature_closed);
    assert!(audit.smc004_particle_excitation_compatibility_closed);
    assert!(audit.smc005_catalog_conservation_coarse_graining_closed);
    assert!(audit.smc006_paper7_regime_consistency_closed);
    assert!(audit.audited_smc_rung_count >= audit.required_smc_rung_count);
    assert!(audit.theorem_docs_audited);
    assert!(audit.proof_log_audited);
    assert!(audit.state_files_audited);
    assert!(audit.upstream_manifest_audited);
    assert!(audit.lean_gate_audited);
    assert!(audit.rust_gate_audited);
    assert!(audit.publication_skeleton_audited);
    assert!(audit.rust_only_runtime_verified);
    assert!(audit.fail_closed_audit_certificate_emitted);
    assert!(audit.closes_smc007());
    assert!(!audit.observed_particle_catalog_import);
    assert!(!audit.physical_standard_model_content_import);
    assert!(!audit.physical_particle_excitation_import);
    assert!(!audit.continuum_qft_import);
    assert_eq!(
        smc007_no_hidden_observed_catalog_import_audit_marker(),
        "smc007-no-hidden-observed-catalog-import-audit-closed"
    );

    let certificate = Paper8SkeletonCertificate::with_smc007_no_hidden_import_audit_closed();
    assert!(certificate.smc001_upstream_binding_closed);
    assert!(certificate.smc002_finite_candidate_sector_family_catalog_closed);
    assert!(certificate.smc003_finite_candidate_interaction_family_signature_closed);
    assert!(certificate.smc004_particle_excitation_compatibility_closed);
    assert!(certificate.smc005_catalog_conservation_coarse_graining_closed);
    assert!(certificate.smc006_paper7_regime_consistency_closed);
    assert!(certificate.smc007_no_hidden_observed_catalog_import_audit_closed);
    assert!(!certificate.smc008_final_conditional_certificate_closed);
    assert!(!certificate.paper8_theorem_closed);
    assert!(!certificate.closes_paper8_theorem());
}

#[test]
fn smc007_no_hidden_import_audit_fails_closed_on_missing_coverage_or_imports() {
    let audit = NoHiddenObservedCatalogImportAudit::canonical_smc007();

    let missing_smc006 = NoHiddenObservedCatalogImportAudit {
        smc006_paper7_regime_consistency_closed: false,
        ..audit
    };
    assert!(!missing_smc006.closes_smc007());

    let too_few_rungs = NoHiddenObservedCatalogImportAudit {
        audited_smc_rung_count: audit.required_smc_rung_count - 1,
        ..audit
    };
    assert!(!too_few_rungs.closes_smc007());

    let missing_rust_gate = NoHiddenObservedCatalogImportAudit {
        rust_gate_audited: false,
        ..audit
    };
    assert!(!missing_rust_gate.closes_smc007());

    let missing_fail_closed_certificate = NoHiddenObservedCatalogImportAudit {
        fail_closed_audit_certificate_emitted: false,
        ..audit
    };
    assert!(!missing_fail_closed_certificate.closes_smc007());

    let hidden_observed_catalog = NoHiddenObservedCatalogImportAudit {
        observed_particle_catalog_import: true,
        ..audit
    };
    assert!(!hidden_observed_catalog.closes_smc007());

    let hidden_physical_standard_model = NoHiddenObservedCatalogImportAudit {
        physical_standard_model_content_import: true,
        ..audit
    };
    assert!(!hidden_physical_standard_model.closes_smc007());

    let simulation_signal = NoHiddenObservedCatalogImportAudit {
        simulation_only_signal: true,
        ..audit
    };
    assert!(!simulation_signal.closes_smc007());
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
        "\"smc003_finite_candidate_interaction_family_signature_closed\": true",
        "UPSTREAM-PAPERS.json",
    );
    assert_contains(
        &upstream,
        "\"smc004_particle_excitation_compatibility_closed\": true",
        "UPSTREAM-PAPERS.json",
    );
    assert_contains(
        &upstream,
        "\"smc005_catalog_conservation_coarse_graining_closed\": true",
        "UPSTREAM-PAPERS.json",
    );
    assert_contains(
        &upstream,
        "\"smc006_paper7_regime_consistency_closed\": true",
        "UPSTREAM-PAPERS.json",
    );
    assert_contains(
        &upstream,
        "\"smc007_no_hidden_observed_catalog_import_audit_closed\": true",
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
fn docs_record_smc007_closed_smc008_active_and_physical_claims_false() {
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
        assert_contains(artifact.1, "SMC-004", artifact.0);
        assert_contains(artifact.1, "SMC-005", artifact.0);
        assert_contains(artifact.1, "SMC-006", artifact.0);
        assert_contains(artifact.1, "SMC-007", artifact.0);
        assert_contains(artifact.1, "SMC-008", artifact.0);
        assert_contains(artifact.1, "finite candidate sector-family", artifact.0);
        assert_contains(
            artifact.1,
            "finite candidate interaction-family",
            artifact.0,
        );
        assert_contains(artifact.1, "Paper 7", artifact.0);
        assert_contains(artifact.1, "coarse-graining", artifact.0);
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
