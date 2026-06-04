pub const PAPER1_FROZEN_COMMIT: &str = "3a9637c65f783ca35e77118f83560290f42f3085";
pub const PAPER2_FROZEN_COMMIT: &str = "053842ef5e1a50282df9d884266e87428ee07f60";
pub const PAPER3_FROZEN_COMMIT: &str = "6067360758108f799fa604855f5513545019492e";
pub const PAPER4_FROZEN_COMMIT: &str = "5a1ac95700786b697a0f25ddecb393fdeaaa166e";
pub const PAPER5_FROZEN_COMMIT: &str = "8db1a334b0c0ca934ccd3628add72c6e3f1ebfcb";
pub const PAPER6_FROZEN_COMMIT: &str = "20df751a0ceb2b4eb33a80dd15dd2795a1ea529a";
pub const PAPER7_FROZEN_COMMIT: &str = "4f52d9980f62977016ef5ee5da9e88a32dce70e5";
pub const PAPER7_FINAL_CERTIFICATE: &str =
    "paper7_final_conditional_particle_excitation_observables_theorem_closed";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Paper8UpstreamBinding {
    pub paper1_commit: &'static str,
    pub paper2_commit: &'static str,
    pub paper3_commit: &'static str,
    pub paper4_commit: &'static str,
    pub paper5_commit: &'static str,
    pub paper6_commit: &'static str,
    pub paper7_commit: &'static str,
    pub paper1_internal_conditional_closed: bool,
    pub paper2_geometry_closed: bool,
    pub paper3_curvature_closed: bool,
    pub paper4_dynamics_closed: bool,
    pub paper5_quantum_compatible_local_dynamics_closed: bool,
    pub paper6_matter_gauge_observables_closed: bool,
    pub paper7_particle_excitation_observables_closed: bool,
    pub paper7_final_certificate_consumed: bool,
    pub may_modify_upstreams: bool,
    pub physical_nature_claim: bool,
    pub continuum_qft_claim: bool,
    pub physical_matter_fields_claim: bool,
    pub physical_gauge_fields_claim: bool,
    pub physical_particle_excitation_claim: bool,
    pub observed_particle_catalog_claim: bool,
    pub physical_standard_model_claim: bool,
    pub physical_quantum_dynamics_claim: bool,
    pub standard_model_candidate_observable_recovery_claim: bool,
    pub unified_field_theory_claim: bool,
}

impl Paper8UpstreamBinding {
    pub fn canonical_smc001() -> Self {
        Self {
            paper1_commit: PAPER1_FROZEN_COMMIT,
            paper2_commit: PAPER2_FROZEN_COMMIT,
            paper3_commit: PAPER3_FROZEN_COMMIT,
            paper4_commit: PAPER4_FROZEN_COMMIT,
            paper5_commit: PAPER5_FROZEN_COMMIT,
            paper6_commit: PAPER6_FROZEN_COMMIT,
            paper7_commit: PAPER7_FROZEN_COMMIT,
            paper1_internal_conditional_closed: true,
            paper2_geometry_closed: true,
            paper3_curvature_closed: true,
            paper4_dynamics_closed: true,
            paper5_quantum_compatible_local_dynamics_closed: true,
            paper6_matter_gauge_observables_closed: true,
            paper7_particle_excitation_observables_closed: true,
            paper7_final_certificate_consumed: true,
            may_modify_upstreams: false,
            physical_nature_claim: false,
            continuum_qft_claim: false,
            physical_matter_fields_claim: false,
            physical_gauge_fields_claim: false,
            physical_particle_excitation_claim: false,
            observed_particle_catalog_claim: false,
            physical_standard_model_claim: false,
            physical_quantum_dynamics_claim: false,
            standard_model_candidate_observable_recovery_claim: false,
            unified_field_theory_claim: false,
        }
    }

