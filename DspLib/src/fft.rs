use crate::DescreteSignal;
use crate::ComplexNumber;
use std::f64::consts::PI;

/// This structure holds vectors that hold values for calculating fourier transform.
/// Some of multiplication operations are repeateble, so to avoid operations duplication values
/// are being kept in vectors that are indexed such way all duplicated operations will have the
/// same index and will be stored only once.
struct FftParams {
    /// Number of samples in signal
    n: usize,
    /// s_nk is vector of multiplication results for s[n] * w_n[n*k]. It is simple way to implement
    /// butterfly diagram for fft.
    snk: Vec<Vec<ComplexNumber>>,
    /// Resolution of fourier transform
    resolution: f64,
}

pub fn fft(signal: &DescreteSignal, spectrum: &mut DescreteSignal) {
    let params = FftParams::new(signal);
    let mut x = 0.0;
    let scale_factor = 2.0/params.n as f64;
    for i in 0..params.n/2 {
        let mut y = ComplexNumber::default();

        for j in 0..params.n {
            y = &y + &params.snk[(i*j)%params.n][j];
        }
        if i == 0 {
            y = &y * 0.5;
        }
        spectrum.push(x, y.module() * scale_factor);
        x += params.resolution;
    }
}

impl FftParams {
    fn new(signal: &DescreteSignal) -> Self {
        let n = signal.len();
        let two_pi_by_n = 2.0 * PI/n as f64;
        let mut wn = vec![ComplexNumber::default(); n];
        let mut snk= vec![vec![ComplexNumber::default(); n]; n];
        let fs = 1.0/signal.get_sampling_period();

        for i in 0..n {
            wn[i] = ComplexNumber::new(f64::cos(two_pi_by_n*i as f64),
                                      -1.0*f64::sin(two_pi_by_n*i as f64));
            for k in 0..n {
                snk[i][k] = &wn[i] * signal[k].1;
            }
        }
        FftParams{n, snk, resolution: fs/n as f64}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_fft_from_vector(data: &[f64], sampling_period: f64) -> (DescreteSignal, FftParams) {
        let mut signal = vec![(0.0, 0.0); data.len()];
        for (i, d) in data.iter().enumerate() {
            signal[i] = (i as f64 * sampling_period, *d);
        }
        let signal = DescreteSignal::new_from_vec(signal);
        let fourier = FftParams::new(&signal);
        (signal, fourier)
    }

    #[test]
    fn selftest_for_create_fft_from_vector() {
        let data = vec![3.46, 4.32, 0.32, -12214.23];
        let period = 0.32;
        let expected_signal = DescreteSignal::new_from_vec(vec![(0.0, 3.46), (0.32, 4.32), (0.64, 0.32), (0.96, -12214.23)]);
        let expected_fourier = FftParams::new(&expected_signal);
        let (sig, fourier) = create_fft_from_vector(&data, period);
        assert_eq!(sig.get_data(), expected_signal.get_data());
        assert_eq!(fourier.n, expected_fourier.n);
        assert_eq!(fourier.snk, expected_fourier.snk);
    }

    #[test]
    fn frequency_resolution_shall_be_equal_to_sampling_rate_divided_by_number_of_samples() {
        let signal = DescreteSignal::new_from_vec(vec![(0.1, 5.0), (0.2, 4.0), (0.3, 3.0), (0.4, 2.0)]);
        let fft_object = FftParams::new(&signal);
        let expected_resolution = 1.0/(0.1*4.0);

        assert_eq!(fft_object.resolution, expected_resolution);
    }

    #[test]
    fn constructor_shall_generate_vectors_of_specific_length() {
        let signal = DescreteSignal::new_from_vec(vec![(1.0, 5.0), (2.0, 4.0), (3.0, 3.0), (4.0, 2.0), (5.0, 1.0)]);
        let fft_object = FftParams::new(&signal);
        assert_eq!(fft_object.n, signal.len());
        assert_eq!(fft_object.snk.len(), signal.len());
        assert_eq!(fft_object.snk[0].len(), signal.len());
    }

    #[test]
    fn constructor_shall_generate_snk_coefficients() {
        let data = [10.0, 10.5, 11.0];
        let (_, fft_object) = create_fft_from_vector(&data, 1.0);
        let expected_snk = [
            [ComplexNumber::new(data[0], 0.0),
             ComplexNumber::new(data[1], 0.0),
             ComplexNumber::new(data[2], 0.0)],
            [&ComplexNumber::new((2.0*PI/3.0).cos(), -1.0*(2.0*PI/3.0).sin()) * data[0],
             &ComplexNumber::new((2.0*PI/3.0).cos(), -1.0*(2.0*PI/3.0).sin()) * data[1],
             &ComplexNumber::new((2.0*PI/3.0).cos(), -1.0*(2.0*PI/3.0).sin()) * data[2]],
            [&ComplexNumber::new((4.0*PI/3.0).cos(), -1.0*(4.0*PI/3.0).sin()) * data[0],
             &ComplexNumber::new((4.0*PI/3.0).cos(), -1.0*(4.0*PI/3.0).sin()) * data[1],
             &ComplexNumber::new((4.0*PI/3.0).cos(), -1.0*(4.0*PI/3.0).sin()) * data[2]]
            ];
        assert_eq!(fft_object.snk, expected_snk);
    }

    #[test]
    fn check_fft_result() {
        let signal = DescreteSignal::new_from_vec(
            vec![(0.0, 2.0), (0.001, 1.0), (0.002, 0.0), (0.003, 1.0),
                      (0.004, 2.0), (0.005, 1.0), (0.006, 0.0), (0.007, 1.0)]);
        let expected_fft = DescreteSignal::new_from_vec(
            vec![(0.0, 1.0), (125.0, 8.326672684688674e-17),
                      (250.0, 1.0), (375.0, 8.326672684688674e-17)]);
        let mut calculated_fft = DescreteSignal::new();

        fft(&signal, &mut calculated_fft);
        assert_eq!(calculated_fft.get_data(), expected_fft.get_data());
    }
}