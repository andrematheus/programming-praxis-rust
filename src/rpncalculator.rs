use std::num;
use std::collections;

pub struct RpnCalculator {
    stack: CalcStack,
    operators: OperatorsMap,
}

#[derive(Debug)]
pub enum RpnCalculatorError {
    ParsingError,
    NotEnoughOperands,
}

impl From<num::ParseFloatError> for RpnCalculatorError {
    fn from(_: num::ParseFloatError) -> RpnCalculatorError {
        RpnCalculatorError::ParsingError
    }
}

pub type Result = std::result::Result<(), RpnCalculatorError>;
pub type CalcStack = Vec<f64>;
pub type OperatorFn = fn(&mut CalcStack) -> Result;
pub type OperatorsMap = collections::BTreeMap<&'static str, OperatorFn>;

macro_rules! binop {
    ($ops:expr, $name:expr, $op:expr) => {{
        fn opfn(s: &mut CalcStack) -> Result {
            let x = s.pop().ok_or(RpnCalculatorError::NotEnoughOperands)?;
            let y = s.pop().ok_or(RpnCalculatorError::NotEnoughOperands)?;
            let result = $op(x, y);
            s.push(result);
            Ok(())
        }
        $ops.insert($name, opfn);
    }}
}

fn default_operators() -> OperatorsMap {
    let mut ops: OperatorsMap = collections::BTreeMap::new();
    binop!(ops, "+", |x, y| x + y);

    ops
}

impl RpnCalculator {
    fn new() -> RpnCalculator {
        RpnCalculator { stack: Vec::new(), operators: default_operators() }
    }

    fn new_with_operators(operators: OperatorsMap) -> RpnCalculator {
        RpnCalculator { stack: Vec::new(), operators: operators }
    }

    fn add_operators(&mut self, mut operators: OperatorsMap) {
        self.operators.append(&mut operators);
    }

    fn add_operator(&mut self, symb: &'static str, f: OperatorFn) {
        self.operators.insert(symb, f);
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
        if self.operators.contains_key(token) {
            let operator = self.operators.get(token).expect("Already checked if operators contains token");
            operator(&mut self.stack)
        } else {
            self.parse_and_push(token)
        }
    }

    fn top(&self) -> f64 {
        *self.stack.last().unwrap()
    }

    fn parse_and_push(&mut self, token: &str) -> Result {
        let value: f64 = token.parse()?;
        self.stack.push(value);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections;

    fn make_calculator() -> RpnCalculator {
        RpnCalculator::new()
    }

    fn make_calculator_with_operators(operators: OperatorsMap) -> RpnCalculator {
        RpnCalculator::new_with_operators(operators)
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
        fn pop(s: &mut CalcStack) -> Result {
            s.pop();
            Ok(())
        }
        calc.add_operator("X", pop);
        calc.evaluate("2.5 3.2").unwrap();
        assert_eq!(3.2, calc.top());
        calc.evaluate("X").unwrap();
        assert_eq!(2.5, calc.top());
    }

    #[test]
    fn should_add_two_f64_in_stack() {
        let mut calc = make_calculator();
        calc.evaluate("2.5 3.2 +").unwrap();
        assert_eq!(5.7, calc.top(), "Calcultor's top should be result of addition");
    }

    #[test]
    fn should_return_error_when_adding_without_enough_operands() {
        let mut calc = make_calculator();
        let result = calc.evaluate("+");
        assert!(result.is_err(), "Should return error because '+' expects two operands");
        match result {
            Err(RpnCalculatorError::NotEnoughOperands) => (),
            _ => assert!(false, "Should return NotEnoughOperands error"),
        }
    }

    #[test]
    fn should_use_operators_passed_at_construction_time() {
        let mut operators: OperatorsMap = collections::BTreeMap::new();
        fn test_op(s: &mut CalcStack) -> Result {
            s.push(10.0);
            Ok(())
        }
        operators.insert("?", test_op);
        let mut calc = make_calculator_with_operators(operators);
        let result = calc.evaluate("?");
        assert!(result.is_ok(), "Should return ok as input is valid");
        assert_eq!(10.0, calc.top(), "Should have returned value at the top");
    }

    #[test]
    fn should_extend_default_operators_with_operators() {
        let mut operators: OperatorsMap = collections::BTreeMap::new();
        fn test_op(s: &mut CalcStack) -> Result {
            s.push(10.0);
            Ok(())
        }
        operators.insert("?", test_op);
        let mut calc = make_calculator();
        calc.add_operators(operators);
        let result = calc.evaluate("? 2 +");
        assert!(result.is_ok(), "Should return ok as input is valid");
        assert_eq!(12.0, calc.top(), "Should have returned result of 10.0 + 2 at the top");
    }

    #[test]
    fn should_extend_default_operators_with_operator() {
        fn test_op(s: &mut CalcStack) -> Result {
            s.push(10.0);
            Ok(())
        }
        let mut calc = make_calculator();
        calc.add_operator("?", test_op);
        let result = calc.evaluate("? 2 +");
        assert!(result.is_ok(), "Should return ok as input is valid");
        assert_eq!(12.0, calc.top(), "Should have returned result of 10.0 + 2 at the top");
    }
}

fn main() {
}
