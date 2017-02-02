pub struct RpnCalculator {
    stack: Vec<f64>,
}

impl RpnCalculator {
    fn new() -> RpnCalculator {
        RpnCalculator { stack: Vec::new() }
    }

    fn evaluate(&mut self, input: &str) {
        let value: f64 = input.parse().unwrap();
        self.stack.push(value);
    }

    fn top(&self) -> f64 {
        *self.stack.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_calculator() -> RpnCalculator {
        RpnCalculator::new()
    }

    #[test]
    fn should_create_a_calculator() {
        let _ = make_calculator();
    }

    #[test]
    fn should_add_f64_to_stack() {
        let mut calc = make_calculator();
        calc.evaluate("2.5");
        assert_eq!(2.5, calc.top());
    }
}
