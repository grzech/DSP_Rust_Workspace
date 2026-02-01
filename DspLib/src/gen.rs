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

pub struct TriangleWave {
    period: f64,
    slope: f64,
}

pub struct SineWave {
    frequency: f64,
}

pub struct RectangleWave {
    period: f64,
    duty_cycle: f64,
    high_phase: f64,
}

pub struct DiracDelta {}

impl SignalShape for TriangleWave {
    fn function(&mut self, x: f64) -> f64 {
        let phase = x % self.period;
        if phase < self.period * 0.25 {
            return phase * self.slope;
        } else if phase < self.period * 0.75 {
            return 2.0 - phase * self.slope;
        } else {
            return phase * self.slope - 4.0;
        }
    }
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

impl SignalShape for DiracDelta {
    fn function(&mut self, x: f64) -> f64 {
        if x == 0.0 {
            return 1.0;
        } else {
            return 0.0;
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

impl Generator<TriangleWave> {
    pub fn triangle_wave(frequency: f64) -> Generator<TriangleWave> {
        Generator {
            signal: DescreteSignal::new(),
            amplitude: 1.0,
            frequency: frequency,
            periods: 1.0,
            phase: 0.0,
            sampling_rate: DEFAULT_SAMPLING * frequency,
            offset: 0.0,
            shape: TriangleWave{period: 1.0 / frequency, slope: frequency * 4.0}
       }
    }

    pub fn set_frequency(mut self, frequency: f64) -> Self {
        self.frequency = frequency;
        self.shape.period = 1.0 / frequency;
        self.shape.slope = frequency * 4.0;
        self
    }
}

impl Generator<DiracDelta> {
    pub fn dirac_delta() -> Generator<DiracDelta> {
        Generator {
            signal: DescreteSignal::new(),
            amplitude: f64::MAX,
            frequency: 1.0,
            periods: 1.0,
            phase: 0.0,
            sampling_rate: DEFAULT_SAMPLING,
            offset: 0.0,
            shape: DiracDelta{}
       }
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
    fn check_set_amplitude() {
        let mut gen = Generator::sine_wave(1.0);

        for amp in [13.55, 4311.3, -32.33, 124121.444, -32490.33] {
            gen = gen.set_amplitude(amp);
            assert_eq!(gen.amplitude, amp);
        }
    }

    #[test]
    fn check_set_phase() {
        let mut gen = Generator::sine_wave(1.0);

        for ph in [13.55, 4311.3, -32.33, 124121.444, -32490.33] {
            gen = gen.set_phase_shift(ph);
            assert_eq!(gen.phase, ph);
        }
    }

    #[test]
    fn check_set_periods() {
        let mut gen = Generator::sine_wave(1.0);

        for period in [13.55, 4311.3, -32.33, 124121.444, -32490.33] {
            gen = gen.set_number_of_periods(period);
            assert_eq!(gen.periods, period);
        }
    }

    #[test]
    fn check_set_offset() {
        let mut gen = Generator::sine_wave(1.0);

        for offset in [0.1, 1.3, -35.33, -6.9, 11.3] {
            gen = gen.set_offset(offset);
            assert_eq!(gen.offset, offset);
        }
    }

    #[test]
    fn check_set_sampling_rate() {
        let mut gen = Generator::sine_wave(1.0);

        for rate in [13.55, 4311.3, -32.33, 124121.444, -32490.33] {
            gen = gen.set_sampling_rate(rate);
            assert_eq!(gen.sampling_rate, rate);
        }
    }

    #[test]
    fn check_set_frequency() {
        let mut gen = Generator::sine_wave(1.0);

        for freq in [13.55, 4311.3, -32.33, 124121.444, -32490.33] {
            gen = gen.set_frequency(freq);
            assert_eq!(gen.frequency, freq);
            assert_eq!(gen.shape.frequency, freq);
        }
    }

    #[test]
    fn check_rectangle_shape_parameters() {
        let freq = 0.25;
        let duty = 0.75;
        let gen = Generator::rectangle_wave(freq, duty);

        assert_eq!(gen.shape.duty_cycle, duty);
        assert_eq!(gen.shape.period, 1.0/freq);
        assert_eq!(gen.shape.high_phase, duty/freq);
    }

    #[test]
    fn check_set_duty_cycle() {
        let freq = 0.1;
        let mut gen = Generator::rectangle_wave(freq, 0.5);

        for duty in [0.1, 0.2, 0.25, 0.5] {
            gen = gen.set_duty_cycle(duty);
            assert_eq!(gen.shape.duty_cycle, duty);
            assert_eq!(gen.shape.high_phase, duty/freq);
        }
    }

    #[test]
    fn check_set_frequency_for_rectangle() {
        let duty = 0.1;
        let mut gen = Generator::rectangle_wave(1.0, duty);

        for freq in [0.1, 0.02, 0.0025, 0.0005] {
            gen = gen.set_frequency(freq);
            assert_eq!(gen.frequency, freq);
            assert_eq!(gen.shape.period, 1.0/freq);
            assert_eq!(gen.shape.duty_cycle, duty);
            assert_eq!(gen.shape.high_phase, duty/freq);
        }
    }

    #[test]
    fn check_triangle_shape_parameters() {
        let freq = 1213.432;
        let gen = Generator::triangle_wave(freq);

        assert_eq!(gen.shape.period, 1.0 / freq);
        assert_eq!(gen.shape.slope, freq * 4.0);
    }


    #[test]
    fn check_set_frequency_for_triangle() {
        let mut gen = Generator::triangle_wave(1.0);

        for freq in [0.1, 0.02, 0.0025, 0.0005] {
            gen = gen.set_frequency(freq);
            assert_eq!(gen.frequency, freq);
            assert_eq!(gen.shape.period, 1.0/freq);
            assert_eq!(gen.shape.slope, 4.0*freq);
        }
    }
}