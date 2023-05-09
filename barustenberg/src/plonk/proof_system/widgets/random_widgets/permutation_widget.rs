use crate::ecc::Field;

use crate::plonk::proof_system::proving_key::ProvingKey;
use crate::plonk::proof_system::widgets::random_widgets::random_widget::ProverRandomWidget;
use crate::proof_system::work_queue::WorkQueue;
use crate::transcript::{BarretenHasher, Transcript};
use std::sync::Arc;
pub struct VerifierPermutationWidget<
    H: BarretenHasher,
    F,
    Group,
    const NUM_ROOTS_CUT_OUT_OF_VANISHING_POLYNOMIAL: usize,
> where
    F: Field,
{
    transcript: Transcript<H>,
    // phantom: PhantomData<(Field, Group)>,
}

impl<H, F, Group, const NUM_ROOTS_CUT_OUT_OF_VANISHING_POLYNOMIAL: usize>
    VerifierPermutationWidget<H, F, Group, NUM_ROOTS_CUT_OUT_OF_VANISHING_POLYNOMIAL>
where
    H: BarretenHasher,
    F: Field,
{
    pub fn new() -> Self {
        Self {
            transcript: Transcript::default(),
            //phantom: PhantomData,
        }
    }

    pub fn compute_quotient_evaluation_contribution(
        key: &Arc<Transcript::Key>,
        alpha_base: Field,
        transcript: &Transcript<H>,
        quotient_numerator_eval: &mut Field,
        idpolys: bool,
    ) -> Field {
        // ...
    }

    pub fn append_scalar_multiplication_inputs(
        key: &Arc<Transcript::Key>,
        alpha_base: Field,
        transcript: &Transcript<H>,
    ) -> Field {
        // ...
    }
}

pub struct ProverPermutationWidget<
    Fr: Field,
    Hash: BarretenHasher,
    const PROGRAM_WIDTH: usize,
    const IDPOLYS: bool,
    const NUM_ROOTS_CUT_OUT_OF_VANISHING_POLYNOMIAL: usize,
> {
    prover_random_widget: dyn ProverRandomWidget,
}

impl<
        Fr: Field,
        Hash: BarretenHasher,
        const PROGRAM_WIDTH: usize,
        const IDPOLYS: bool,
        const NUM_ROOTS_CUT_OUT_OF_VANISHING_POLYNOMIAL: usize,
    >
    ProverPermutationWidget<
        Fr,
        Hash,
        PROGRAM_WIDTH,
        IDPOLYS,
        NUM_ROOTS_CUT_OUT_OF_VANISHING_POLYNOMIAL,
    >
{
    pub fn new(proving_key: Arc<ProvingKey<Fr>>) -> Self {
        Self {
            prover_random_widget: ProverRandomWidget::new(&proving_key),
        }
    }

    pub fn compute_round_commitments(
        &mut self,
        transcript: &mut Transcript<Hash>,
        round_number: usize,
        queue: &mut WorkQueue<Fr>,
    ) {
        // ...
    }

    pub fn compute_quotient_contribution(
        &self,
        alpha_base: Fr,
        transcript: &Transcript<Hash>,
    ) -> Fr {
        // ...
    }
}