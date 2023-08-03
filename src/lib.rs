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
pub use crate::txlsh_builders::{default_builder, full_builder,tx_lsh_builder};

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
        match builder.build() {
            Ok(result) => Ok(String::from(result.hash())),
            // python implementation doesn't really address error propagation.
            // not long enough, q3=0 all just becomes null
            Err(_) => Ok(String::from("TNULL"))
        }
}

#[pyfunction]
fn full_hash(binary_data: &PyBytes) -> PyResult<String> {
        let mut builder = full_builder();
        builder.update(binary_data.as_bytes());
        match builder.build() {
            Ok(result) => Ok(String::from(result.hash())),
            // python implementation doesn't really address error propagation.
            // not long enough, q3=0 all just becomes null
            Err(_) => Ok(String::from("TNULL"))
        }
}

#[pyfunction]
fn txlsh_hash(binary_data: &PyBytes) -> PyResult<String> {
        let mut builder = tx_lsh_builder();
        builder.update(binary_data.as_bytes());
        match builder.build() {
            Ok(result) => Ok(String::from(result.hash())),
            // python implementation doesn't really address error propagation.
            // not long enough, q3=0 all just becomes null
            Err(_) => Ok(String::from("TNULL"))
        }
}



/// A Python module implemented in Rust.
#[pymodule]
fn txlsh(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(pearson_hash, m)?)?;
    m.add_function(wrap_pyfunction!(default_hash, m)?)?;
    m.add_function(wrap_pyfunction!(full_hash, m)?)?;
    m.add_function(wrap_pyfunction!(txlsh_hash, m)?)?;
    Ok(())
}
