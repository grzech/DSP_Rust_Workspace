use std::f64::consts::PI;
use crate::DescreteSignal;

enum WaveShape {
    Sine,
    Rectangle,
}

trait SignalShape {
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

struct SineWave {
    frequency: f64,
}

struct RectangleWave {
    period: f64,
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

pub struct Generator {
    signal: DescreteSignal,
    amplitude: f64,
    frequency: f64,
    periods: f64,
    phase: f64,
    sampling_rate: f64,
    offset: f64,
    shape: WaveShape,
}

impl Default for Generator {
    fn default() -> Self {
        Self{signal: DescreteSignal::new(),
             amplitude: 1.0,
             frequency: 1.0,
             periods: 1.0,
             phase: 0.0,
             sampling_rate: 20.0,
             offset: 0.0,
             shape: WaveShape::Sine
        }
    }
}

impl Generator {
    pub fn generate(&mut self) -> DescreteSignal {
        let mut shape: Box<dyn SignalShape> = match self.shape {
            WaveShape::Sine => Box::new(SineWave{frequency: self.frequency}),
            WaveShape::Rectangle => Box::new(RectangleWave{period: 1.0/self.frequency, high_phase: 0.5}),
        };
        self.signal = shape.generate_signal(self.amplitude,
                                            self.frequency,
                                            self.periods,
                                            self.sampling_rate,
                                            self.phase,
                                            self.offset);
        self.signal.clone()
    }

    pub fn sine_wave(mut self) -> Self {
        self.shape = WaveShape::Sine;
        self
    }

    pub fn rectangle_wave(mut self) -> Self {
        self.shape = WaveShape::Rectangle;
        self
    }

    pub fn set_amplitude(mut self, amp: f64) -> Self {
        self.amplitude = amp;
        self
    }

    pub fn set_offset(mut self, offset: f64) -> Self {
        self.offset = offset;
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