use ark_bn254::{Fr, G1Affine};
use ark_ec::AffineRepr;
use ark_ff::{FftField, Field};
use typenum::U1;

use crate::{
    plonk::proof_system::{
        proving_key::ProvingKey,
        types::polynomial_manifest::{EvaluationType, PolynomialIndex},
    },
    transcript::BarretenHasher,
};

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    marker::PhantomData,
    rc::Rc,
};

use super::{
    containers::{ChallengeArray, CoefficientArray, CHALLENGE_BIT_ALPHA},
    getters::BaseGetter,
    transition_widget::{KernelBase, TransitionWidget, TransitionWidgetBase},
};

#[derive(Debug)]
pub(crate) struct ArithmeticKernel<H: BarretenHasher, F: Field + FftField, G: AffineRepr> {
    _marker: PhantomData<(H, F, G)>,
}

impl<H: BarretenHasher, F, G> ArithmeticKernel<H, F, G>
where
    F: Field + FftField,
    G: AffineRepr,
{
    // TODO see all these U1s they should be a named variable but they are not :( inherent associate type problem
    pub(crate) const QUOTIENT_REQUIRED_CHALLENGES: u8 = CHALLENGE_BIT_ALPHA as u8;
    pub(crate) const UPDATE_REQUIRED_CHALLENGES: u8 = CHALLENGE_BIT_ALPHA as u8;

    pub(crate) fn get_required_polynomial_ids() -> &'static HashSet<PolynomialIndex> {
        // ...
        todo!("ArithmeticKernel::get_required_polynomial_ids")
    }
}

