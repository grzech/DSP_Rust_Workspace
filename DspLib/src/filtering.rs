use std::f64::consts::PI;

use crate::DescreteSignal;

pub fn fir_filter(signal: &DescreteSignal, fir: &[f64], output: &mut DescreteSignal) {
    for n in (fir.len()-1)..signal.len(){
        let mut y = 0.0;
        for (i, h) in fir.iter().enumerate() {
            y += h * signal[n-i].1;
        }
        output.push(signal[n+1-fir.len()].0, y);
    }
}

pub fn get_averaging_fir_coefficients(size: usize, fir: &mut [f64]) {
    let len = if size >= fir.len() {
        size
    }else {
        fir.len()
    };

    for i in 0..len {
        fir[i] = 1.0/len as f64;
    }
}

pub fn get_low_pass_fir_coefficients(size: usize, fir: &mut [f64]) {
    let freq = 1000.0;
    let len = if size > fir.len() {
        fir.len()
    }else {
        size
    };

    
    for i in 1..=len {
        let arg = i as f64 / (0.2*freq*(len as f64));
        fir[i-1] = 2.0 * freq * (2.0 * PI * freq * arg).sin()/(2.0 * PI * freq * arg);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fir_filter_shall_copy_timestamps_to_output() {
        let timestamps = [1.0, 3.2, 4.4, 65.2, 99.4, 23213.14];
        let mut signal = DescreteSignal::new();
        for t in timestamps {
            signal.push(t, t);
        }
        let fir = vec![1.0];
        let mut output = DescreteSignal::new();

        fir_filter(&signal, &fir, &mut output);
        for (i, &(t, _)) in output.get_data().into_iter().enumerate() {
            assert_eq!(t, timestamps[i]);
        }
    }

    #[test]
    fn fir_filter_shall_return_data_of_correct_length() {
        let sig_len = 1154;
        let fir_len = 32;
        let signal = DescreteSignal::new_from_vec(
            vec![(0.0, 0.0); sig_len]);
        let fir = vec![0.0; fir_len];
        let mut output = DescreteSignal::new();
        let expected_len = sig_len + 1 -fir_len;
        
        fir_filter(&signal, &fir, &mut output);
        assert_eq!(output.len(), expected_len);
    }

    #[test]
    fn fir_filter_shall_perform_convolution_on_signal() {
        let signal = DescreteSignal::new_from_vec(
            vec![(0.0, 0.0), (1.0, 1.0), (2.0, 0.0), (3.0, -1.0),
                      (4.0, 0.0), (5.0, 1.0), (6.0, 0.0), (7.0, -1.0)]);
        let fir = vec![0.1, 2.0, 10.0];
        let mut output = DescreteSignal::new();
        let expected_output = vec![(0.0, 2.0), (1.0, 9.9), (2.0, -2.0), (3.0, -9.9), (4.0, 2.0), (5.0, 9.9)];
        
        fir_filter(&signal, &fir, &mut output);
        assert_eq!(output.get_data(), expected_output);
    }
}