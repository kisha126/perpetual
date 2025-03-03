use extendr_api::prelude::*;

mod booster;
mod multi_output;
mod utils;

use booster::PerpetualBooster;
use multi_output::MultiOutputBooster;

// Macro to generate exports
#[extendr]
fn print_matrix(x: &[f32], rows: usize, cols: usize) -> Result<()> {
    let m = perpetual_rs::data::Matrix::new(x, rows, cols);
    rprintln!("{}", m);
    Ok(())
}

#[extendr]
fn percentiles(v: &[f64], sample_weight: &[f64], percentiles: &[f64]) -> Vec<f64> {
    perpetual_rs::utils::percentiles(v, sample_weight, percentiles)
}

// Macro to generate exports
extendr_module! {
    mod perpetual;
    fn print_matrix;
    fn percentiles;
    impl PerpetualBooster;
    impl MultiOutputBooster;
}
