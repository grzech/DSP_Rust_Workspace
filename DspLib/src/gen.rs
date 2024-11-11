use std::f64::consts::PI;

trait SignalShape {
    fn function(x: f64) -> f64;
    fn generate_signal(amplitude: f64, frequency: f64, number_of_periods: f64, sampling_rate: f64, phase_shift: f64) -> DescreteSignal {
        let mut signal = DescreteSignal::new();
        let step = frequency / sampling_rate;
        let end = phase_shift + number_of_periods;
        let mut x = phase_shift;

        while x < end {
            signal.push(x, Self::function(x) * amplitude);
            x += step;
        }
        
        signal
    }
}

enum SineWave {}

impl SignalShape for SineWave {
    fn function(x: f64) -> f64 {
        (x * 2.0 * PI).sin()
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
             sampling_rate: 25.0,
        }
    }
}

impl Generator {
    pub fn generate(&mut self) -> &[(f64, f64)] {
        self.signal = SineWave::generate_signal(self.amplitude,
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
