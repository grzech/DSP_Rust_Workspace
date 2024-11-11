pub mod generators {
    use std::f64::consts::PI;

    pub fn generate_sine(amplitude: f32, frequency: f32, number_of_periods: f32, sampling_rate: f32) -> Vec<(f32, f32)> {
        let mut signal = Vec::new();
        let step = (frequency / sampling_rate) as f64;
        let mut x = 0.0;

        while x < number_of_periods as f64 {
            signal.push((x as f32, (x * PI * 2.0).sin() as f32 * amplitude));
            x += step;
        }
        
        
        
        
        signal
    }
}
