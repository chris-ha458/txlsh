use pyo3::prelude::*;

mod consts;
mod helper;
pub use crate::helper::{BucketKind, ChecksumKind, Version};

mod error;
pub use error::TxLshError;

mod pearson;
use crate::pearson::pearson_h;

mod txlsh_mod;
pub use crate::txlsh_mod::{TxLsh, TxLshBuilder};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}
/// Pearson hash exposed for Python
#[pyfunction]
fn pearson_hash(salt: u8, ii: u8, jj: u8, kk: u8) -> PyResult<u8> {
    let h = pearson_h(salt, ii, jj, kk);
    Ok(h)
}

/// A Python module implemented in Rust.
#[pymodule]
fn txlsh(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(pearson_hash, m)?)?;
    Ok(())
}
