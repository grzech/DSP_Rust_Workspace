use std::f64::consts::PI;

trait SignalShape {
    fn function(&mut self, x: f64) -> f64;
    fn generate_signal(&mut self, amplitude: f64,
                       frequency: f64,
                       number_of_periods: f64,
                       sampling_rate: f64,
                       phase_shift: f64) -> DescreteSignal
    {
        let mut signal = DescreteSignal::new();
        let step = 1.0/sampling_rate;
        let mut x = phase_shift/frequency;
        let end = number_of_periods/frequency + x;

        println!("Step by {step} from {x} to {end}");
        while x < end {
            signal.push(x, self.function(x) * amplitude);
            x += step;
        }
        
        signal
    }
}

struct SineWave {
    frequency: f64,
}

impl SignalShape for SineWave {
    fn function(&mut self, x: f64) -> f64 {
        (x * self.frequency * 2.0 * PI).sin()
    }
}

pub struct DescreteSignal {
    data: Vec<(f64, f64)>,
}

impl DescreteSignal {
    pub fn new() -> Self {
        Self{data: Vec::new()}
    }

    pub fn push(&mut self, x: f64, y: f64) {
        self.data.push((x, y));
    }
}

pub struct Generator{
    signal: DescreteSignal,
    amplitude: f64,
    frequency: f64,
    periods: f64,
    phase: f64,
    sampling_rate: f64,
}

impl Default for Generator {
    fn default() -> Self {
        Self{signal: DescreteSignal::new(),
             amplitude: 1.0,
             frequency: 1.0,
             periods: 1.0,
             phase: 0.0,
             sampling_rate: 20.0,
        }
    }
}

impl Generator {
    pub fn generate(&mut self) -> &[(f64, f64)] {
        let mut shape = SineWave{frequency: self.frequency};
        self.signal = shape.generate_signal(self.amplitude,
                                            self.frequency,
                                            self.periods,
                                            self.sampling_rate,
                                            self.phase);
        &self.signal.data
    }

    pub fn set_amplitude(mut self, amp: f64) -> Self {
        self.amplitude = amp;
        self
    }

    pub fn set_frequency(mut self, freq: f64) -> Self {
        self.frequency = freq;
        self
    }

    pub fn set_phase_shift(mut self, ph: f64) -> Self {
        self.phase = ph;
        self
    }

    pub fn set_sampling_rate(mut self, rate: f64) -> Self {
        self.sampling_rate = rate;
        self
    }

    pub fn set_number_of_periods(mut self, periods: f64) -> Self {
        self.periods = periods;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_default_values() {
        let gen = Generator::default();

        assert_eq!(gen.amplitude, 1.0);
        assert_eq!(gen.frequency, 1.0);
        assert_eq!(gen.periods, 1.0);
        assert_eq!(gen.phase, 0.0);
        assert_eq!(gen.sampling_rate, 20.0);
        assert_eq!(gen.signal.data, Vec::new());
    }

    #[test]
    fn check_set_amplitude() {
        let mut gen = Generator::default();

        for amp in [13.55, 4311.3, -32.33, 124121.444, -32490.33] {
            gen = gen.set_amplitude(amp);
            assert_eq!(gen.amplitude, amp);
        }
    }

    #[test]
    fn check_set_frequency() {
        let mut gen = Generator::default();

        for freq in [13.55, 4311.3, -32.33, 124121.444, -32490.33] {
            gen = gen.set_frequency(freq);
            assert_eq!(gen.frequency, freq);
        }
    }

    #[test]
    fn check_set_phase() {
        let mut gen = Generator::default();

        for ph in [13.55, 4311.3, -32.33, 124121.444, -32490.33] {
            gen = gen.set_phase_shift(ph);
            assert_eq!(gen.phase, ph);
        }
    }

    #[test]
    fn check_set_periods() {
        let mut gen = Generator::default();

        for period in [13.55, 4311.3, -32.33, 124121.444, -32490.33] {
            gen = gen.set_number_of_periods(period);
            assert_eq!(gen.periods, period);
        }
    }

    #[test]
    fn check_set_sampling_rate() {
        let mut gen = Generator::default();

        for rate in [13.55, 4311.3, -32.33, 124121.444, -32490.33] {
            gen = gen.set_sampling_rate(rate);
            assert_eq!(gen.sampling_rate, rate);
        }
    }
}