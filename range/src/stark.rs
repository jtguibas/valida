use crate::RangeCheckerChip;

use p3_air::{Air, AirBuilder, BaseAir};
use p3_field::Field;
use p3_matrix::dense::RowMajorMatrix;

impl<AB, const MAX: u32> Air<AB> for RangeCheckerChip<MAX>
where
    AB: AirBuilder,
{
    fn eval(&self, _builder: &mut AB) {
        // TODO
    }
}

impl<F: Field, const MAX: u32> BaseAir<F> for RangeCheckerChip<MAX> {
    fn preprocessed_trace(&self) -> Option<RowMajorMatrix<F>> {
        let column = (0..MAX).map(F::from_canonical_u32).collect();
        Some(RowMajorMatrix::new_col(column))
    }
}
