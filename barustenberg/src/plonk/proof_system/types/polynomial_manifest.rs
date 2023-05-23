#[derive(Clone, Copy, Default)]
pub(crate) struct PolynomialManifest {}

impl PolynomialManifest {
    fn len(&self) -> usize {
        todo!()
    }
}

pub(crate) enum PolynomialSource {
    WITNESS,
    SELECTOR,
    PERMUTATION,
    OTHER,
}

pub(crate) enum EvaluationType {
    NonShifted,
    Shifted,
}

pub(crate) enum PolynomialIndex {
    Q1,
    Q2,
    Q3,
    Q4,
    Q5,
    QM,
    QC,
    QArithmetic,
    QFixedBase,
    QRange,
    QSort,
    QLogic,
    Table1,
    Table2,
    Table3,
    Table4,
    TableIndex,
    TableType,
    QElliptic,
    QAux,
    Sigma1,
    Sigma2,
    Sigma3,
    Sigma4,
    Id1,
    Id2,
    Id3,
    Id4,
    W1,
    W2,
    W3,
    W4,
    S,
    Z,
    ZLookup,
    LagrangeFirst,
    LagrangeLast,
    // SUBGROUP_GENERATOR,
    MaxNumPolynomials,
}
