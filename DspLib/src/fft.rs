use crate::DescreteSignal;
use crate::ComplexNumber;
use std::f64::consts::PI;

/// This structure holds vectors that hold values for calculating fourier transform.
/// Some of multiplication operations are repeateble, so to avoid operations duplication values
/// are being kept in vectors that are indexed such way all duplicated operations will have the
/// same index and will be stored only once.
struct Fft {
    /// Number of samples in signal
    n: usize,
    /// Constant part of sine argument - kept as field to perform this calculation only once
    two_pi_by_n: f64,
    /// w_N is a vector of cos(2*pi*k*n/N) - jsin(2*pi*k*n/N) - this vector should be limited to
    /// length of N. All entries that stisfy condition k1*n1 = k2*n2 (even if k1 != k2) shall not
    /// be calculated again.
    wn: Vec<ComplexNumber>,
    /// s_nk is vector of multiplication results for s[n] * w_n[n*k]. It is simple way to implement
    /// butterfly diagram for fft.
    snk: Vec<Vec<ComplexNumber>>,
}

pub fn fft(signal: &DescreteSignal, spectrum: &mut DescreteSignal) {
    let params = Fft::new(signal);

    for i in 0..params.n/60 {
        let x = i as f64;
        let mut y = ComplexNumber::default();
        for j in 0..params.n {
            y = &y + &params.snk[(i*j)%params.n][j];
        }
        spectrum.push(x, y.module());
    }
}

impl Fft {
    fn new(signal: &DescreteSignal) -> Self {
        let n = signal.len();
        let two_pi_by_n = 2.0 * PI/n as f64;
        let mut wn = vec![ComplexNumber::default(); n];
        let mut snk= vec![vec![ComplexNumber::default(); n]; n];
        let data = signal.get_data();

        for i in 0..n {
            wn[i] = ComplexNumber::new(f64::cos(two_pi_by_n*i as f64),
                                      -1.0*f64::sin(two_pi_by_n*i as f64));
            for k in 0..n {
                snk[i][k] = &wn[i] * &data[k].1;
            }
        }
        Fft{n, two_pi_by_n, wn, snk}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_fft_from_vector(data: &[f64], sampling_period: f64) -> (DescreteSignal, Fft) {
        let mut signal = vec![(0.0, 0.0); data.len()];
        for (i, d) in data.iter().enumerate() {
            signal[i] = (i as f64 * sampling_period, *d);
        }
        let signal = DescreteSignal::new_from_vec(signal);
        let fourier = Fft::new(&signal);
        (signal, fourier)
    }

    #[test]
    fn selftest_for_create_fft_from_vector() {
        let data = vec![3.46, 4.32, 0.32, -12214.23];
        let period = 0.32;
        let expected_signal = DescreteSignal::new_from_vec(vec![(0.0, 3.46), (0.32, 4.32), (0.64, 0.32), (0.96, -12214.23)]);
        let expected_fourier = Fft::new(&expected_signal);
        let (sig, fourier) = create_fft_from_vector(&data, period);
        assert_eq!(sig.get_data(), expected_signal.get_data());
        assert_eq!(fourier.n, expected_fourier.n);
        assert_eq!(fourier.wn, expected_fourier.wn);
        assert_eq!(fourier.snk, expected_fourier.snk);
    }

    #[test]
    fn constructor_shall_generate_vectors_of_specific_length() {
        let signal = DescreteSignal::new_from_vec(vec![(1.0, 5.0), (2.0, 4.0), (3.0, 3.0), (4.0, 2.0), (5.0, 1.0)]);
        let fft_object = Fft::new(&signal);
        assert_eq!(fft_object.n, signal.len());
        assert_eq!(fft_object.two_pi_by_n, 2.0*PI/signal.len() as f64);
        assert_eq!(fft_object.wn.len(), signal.len());
        assert_eq!(fft_object.snk.len(), signal.len());
        assert_eq!(fft_object.snk[0].len(), signal.len());
    }

    #[test]
    fn constructor_shall_generate_wn_coefficients() {
        let (_, fft_object) = create_fft_from_vector(&[0.0, 0.5, 1.0, 1.5, 2.0], 1.0);
        let expected_wn = [
            ComplexNumber::new(1.0, 0.0),
            ComplexNumber::new((2.0*PI/5.0).cos(), -1.0*(2.0*PI/5.0).sin()),
            ComplexNumber::new((4.0*PI/5.0).cos(), -1.0*(4.0*PI/5.0).sin()),
            ComplexNumber::new((6.0*PI/5.0).cos(), -1.0*(6.0*PI/5.0).sin()),
            ComplexNumber::new((8.0*PI/5.0).cos(), -1.0*(8.0*PI/5.0).sin())];
        assert_eq!(fft_object.wn, expected_wn);
    }

    #[test]
    fn constructor_shall_generate_snk_coefficients() {
        let data = [10.0, 10.5, 11.0];
        let (_, fft_object) = create_fft_from_vector(&data, 1.0);
        let expected_snk = [
            [ComplexNumber::new(data[0], 0.0),
             ComplexNumber::new(data[1], 0.0),
             ComplexNumber::new(data[2], 0.0)],
            [&ComplexNumber::new((2.0*PI/3.0).cos(), -1.0*(2.0*PI/3.0).sin()) * &data[0],
             &ComplexNumber::new((2.0*PI/3.0).cos(), -1.0*(2.0*PI/3.0).sin()) * &data[1],
             &ComplexNumber::new((2.0*PI/3.0).cos(), -1.0*(2.0*PI/3.0).sin()) * &data[2]],
            [&ComplexNumber::new((4.0*PI/3.0).cos(), -1.0*(4.0*PI/3.0).sin()) * &data[0],
             &ComplexNumber::new((4.0*PI/3.0).cos(), -1.0*(4.0*PI/3.0).sin()) * &data[1],
             &ComplexNumber::new((4.0*PI/3.0).cos(), -1.0*(4.0*PI/3.0).sin()) * &data[2]]
            ];
        assert_eq!(fft_object.snk, expected_snk);
    }
}