    pub fn closes_smc001(&self) -> bool {
        self.paper1_commit == PAPER1_FROZEN_COMMIT
            && self.paper2_commit == PAPER2_FROZEN_COMMIT
            && self.paper3_commit == PAPER3_FROZEN_COMMIT
            && self.paper4_commit == PAPER4_FROZEN_COMMIT
            && self.paper5_commit == PAPER5_FROZEN_COMMIT
            && self.paper6_commit == PAPER6_FROZEN_COMMIT
            && self.paper7_commit == PAPER7_FROZEN_COMMIT
            && self.paper1_internal_conditional_closed
            && self.paper2_geometry_closed
            && self.paper3_curvature_closed
            && self.paper4_dynamics_closed
            && self.paper5_quantum_compatible_local_dynamics_closed
            && self.paper6_matter_gauge_observables_closed
            && self.paper7_particle_excitation_observables_closed
            && self.paper7_final_certificate_consumed
            && !self.may_modify_upstreams
            && !self.physical_nature_claim
            && !self.continuum_qft_claim
            && !self.physical_matter_fields_claim
            && !self.physical_gauge_fields_claim
            && !self.physical_particle_excitation_claim
            && !self.observed_particle_catalog_claim
            && !self.physical_standard_model_claim
            && !self.physical_quantum_dynamics_claim
            && !self.standard_model_candidate_observable_recovery_claim
            && !self.unified_field_theory_claim
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Paper8SkeletonCertificate {
    pub smc001_upstream_binding_closed: bool,
    pub smc002_finite_candidate_sector_family_catalog_closed: bool,
    pub smc003_finite_candidate_interaction_family_signature_closed: bool,
    pub smc004_particle_excitation_compatibility_closed: bool,
    pub smc005_catalog_conservation_coarse_graining_closed: bool,
    pub smc006_paper7_regime_consistency_closed: bool,
    pub smc007_no_hidden_observed_catalog_import_audit_closed: bool,
    pub smc008_final_conditional_certificate_closed: bool,
    pub paper8_theorem_closed: bool,
    pub physical_nature_claim: bool,
    pub observed_particle_catalog_claim: bool,
    pub physical_standard_model_claim: bool,
    pub physical_particle_excitation_claim: bool,
    pub physical_matter_fields_claim: bool,
    pub physical_gauge_fields_claim: bool,
    pub physical_quantum_dynamics_claim: bool,
    pub continuum_qft_claim: bool,
    pub unified_field_theory_claim: bool,
}

impl Paper8SkeletonCertificate {
    pub fn initial_smc001_only() -> Self {
        let binding = Paper8UpstreamBinding::canonical_smc001();
        Self {
            smc001_upstream_binding_closed: binding.closes_smc001(),
            smc002_finite_candidate_sector_family_catalog_closed: false,
            smc003_finite_candidate_interaction_family_signature_closed: false,
            smc004_particle_excitation_compatibility_closed: false,
            smc005_catalog_conservation_coarse_graining_closed: false,
            smc006_paper7_regime_consistency_closed: false,
            smc007_no_hidden_observed_catalog_import_audit_closed: false,
            smc008_final_conditional_certificate_closed: false,
            paper8_theorem_closed: false,
            physical_nature_claim: false,
            observed_particle_catalog_claim: false,
            physical_standard_model_claim: false,
            physical_particle_excitation_claim: false,
            physical_matter_fields_claim: false,
            physical_gauge_fields_claim: false,
            physical_quantum_dynamics_claim: false,
            continuum_qft_claim: false,
            unified_field_theory_claim: false,
        }
    }

    pub fn closes_paper8_theorem(&self) -> bool {
        self.smc001_upstream_binding_closed
            && self.smc002_finite_candidate_sector_family_catalog_closed
            && self.smc003_finite_candidate_interaction_family_signature_closed
            && self.smc004_particle_excitation_compatibility_closed
            && self.smc005_catalog_conservation_coarse_graining_closed
            && self.smc006_paper7_regime_consistency_closed
            && self.smc007_no_hidden_observed_catalog_import_audit_closed
            && self.smc008_final_conditional_certificate_closed
            && self.paper8_theorem_closed
            && !self.physical_nature_claim
            && !self.observed_particle_catalog_claim
            && !self.physical_standard_model_claim
            && !self.physical_particle_excitation_claim
            && !self.physical_matter_fields_claim
            && !self.physical_gauge_fields_claim
            && !self.physical_quantum_dynamics_claim
            && !self.continuum_qft_claim
            && !self.unified_field_theory_claim
    }
}

pub fn paper8_skeleton_marker() -> &'static str {
    "paper8-standard-model-candidate-observables-smc001-nonpromoting-skeleton"
}
