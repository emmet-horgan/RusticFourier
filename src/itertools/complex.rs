use num_complex::Complex;
use num_traits::{Float, FloatConst, NumAssign};
use crate::traits::Iterable;

pub fn pad<I, F>(x: &I, padding: Complex<F>, len: usize) -> Result<I, ()> 
where
    // Bound F to float types
    F: Float + FloatConst + NumAssign + 'static,
    // Bound I to to an iterable collection of F
    for<'c> I: Iterable<OwnedItem = Complex<F>, Item<'c> = &'c Complex<F>>,
{
    let n = x.iter().len();
    if len < n {
        return Err(());
    }
    let num_padding = len - n;
    let pad_iter = std::iter::repeat(padding).take(num_padding);
    Ok(I::from_iter(x.iter().cloned().chain(pad_iter)))
}

pub fn pad_to_nearest_power_of_two<I, F>(x: &I, padding: Complex<F>) -> Result<I, ()>
where
    // Bound F to float types
    F: Float + FloatConst + NumAssign + 'static,
    // Bound I to to an iterable collection of F
    for<'c> I: Iterable<OwnedItem = Complex<F>, Item<'c> = &'c Complex<F>>,
    I: Clone,
{
    let n = x.iter().len();
    if n.is_power_of_two() {
        Ok(x.clone())
    } else {
        Ok(pad(x, padding, n.next_power_of_two())?) // Pad to the nearest power of 2
    }
}

pub fn zero_pad<I, F>(n: usize, x: &I) -> Result<I, ()> 
where
    // Bound F to float types
    F: Float + FloatConst + NumAssign + 'static,
    // Bound I to to an iterable collection of F
    for<'c> I: Iterable<OwnedItem = Complex<F>, Item<'c> = &'c Complex<F>>,
{
    Ok(pad(x, Complex::new(F::zero(), F::zero()), n)?)
}

pub fn zero_pad_to_nearest_power_of_two<I, F>(x: &I) -> Result<I, ()>
where
    // Bound F to float types
    F: Float + FloatConst + NumAssign + 'static,
    // Bound I to to an iterable collection of F
    for<'c> I: Iterable<OwnedItem = Complex<F>, Item<'c> = &'c Complex<F>>,
    I: Clone
{
    pad_to_nearest_power_of_two(x, Complex::new(F::zero(), F::zero()))
}