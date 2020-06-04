// Bloom filter Python library written in Rust

extern crate farmhash;

use farmhash::FarmHasher;
use std::hash::{Hash, Hasher};

use pyo3::prelude::*;
use pyo3::wrap_pymodule;

#[pyclass]
struct BloomFilter {
    bv: Vec<bool>,
    hashes: u64,
}


#[inline]
fn num_of_bits_in_vec(capacity: usize, error_rate: f64) -> usize {
    (-1.0 * (((capacity as f64) * error_rate.ln()) /
        (1.0 / std::f64::consts::LN_2.powf(2.0)).ln())).ceil() as usize
}

#[inline]
fn num_of_hash_funcs(m: usize, capacity: usize) -> u64 {
    (std::f64::consts::LN_2 * ((m as f64) / (capacity as f64))).round().abs() as u64
}


#[pymethods]
impl BloomFilter {
    #[new]
    pub fn new(capacity: usize, error_rate: f64) -> PyResult<Self> {
        assert!((error_rate > 0.0 && error_rate < 1.0) && capacity > 0);

        let bv = vec![false; capacity];

        // https://en.wikipedia.org/wiki/Bloom_filter#Probability_of_false_positives
        let m = num_of_bits_in_vec(capacity, error_rate);

        // https://en.wikipedia.org/wiki/Bloom_filter#Optimal_number_of_hash_functions
        let k = num_of_hash_funcs(m, capacity);

        Ok(BloomFilter {
            bv,
            hashes: k,
        })
    }

    fn nth_hash(&self, x: &str, m: u64) -> usize {
        let mut hasher = FarmHasher::default();
        hasher.write(&m.to_be_bytes());
        x.hash(&mut hasher);
        ((hasher.finish()) % (self.bv.capacity() as u64)) as usize
    }

    pub fn insert(&mut self, value: &str) -> PyResult<bool> {
        for i in 0..self.hashes {
            let index = self.nth_hash(&value, i);
            self.bv[index] = true;
        }
        Ok(true)
    }

    pub fn has(&self, value: &str) -> PyResult<bool> {
        for i in 0..self.hashes {
            let index = self.nth_hash(&value, i);
            if !self.bv[index] {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

#[pymodule]
fn ubloom(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<BloomFilter>()?;
    Ok(())
}

#[pymodule]
fn ubloom_filter(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(ubloom))?;
    Ok(())
}
