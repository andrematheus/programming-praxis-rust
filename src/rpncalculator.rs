use std::num;

pub struct RpnCalculator {
    stack: Vec<f64>,
}

#[derive(Debug)]
enum RpnCalculatorError {
    EmptyStack,
    ParsingError,
}

impl From<num::ParseFloatError> for RpnCalculatorError {
    fn from(_: num::ParseFloatError) -> RpnCalculatorError {
        RpnCalculatorError::ParsingError
    }
}

type Result = std::result::Result<(), RpnCalculatorError>;

impl RpnCalculator {
    fn new() -> RpnCalculator {
        RpnCalculator { stack: Vec::new() }
    }

    fn evaluate(&mut self, input: &str) -> Result {
        let mut tokens = input.split_whitespace();
        loop {
            let next = tokens.next();
            match next {
                None => break,
                Some(token) => self.parse_token(token)?,
            }
        }
        Ok(())
    }

    fn parse_token(&mut self, token: &str) -> Result {
        match token {
            "+" => self.add_two(),
            _ => self.parse_and_push(token),
        }
    }

    fn top(&self) -> f64 {
        *self.stack.last().unwrap()
    }

    fn pop(&mut self) -> f64{
        self.stack.pop().unwrap()
    }

    fn parse_and_push(&mut self, token: &str) -> Result {
        let value: f64 = token.parse()?;
        self.stack.push(value);
        Ok(())
    }

    fn add_two(&mut self) -> Result {
        let x = self.stack.pop().ok_or(RpnCalculatorError::EmptyStack)?;
        let y = self.stack.pop().ok_or(RpnCalculatorError::EmptyStack)?;
        let s = x + y;
        self.stack.push(s);
        Ok(())
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
        calc.evaluate("2.5").unwrap();
        assert_eq!(2.5, calc.top());
    }

    #[test]
    fn should_return_error_when_evaluating_garbage() {
        let mut calc = make_calculator();
        let result = calc.evaluate("garbage");
        assert!(result.is_err());
    }

    #[test]
    fn should_add_two_f64_to_stack() {
        let mut calc = make_calculator();
        calc.evaluate("2.5 3.2").unwrap();
        assert_eq!(3.2, calc.top());
        calc.pop();
        assert_eq!(2.5, calc.top());
    }

    #[test]
    fn should_add_two_f64_in_stack() {
        let mut calc = make_calculator();
        calc.evaluate("2.5 3.2 +").unwrap();
        assert_eq!(5.7, calc.top());
    }

    #[test]
    fn should_return_error_when_adding_with_empty_stack() {
        let mut calc = make_calculator();
        let result = calc.evaluate("+");
        assert!(result.is_err());
    }
}
