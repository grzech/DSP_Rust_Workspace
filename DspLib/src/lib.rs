mod gen;
mod fft;
mod complex_numbers;
mod descrete_signal;
pub use gen::{Generator, SineWave, RectangleWave};
pub use descrete_signal::DescreteSignal;
pub use fft::fft;
pub use complex_numbers::ComplexNumber;

#[cfg(test)]
mod tests {
    use super::*;

}
