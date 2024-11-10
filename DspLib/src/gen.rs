pub mod generators {
    use std::f64::consts::PI;

    pub fn generate_sine(amplitude: f32, frequency: f32, number_of_periods: f32, sampling_rate: f32) -> Vec<(f32, f32)> {
        let mut signal = Vec::new();
        let two_pi = PI * 2.0;
        let step = (frequency / sampling_rate) as f64 * two_pi;
        let mut x = 0.0;
        while x < two_pi * number_of_periods as f64 {
            signal.push((x as f32, x.sin() as f32 * amplitude));
            x += step;
        }
        
        
        
        
        signal
    }
}
