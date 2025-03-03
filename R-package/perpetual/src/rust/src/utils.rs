use extendr_api::prelude::*;
use perpetual_rs::constraints::{Constraint, ConstraintMap};
use perpetual_rs::data::Matrix;
use perpetual_rs::utils::percentiles as crate_percentiles;
use std::collections::HashMap;

#[extendr]
pub fn int_map_to_constraint_map(int_map: HashMap<usize, i8>) -> Result<HashMap<usize, Constraint>, extendr_api::Error> {
    let mut constraints: HashMap<usize, Constraint> = HashMap::new();
    for (f, c) in int_map.iter() {
        let c_ = match c {
            -1 => Ok(Constraint::Negative),
            1 => Ok(Constraint::Positive),
            0 => Ok(Constraint::Unconstrained),
            _ => Err(extendr_api::Error::Other(format!(
                "Invalid monotone constraint for feature {}: {}",
                f, c
            ).into())),
        }?;
        constraints.insert(*f, c_);
    }
    Ok(constraints)
}

#[extendr]
pub fn print_matrix(x: Vec<f32>, rows: usize, cols: usize) -> Result<()> {
    let m = Matrix::new(&x, rows, cols);
    println!("{}", m);
    Ok(())
}

#[extendr]
pub fn percentiles(v: Vec<f64>, sample_weight: Vec<f64>, percentiles_vec: Vec<f64>) -> Result<Vec<f64>, extendr_api::Error> {
    let p = crate_percentiles(&v, &sample_weight, &percentiles_vec).map_err(|e| extendr_api::Error::Other(e.to_string().into()))?;
    Ok(p)
}
