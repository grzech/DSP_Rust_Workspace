mod gen;
mod fft;
mod complex_numbers;
pub use gen::Generator;
pub use gen::DescreteSignal;
pub use fft::fft;
pub use complex_numbers::ComplexNumber;

#[cfg(test)]
mod tests {
    use super::*;

}
