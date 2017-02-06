//!
//! # Problem description:
//! #
//! # RPN Calculator
//! *February 19, 2009*
//!
//! Implement an RPN calculator that takes an expression like 19 2.14 +
//! 4.5 2 4.3 / - * which is usually expressed as (19 + 2.14) * (4.5 - 2 /
//! 4.3) and responds with 85.2974. The program should read expressions
//! from standard input and print the top of the stack to standard output
//! when a newline is encountered. The program should retain the state of
//! the operand stack between expressions.
//!

use std::num;
use std::collections;
use std::result;
use std::io;

pub struct RpnCalculator {
    stack: CalcStack,
    operators: OperatorsMap,
}

#[derive(Debug)]
pub enum RpnCalculatorError {
    ParsingError,
    NotEnoughOperands,
    Quit,
    IOError,
}

impl From<num::ParseFloatError> for RpnCalculatorError {
    fn from(_: num::ParseFloatError) -> RpnCalculatorError {
        RpnCalculatorError::ParsingError
    }
}

impl From<io::Error> for RpnCalculatorError {
    fn from(_: io::Error) -> RpnCalculatorError {
        RpnCalculatorError::IOError
    }
}

pub type CalcResult = result::Result<(), RpnCalculatorError>;
pub type CalcStack = Vec<f64>;
pub type OperatorFn = fn(&mut CalcStack) -> CalcResult;
pub type OperatorsMap = collections::BTreeMap<&'static str, OperatorFn>;

#[macro_export]
macro_rules! new_operator {
    ($ops:expr, $name:expr, [ $( $var:ident ),* ], $code:block) => {{
        fn opfn(s: &mut CalcStack) -> CalcResult {
            $(
                let $var = s.pop().ok_or(RpnCalculatorError::NotEnoughOperands)?;
            )*
            let result = { $code };
            s.push(result);
            Ok(())
        }
        $ops.insert($name, opfn);
    }};
    ($ops:expr, $name:expr, $stackvar:ident, $code:block) => {{
        fn opfn(s: &mut CalcStack) -> CalcResult {
            let $stackvar = s;
            $code
        }
        $ops.insert($name, opfn);
    }};
}

pub fn default_operators() -> OperatorsMap {
    let mut ops: OperatorsMap = collections::BTreeMap::new();
    new_operator!(ops, "+", [x, y], { x + y });

    ops
}

impl RpnCalculator {
    pub fn new() -> RpnCalculator {
        RpnCalculator { stack: Vec::new(), operators: default_operators() }
    }

    pub fn new_with_operators(operators: OperatorsMap) -> RpnCalculator {
        RpnCalculator { stack: Vec::new(), operators: operators }
    }

    pub fn evaluate(&mut self, input: &str) -> CalcResult {
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

    fn parse_token(&mut self, token: &str) -> CalcResult {
        if self.operators.contains_key(token) {
            let operator = self.operators.get(token).expect("Already checked if operators contains token");
            operator(&mut self.stack)
        } else {
            self.parse_and_push(token)
        }
    }

    pub fn top(&self) -> f64 {
        *self.stack.last().unwrap()
    }

    fn parse_and_push(&mut self, token: &str) -> CalcResult {
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
        new_operator!(calc.operators, "X", [_x, _y], {0.0});
        calc.evaluate("2.5 3.2").unwrap();
        assert_eq!(3.2, calc.top());
        calc.evaluate("X").unwrap();
        assert_eq!(0.0, calc.top());
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
        fn test_op(s: &mut CalcStack) -> CalcResult {
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
        let mut calc = make_calculator();
        new_operator!(calc.operators, "?", [], { 10.0 });
        let result = calc.evaluate("? 2 +");
        assert!(result.is_ok(), "Should return ok as input is valid");
        assert_eq!(12.0, calc.top(), "Should have returned result of 10.0 + 2 at the top");
    }

    #[test]
    fn should_be_possible_to_add_operator_that_operates_on_stack() {
        let mut calc = make_calculator();
        new_operator!(calc.operators, "?", s, { s.pop(); Ok(()) });
        let result = calc.evaluate("2 3 ?");
        assert!(result.is_ok());
        assert_eq!(2.0, calc.top(), "top should be popped");
    }
}
