
use std::{ops::{Add, Mul}, fmt::{Display, Debug}};

#[derive(Default, Clone, PartialEq)]
pub struct ComplexNumber (f64, f64);

impl ComplexNumber {
    pub fn new(re: f64, im:f64) -> Self {
        ComplexNumber(re, im)
    }

    pub fn re(&self) -> f64 {
        self.0
    }

    pub fn im(&self) -> f64 {
        self.1
    }

    pub fn module(&self) -> f64 {
        (self.0*self.0 + self.1*self.1).sqrt()
    }
}

impl Add<&ComplexNumber> for &ComplexNumber {
    type Output = ComplexNumber;

    fn add(self, other: &ComplexNumber) -> ComplexNumber {
        ComplexNumber(self.0 + other.0, self.1 + other.1)
    }
}

impl Add<&f64> for &ComplexNumber {
    type Output = ComplexNumber;

    fn add(self, other: &f64) -> ComplexNumber {
        ComplexNumber(self.0 + other, self.1)
    }
}

impl Mul<&ComplexNumber> for &ComplexNumber {
    type Output = ComplexNumber;

    fn mul(self, other: &ComplexNumber) -> ComplexNumber {
        ComplexNumber(self.0 * other.0 - self.1 * other.1,
                      self.0 * other.1 + self.1 * other.0)
    }
}


impl Mul<&f64> for &ComplexNumber {
    type Output = ComplexNumber;

    fn mul(self, other: &f64) -> ComplexNumber {
        ComplexNumber(self.0 * other, self.1 * other)
    }
}

impl Display for ComplexNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.1 < 0.0 {
            return write!(f, "{} - i{}", self.0, self.1.abs())
        }
        write!(f, "{} + i{}", self.0, self.1)
    }
}

impl Debug for ComplexNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.1 < 0.0 {
            return write!(f, "{} - i{}", self.0, self.1.abs())
        }
        write!(f, "{} + i{}", self.0, self.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_shall_create_tuple_based_on_args() {
        let input = (123.324, 53245.22);
        let c = ComplexNumber::new(input.0, input.1);
        assert_eq!(c.0, input.0);
        assert_eq!(c.1, input.1);
    }

    #[test]
    fn re_shall_return_first_value() {
        let input = (13124.14, 30980.232);
        let c = ComplexNumber::new(input.0, input.1);
        assert_eq!(c.re(), input.0);
    }


    #[test]
    fn im_shall_return_second_value() {
        let input = (984309.234, 8904358343.3);
        let c = ComplexNumber::new(input.0, input.1);
        assert_eq!(c.im(), input.1);
    }

    #[test]
    fn adding_shall_add_re_part_and_im_part_separately() {
        let in1 = (32432.12312, 1542.789);
        let in2 = (5498.3246, 468786.54387);
        let c1 = ComplexNumber::new(in1.0, in1.1);
        let c2 = ComplexNumber::new(in2.0, in2.1);
        let c3 = &c1 + &c2;
        assert_eq!(c3.re(), in1.0+in2.0);
        assert_eq!(c3.im(), in1.1+in2.1);
    }

    #[test]
    fn adding_float_shall_add_only_to_re_part() {
        let input = (6.1, 84.5);
        let re = 66.4;
        let c1 = ComplexNumber::new(input.0, input.1);
        let c3 = &c1 + &re;
        assert_eq!(c3.re(), input.0+re);
        assert_eq!(c3.im(), input.1);
    }

    #[test]
    fn multiplying_shall_follow_correct_procedure() {
        let (x1, y1) = (612.22, 845.22);
        let (x2, y2) = (1.2, 2.1);
        let c1 = ComplexNumber::new(x1, y1);
        let c2 = ComplexNumber::new(x2, y2);
        let c3 = &c1 * &c2;
        assert_eq!(c3.re(), x1*x2 - y1*y2);
        assert_eq!(c3.im(), x1*y2 + x2*y1);
    }

    #[test]
    fn multiplying_float_shall_multiply_both() {
        let input = (6.1, 84.5);
        let mul = 3.6;
        let c1 = ComplexNumber::new(input.0, input.1);
        let c3 = &c1 * &mul;
        assert_eq!(c3.re(), input.0 * mul);
        assert_eq!(c3.im(), input.1 * mul);
    }

    #[test]
    fn module_shall_return_correct_float_number() {
        let c = ComplexNumber::new(3.0, 4.0);
        let expected = 5.0;
        assert_eq!(c.module(), expected);
    }

    #[test]
    fn complex_shall_implement_display() {
        let c1 = ComplexNumber::new(3.0, 4.0);
        let c2 = ComplexNumber::new(-6.99, -33.2);
        let expected = String::from("3 + i4; -6.99 - i33.2");
        assert_eq!(format!("{}; {}", c1, c2), expected);
    }
}