impl<H: BarretenHasher, F: Field + FftField, G: AffineRepr> KernelBase
    for ArithmeticKernel<H, F, G>
{
    type Field = F;
    type Group = G;
    type Hasher = H;
    type NumIndependentRelations = U1;

    #[inline]
    fn get_required_polynomial_ids() -> HashSet<PolynomialIndex> {
        HashSet::from([
            PolynomialIndex::Q1,
            PolynomialIndex::Q2,
            PolynomialIndex::Q3,
            PolynomialIndex::QM,
            PolynomialIndex::QC,
            PolynomialIndex::W1,
            PolynomialIndex::W2,
            PolynomialIndex::W3,
        ])
    }

    fn quotient_required_challenges() -> u8 {
        todo!()
    }

    fn update_required_challenges() -> u8 {
        todo!()
    }

    #[inline]
    fn compute_linear_terms<Get: BaseGetter<Fr = Self::Field>>(
        polynomials: &Get::PC,
        _challenges: &ChallengeArray<F, U1>,
        linear_terms: &mut CoefficientArray<F>,
        index: Option<usize>,
    ) {
        let index = index.unwrap_or_default();

        let w_1 = Get::get_value(
            polynomials,
            EvaluationType::NonShifted,
            PolynomialIndex::W1,
            Some(index),
        );
        let w_2 = Get::get_value(
            polynomials,
            EvaluationType::NonShifted,
            PolynomialIndex::W2,
            Some(index),
        );
        let w_3 = Get::get_value(
            polynomials,
            EvaluationType::NonShifted,
            PolynomialIndex::W3,
            Some(index),
        );
        linear_terms[0.into()] = w_1 * w_2;
        linear_terms[1.into()] = w_1;
        linear_terms[2.into()] = w_2;
        linear_terms[3.into()] = w_3;
    }

    fn sum_linear_terms<Get: BaseGetter>(
        _polynomials: &Get::PC,
        _challenges: &ChallengeArray<F, U1>,
        _linear_terms: &CoefficientArray<F>,
        _index: usize,
    ) -> F {
        /*
                /**
         * @brief Scale and sum the linear terms for the final equation.
         *
         * @details Multiplies the linear terms by selector values and scale the whole sum by alpha before returning
         *
         * @param polynomials Container with polynomials or their simulation
         * @param challenges A structure with various challenges
         * @param linear_terms Precomuputed linear terms to be scaled and summed
         * @param i The index at which selector/witness values are sampled
         * @return FieldExt Scaled sum of values
         */
        inline static FieldExt sum_linear_terms(PolyContainer& polynomials,
                                             const challenge_array& challenges,
                                             coefficient_array& linear_terms,
                                             const size_t i = 0)
        {
            const FieldExt& alpha = challenges.alpha_powers[0];
            const FieldExt& q_1 =
                Getters::template get_value<EvaluationType::NON_SHIFTED, PolynomialIndex::Q_1>(polynomials, i);
            const FieldExt& q_2 =
                Getters::template get_value<EvaluationType::NON_SHIFTED, PolynomialIndex::Q_2>(polynomials, i);
            const FieldExt& q_3 =
                Getters::template get_value<EvaluationType::NON_SHIFTED, PolynomialIndex::Q_3>(polynomials, i);
            const FieldExt& q_m =
                Getters::template get_value<EvaluationType::NON_SHIFTED, PolynomialIndex::Q_M>(polynomials, i);
            const FieldExt& q_c =
                Getters::template get_value<EvaluationType::NON_SHIFTED, PolynomialIndex::Q_C>(polynomials, i);

            FieldExt result = linear_terms[0] * q_m;
            result += (linear_terms[1] * q_1);
            result += (linear_terms[2] * q_2);
            result += (linear_terms[3] * q_3);
            result += q_c;
            result *= alpha;
            return result;
        }
             */
        todo!()
    }

    /// Not being used in arithmetic_widget because there are none
    fn compute_non_linear_terms<Get: BaseGetter<Fr = Self::Field>>(
        _polynomials: &Get::PC,
        _challenges: &ChallengeArray<F, U1>,
        _quotient_term: &mut F,
        _index: usize,
    ) {
        unimplemented!(
            "ArithmeticKernel::compute_non_linear_terms- there are no non-linear terms..."
        )
    }

    /// Compute the scaled values of openings
    ///
    /// # Arguments
    /// - `linear_terms` - The original computed linear terms of the product and wires
    /// - `scalars` - A map where we put the values
    /// - `challenges` - Challenges where we get the alpha
    fn update_kate_opening_scalars(
        linear_terms: &CoefficientArray<F>,
        scalars: &mut HashMap<String, F>,
        challenges: &ChallengeArray<F, U1>,
    ) {
        let alpha: F = challenges.alpha_powers[0];
        scalars.insert(
            "Q_M".to_string(),
            *scalars.get("Q_M").unwrap() + linear_terms[0.into()] * alpha,
        );
        scalars.insert(
            "Q_1".to_string(),
            *scalars.get("Q_1").unwrap() + linear_terms[1.into()] * alpha,
        );
        scalars.insert(
            "Q_2".to_string(),
            *scalars.get("Q_2").unwrap() + linear_terms[2.into()] * alpha,
        );
        scalars.insert(
            "Q_3".to_string(),
            *scalars.get("Q_3").unwrap() + linear_terms[3.into()] * alpha,
        );
        scalars.insert("Q_C".to_string(), *scalars.get("Q_C").unwrap() + alpha);
    }
}

#[derive(Debug)]
pub(crate) struct ProverArithmeticWidget<H: BarretenHasher>(
    TransitionWidget<H, Fr, G1Affine, U1, ArithmeticKernel<H, Fr, G1Affine>>,
);

impl<H: BarretenHasher> TransitionWidgetBase for ProverArithmeticWidget<H> {
    type Hasher = H;
    type Field = Fr;

    fn compute_quotient_contribution(
        &self,
        alpha_base: Self::Field,
        transcript: &crate::transcript::Transcript<Self::Hasher>,
        rng: &mut dyn rand::RngCore,
    ) -> Self::Field {
        self.0
            .compute_quotient_contribution(alpha_base, transcript, rng)
    }
}

impl<H: BarretenHasher> ProverArithmeticWidget<H> {
    pub(crate) fn new(key: Rc<RefCell<ProvingKey<Fr, G1Affine>>>) -> Self {
        Self(TransitionWidget::new(key))
    }
}
