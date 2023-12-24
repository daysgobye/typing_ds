use pyo3::prelude::*;
use std::thread;

mod file_saver;
mod key_monitor; // Import the Keymonitor module from keyMonitor.rs
mod letter_manager;

#[pyfunction]
fn main_loop() {
    thread::spawn(|| {
        let mut keymonitor: key_monitor::Keymonitor = key_monitor::Keymonitor::new();
        loop {
            keymonitor.main_loop_fn()
        }
    });
}
/// Formats the sum of two numbers as string.

/// A Python module implemented in Rust.
#[pymodule]
fn typing_ds(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(main_loop, m)?)?;
    Ok(())
}
