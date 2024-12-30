use std::f64::consts::PI;
use crate::DescreteSignal;

pub trait SignalShape {
    fn function(&mut self, x: f64) -> f64;
    fn generate_signal(&mut self, amplitude: f64,
                       frequency: f64,
                       number_of_periods: f64,
                       sampling_rate: f64,
                       phase_shift: f64,
                       offset: f64) -> DescreteSignal
    {
        let mut signal = DescreteSignal::new();
        let step = 1.0/sampling_rate;
        let mut x = phase_shift/frequency;
        let end = number_of_periods/frequency + x;

        while x < end {
            signal.push(x, self.function(x) * amplitude + offset);
            x += step;
        }
        
        signal
    }
}

pub struct SineWave {
    frequency: f64,
}

pub struct RectangleWave {
    period: f64,
    duty_cycle: f64,
    high_phase: f64,
}

impl SignalShape for SineWave {
    fn function(&mut self, x: f64) -> f64 {
        (x * self.frequency * 2.0 * PI).sin()
    }
}

impl SignalShape for RectangleWave {
    fn function(&mut self, x: f64) -> f64 {
        let phase = x % self.period;
        if phase > self.high_phase {
            -1.0
        } else {
            1.0
        }
    }
}

pub struct Generator<S: SignalShape> {
    signal: DescreteSignal,
    amplitude: f64,
    frequency: f64,
    periods: f64,
    phase: f64,
    sampling_rate: f64,
    offset: f64,
    shape: S,
}

const DEFAULT_SAMPLING: f64 = 20.0;

impl Default for Generator<SineWave> {
    fn default() -> Generator<SineWave> {
        let freq = 1.0;
        Self{signal: DescreteSignal::new(),
             amplitude: 1.0,
             frequency: freq,
             periods: 1.0,
             phase: 0.0,
             sampling_rate: DEFAULT_SAMPLING,
             offset: 0.0,
             shape: SineWave{frequency: freq}
        }
    }
}

impl Generator<SineWave> {
    pub fn sine_wave(frequency: f64) -> Self {
        Self {
            signal: DescreteSignal::new(),
            amplitude: 1.0,
            frequency: frequency,
            periods: 1.0,
            phase: 0.0,
            sampling_rate: DEFAULT_SAMPLING * frequency,
            offset: 0.0,
            shape: SineWave{frequency}
       }
    }

    pub fn set_frequency(mut self, frequency: f64) -> Self {
        self.frequency = frequency;
        self.shape.frequency = frequency;
        self
    }
}

impl Generator<RectangleWave> {
    pub fn rectangle_wave(frequency: f64, duty_cycle: f64) -> Generator<RectangleWave> {
        let period = 1.0/frequency;

        Generator {
            signal: DescreteSignal::new(),
            amplitude: 1.0,
            frequency: frequency,
            periods: 1.0,
            phase: 0.0,
            sampling_rate: DEFAULT_SAMPLING * frequency,
            offset: 0.0,
            shape: RectangleWave{period, duty_cycle, high_phase: duty_cycle * period}
       }
    }

    pub fn set_frequency(mut self, frequency: f64) -> Self {
        let period = 1.0/frequency;
        self.frequency = frequency;
        self.shape.period = period;
        self.shape.high_phase = self.shape.duty_cycle * period;
        self
    }

    pub fn set_duty_cycle(mut self, duty_cycle: f64) -> Self {
        self.shape.duty_cycle = duty_cycle;
        self.shape.high_phase = self.shape.duty_cycle * self.shape.period;
        self
    }
}

impl<S: SignalShape> Generator<S> {
    pub fn generate(&mut self) -> DescreteSignal {
        self.signal = self.shape.generate_signal(self.amplitude,
                                            self.frequency,
                                            self.periods,
                                            self.sampling_rate,
                                            self.phase,
                                            self.offset);
        self.signal.clone()
    }

    pub fn set_amplitude(mut self, amp: f64) -> Self {
        self.amplitude = amp;
        self
    }

    pub fn set_offset(mut self, offset: f64) -> Self {
        self.offset = offset;
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
        assert_eq!(gen.sampling_rate, DEFAULT_SAMPLING);
        assert_eq!(gen.signal.get_data(), Vec::new());
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
    fn check_set_offset() {
        let mut gen = Generator::default();

        for offset in [0.1, 1.3, -35.33, -6.9, 11.3] {
            gen = gen.set_offset(offset);
            assert_eq!(gen.offset, offset);
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