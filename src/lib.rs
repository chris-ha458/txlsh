use pyo3::{prelude::*, types::PyBytes};
mod consts;
mod helper;
pub use crate::helper::{BucketKind, ChecksumKind, Version};

mod error;
pub use error::TxLshError;

mod hash_funcs;
use crate::hash_funcs::pearson_h;

mod txlsh_mod;
pub use crate::txlsh_mod::{TxLsh, TxLshBuilder};

mod txlsh_builders;
pub use crate::txlsh_builders::default_builder;

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

/// Default Tlsh hash exposed for Python
#[pyfunction]
fn default_hash(binary_data: &PyBytes) -> PyResult<String> {
        let mut builder = default_builder();
        builder.update(binary_data.as_bytes());
        let default_tlsh = builder.build();
        match default_tlsh {
            Ok(result) => Ok(String::from(result.hash())),
            // python implementation doesn't really address error propagation.
            // not long enough, q3=0 all just becomes null
            Err(_) => Ok(String::from("TNULL"))
        }
}

/// A Python module implemented in Rust.
#[pymodule]
fn txlsh(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(pearson_hash, m)?)?;
    m.add_function(wrap_pyfunction!(default_hash, m)?)?;
    Ok(())
}
