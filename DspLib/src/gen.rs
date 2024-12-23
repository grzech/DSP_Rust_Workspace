use std::f64::consts::PI;
use std::ops::Add;

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

impl SignalShape for SineWave {
    fn function(&mut self, x: f64) -> f64 {
        (x * self.frequency * 2.0 * PI).sin()
    }
}


pub struct DescreteSignal {
    data: Vec<(f64, f64)>,
}

impl Clone for DescreteSignal {
    fn clone(&self) -> Self {
        let mut data = vec![];
        for d in self.data.iter() {
            data.push(*d);
        }
        DescreteSignal{data}
    }
}

impl DescreteSignal {
    pub fn new() -> Self {
        Self{data: Vec::new()}
    }

    pub fn new_from_vec(data: Vec<(f64, f64)>) -> Self {
        Self{data}
    }

    pub fn push(&mut self, x: f64, y: f64) {
        self.data.push((x, y));
    }

    pub fn get_data(&self) -> &[(f64, f64)] {
        &self.data
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    fn get_approximation_coeficients((x1, y1): (f64, f64),
                                     (x2, y2): (f64, f64)) -> (f64, f64)
    {
        let a = (y1-y2)/(x1-x2);
        let b = y1 - a*x1;

        (a, b)
    }

    fn add_data_until_stamp(&mut self, data: &[(f64, f64)], offset: usize, stamp: f64) -> usize {
        for (i, (x, y)) in data.iter().enumerate() {
            if *x >= stamp {
                return offset + i;
            } else {
                self.push(*x, *y);
            }
        }

        offset + data.len()
    }

    fn add_data(&mut self, data: &[(f64, f64)], offset: usize) {
        for (x, y) in &data[offset..] {
            self.push(*x, *y);
        }
    }

    fn insert_data(&mut self,
                   data: &[(f64, f64)],
                   offset: usize,
                   (p1, p2): ((f64, f64), (f64, f64))) -> usize
    {
        let mut i = offset;
        let (a, b) = Self::get_approximation_coeficients(p1, p2);

        while i < data.len() {
            let (x, y) = data[i];
            i += 1;
            if x < p2.0 {
                self.push(x, y + a*x+b);
            } else {
                return i - 1;
            }
        }

        i
    }
}

impl Add<&DescreteSignal> for &DescreteSignal {
    type Output = DescreteSignal;

    fn add(self, rhs: &DescreteSignal) -> Self::Output {
        let mut ret = DescreteSignal::new();
        let mut l = 0;
        let mut r = 0;

        if self.data[l].0 > rhs.data[r].0 {
            r = ret.add_data_until_stamp(&rhs.data, r, self.data[0].0);
        } else {
            l = ret.add_data_until_stamp(&self.data, l, rhs.data[0].0);
        }

        while l < self.data.len() && r < rhs.data.len() {
            if self.data[l].0 == rhs.data[r].0 {
                ret.push(self.data[l].0, self.data[l].1 + rhs.data[r].1);
                l += 1;
                r += 1;
            } else if self.data[l].0 > rhs.data[r].0 {
                r = ret.insert_data(&rhs.data, r, (self.data[l-1], self.data[l]));
            } else {
                l = ret.insert_data(&self.data, l, (rhs.data[r-1], rhs.data[r]));
            }
        }

        if self.data.len() > l {
            ret.add_data(&self.data, l);
        } else {
            ret.add_data(&rhs.data, r);
        }

        ret
    }


}

pub struct Generator{
    signal: DescreteSignal,
    amplitude: f64,
    frequency: f64,
    periods: f64,
    phase: f64,
    sampling_rate: f64,
    offset: f64,
}

impl Default for Generator {
    fn default() -> Self {
        Self{signal: DescreteSignal::new(),
             amplitude: 1.0,
             frequency: 1.0,
             periods: 1.0,
             phase: 0.0,
             sampling_rate: 20.0,
             offset: 0.0
        }
    }
}

impl Generator {
    pub fn generate(&mut self) -> DescreteSignal {
        let mut shape = SineWave{frequency: self.frequency};
        self.signal = shape.generate_signal(self.amplitude,
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

    #[test]
    fn adding_shall_extend_signal_with_data_before_first_stamp() {
        let first = DescreteSignal{data: vec![(0.1, 0.0), (0.2, 300.0), (0.3, 45.22)]};
        let second = DescreteSignal{data: vec![(0.0, 0.1), (0.01, 11.3), (0.02, 4.78)]};
        let sum = vec![(0.0, 0.1), (0.01, 11.3), (0.02, 4.78), (0.1, 0.0), (0.2, 300.0), (0.3, 45.22)];
        
        assert_eq!((&first + &second).data, sum);
    }

    #[test]
    fn adding_shall_extend_signal_with_data_after_last_stamp() {
        let first = DescreteSignal{data: vec![(0.1, 0.0), (0.2, 300.0), (0.3, 45.22)]};
        let second = DescreteSignal{data: vec![(0.5, 0.1), (0.51, 11.3), (0.52, 4.78)]};
        let sum = vec![(0.1, 0.0), (0.2, 300.0), (0.3, 45.22), (0.5, 0.1), (0.51, 11.3), (0.52, 4.78)];
        
        assert_eq!((&first + &second).data, sum);
    }

    #[test]
    fn check_adding_to_signals_with_equal_stamps() {
        let first = DescreteSignal{data: vec![(0.0, 0.0), (0.1, 300.0), (1.5, 45.22)]};
        let second = DescreteSignal{data: vec![(0.0, 0.1), (0.1, 11.3), (1.5, 4.78)]};
        let sum = vec![(0.0, 0.1), (0.1, 311.3), (1.5, 50.0)];
        
        assert_eq!((&first + &second).data, sum);
    }

    #[test]
    fn check_adding_to_signals_with_stamps_between() {
        let first = DescreteSignal{data: vec![(0.0, 0.0), (1.0, 10.0), (2.25, 22.5), (2.5, 25.0), (2.75, 27.5)]};
        let second = DescreteSignal{data: vec![(1.0, 2.0), (2.0, 4.0), (3.0, 6.0)]};
        let sum = vec![(0.0, 0.0), (1.0, 12.0), (2.0, 24.0), (2.25, 27.0), (2.5, 30.0), (2.75, 33.0), (3.0, 6.0)];
        
        assert_eq!((&first + &second).data, sum);
    }

    #[test]
    fn check_adding_mixed_data() {
        let first = DescreteSignal{data: vec![(0.0, 1.0), (1.0, 11.0), (2.0, 21.0), (3.0, 31.0), (4.0, 41.0)]};
        let second = DescreteSignal{data: vec![(0.0, 2.0), (0.1, 3.0), (0.2, 4.0), (3.3, 35.0), (3.6, 38.0), (4.1, 43.0)]};
        let sum = vec![(0.0, 3.0), (0.1, 5.0), (0.2, 7.0), (1.0, 23.0), (2.0, 43.0), (3.0, 63.00000000000001), (3.3, 69.0), (3.6, 75.0), (4.0, 83.0), (4.1, 43.0)];
        
        assert_eq!((&first + &second).data, sum);
    }
}