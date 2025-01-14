use std::{cell::RefCell, collections::HashMap, rc::Rc};

use ark_bn254::G1Affine;
use ark_ff::{FftField, Field};

use crate::{
    numeric::bitop::Msb,
    plonk::composer::composer_base::ComposerType,
    polynomials::evaluation_domain::EvaluationDomain,
    srs::reference_string::{
        mem_reference_string::VerifierMemReferenceString, VerifierReferenceString,
    },
};

use super::types::PolynomialManifest;

#[derive(Debug)]
pub struct VerificationKey<Fr: Field + FftField> {
    pub(crate) composer_type: ComposerType,
    pub(crate) circuit_size: usize,
    log_circuit_size: usize,
    pub(crate) num_public_inputs: usize,
    pub(crate) domain: EvaluationDomain<Fr>,
    pub(crate) reference_string: Rc<RefCell<dyn VerifierReferenceString>>,
    pub(crate) commitments: HashMap<String, G1Affine>,
    pub(crate) polynomial_manifest: PolynomialManifest,
    /// This is a member variable so as to avoid recomputing it in the different places of the verifier algorithm.
    /// Note that recomputing would also have added constraints to the recursive verifier circuit.
    /// ʓ^n (ʓ being the 'evaluation challenge')
    pub(crate) z_pow_n: Fr,
    pub(crate) contains_recursive_proof: bool,
    pub(crate) recursive_proof_public_input_indices: Vec<u32>,
    pub(crate) program_width: usize,
}

impl<Fr: FftField + Field> Default for VerificationKey<Fr> {
    fn default() -> Self {
        VerificationKey {
            composer_type: Default::default(),
            circuit_size: 0,
            log_circuit_size: 0,
            num_public_inputs: Default::default(),
            domain: Default::default(),
            reference_string: Rc::new(RefCell::new(VerifierMemReferenceString::default())),
            commitments: Default::default(),
            polynomial_manifest: Default::default(),
            z_pow_n: Default::default(),
            contains_recursive_proof: false,
            recursive_proof_public_input_indices: vec![],
            program_width: 0,
        }
    }
}

impl<Fr: Field + FftField> VerificationKey<Fr> {
    pub(crate) fn new(
        circuit_size: usize,
        num_public_inputs: usize,
        reference_string: Rc<RefCell<dyn VerifierReferenceString>>,
        composer_type: ComposerType,
    ) -> Self {
        VerificationKey {
            circuit_size,
            log_circuit_size: circuit_size.get_msb(),
            num_public_inputs,
            reference_string,
            composer_type,
            domain: EvaluationDomain::new(circuit_size, None),
            commitments: HashMap::new(),
            polynomial_manifest: PolynomialManifest::new(),
            z_pow_n: Fr::zero(),
            contains_recursive_proof: false,
            recursive_proof_public_input_indices: Vec::new(),
            program_width: 0,
        }
    }
}
