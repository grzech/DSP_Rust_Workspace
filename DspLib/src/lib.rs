mod gen;
mod fft;
mod complex_numbers;
mod descrete_signal;
mod filtering;
pub use gen::Generator;
pub use descrete_signal::DescreteSignal;
pub use fft::fft;
pub use complex_numbers::ComplexNumber;
pub use filtering::{get_averaging_fir_coefficients, fir_filter, get_low_pass_fir_coefficients};

#[cfg(test)]
mod tests {
    //use super::*;

}
