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

/// All RPN Calculator errors
#[derive(Debug)]
pub enum RpnCalculatorError {
    /// Error parsing input
    ParsingError,
    /// Not enough operands in the stack for doing the operation
    NotEnoughOperands,
    /// This error signals that the calculator has to quit (maybe should not be an error?)
    Quit,
    /// This is used when there is an IO error outside the calc, maybe should be done some other way.
    /// Both Quit and IOError could be replaced by some standardized way of defining custom returns
    /// for calculator operators.
    IOError,
}

/// The result used fo all calculator operations
pub type CalcResult = result::Result<(), RpnCalculatorError>;
/// The stack used by the calculator
pub type CalcStack = Vec<f64>;
/// The function each operator uses for mutating the calculator stack
pub type OperatorFn = fn(&mut CalcStack) -> CalcResult;
/// A mapping of string symbols to operator functions
pub type OperatorsMap = collections::BTreeMap<&'static str, OperatorFn>;

/// Defines new operators and putting them in an operators map.
///
/// There are two forms of this macro:
///
/// * Define an operator that takes *n* operands and returns a value to be pushed into the stack
///
/// ```
/// #[macro_use]
/// extern crate pprust;
/// # fn main() {
/// use pprust::rpncalculator::*;
///
/// let mut ops = default_operators();
/// new_operator!(ops, "+", [x, y], { x + y });
/// let mut stack : Vec<f64> = Vec::new();
/// let f = ops.get("+").unwrap();
/// stack.push(1.0);
/// stack.push(2.0);
/// f(&mut stack);
/// assert_eq!(3.0, *stack.last().unwrap());
/// # }
/// ```
///
/// * Define an operator that operates directly on the stack
///
/// ```
/// #[macro_use]
/// extern crate pprust;
/// # fn main() {
/// use pprust::rpncalculator::*;
/// let mut ops = default_operators();
/// let mut stack : Vec<f64> = Vec::new();
/// stack.push(1.0);
/// new_operator!(ops, "p", s, { s.pop().ok_or(RpnCalculatorError::NotEnoughOperands)?; Ok(()) });
/// let f = ops.get("p").unwrap();
/// let res = f(&mut stack);
/// assert!(res.is_ok());
/// assert_eq!(0, stack.len());
/// # }
/// ```
#[macro_export]
macro_rules! new_operator {
    ($ops:expr, $name:expr, [ $( $var:ident ),* ], $code:block) => {{
        fn opfn(s: &mut CalcStack) -> CalcResult {
            let i = s.len();
            $(
                let $var: f64;
                if i == 0 {
                    return Err(RpnCalculatorError::NotEnoughOperands);
                } else {
                    $var = s[i - 1];
                }
                let i = i - 1;
            )*;
            let n = s.len() - i;
            if n > 0 {
                for _ in 0..n {
                    s.pop();
                }
            }
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

/// This function builds an operators map with all the default
/// operators the calculator supports. This map can be used to add
/// more operators before creating a new calculator.
/// See
///
/// # Example
/// ```
/// use pprust::rpncalculator::{default_operators, CalcResult};
///
/// let mut ops = default_operators();
/// fn op(s: &mut Vec<f64>) -> CalcResult {
///     s.push(2.0);
///     Ok(())
/// }
/// ops.insert("?", op);
/// ```
pub fn default_operators() -> OperatorsMap {
    let mut ops: OperatorsMap = collections::BTreeMap::new();
    new_operator!(ops, "+", [y, x], { x + y });
    new_operator!(ops, "-", [y, x], { x - y });
    new_operator!(ops, "*", [y, x], { x * y });
    new_operator!(ops, "/", [y, x], { x / y });
    ops
}

/// The calculator
pub struct RpnCalculator {
    stack: CalcStack,
    operators: OperatorsMap,
}

impl RpnCalculator {
    /// Creates a new calculator with default operators
    pub fn new() -> RpnCalculator {
        RpnCalculator { stack: Vec::new(), operators: default_operators() }
    }

    /// Creates a new calculator with the operators passed
    pub fn new_with_operators(operators: OperatorsMap) -> RpnCalculator {
        RpnCalculator { stack: Vec::new(), operators: operators }
    }

    /// Returns the top of the calculator's stack
    pub fn top(&self) -> Option<&f64> {
        self.stack.last()
    }

    /// evaluates an input string and mutates the calculator
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

    fn parse_and_push(&mut self, token: &str) -> CalcResult {
        let value: f64 = token.parse()?;
        self.stack.push(value);
        Ok(())
    }
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
        assert_eq!(2.5, *calc.top().unwrap());
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
        assert_eq!(3.2, *calc.top().unwrap());
        calc.evaluate("X").unwrap();
        assert_eq!(0.0, *calc.top().unwrap());
    }

    #[test]
    fn should_add_two_f64_in_stack() {
        let mut calc = make_calculator();
        calc.evaluate("2.5 3.2 +").unwrap();
        assert_eq!(5.7, *calc.top().unwrap(), "Calcultor's top should be result of addition");
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
        assert_eq!(10.0, *calc.top().unwrap(), "Should have returned value at the top");
    }

    #[test]
    fn should_extend_default_operators_with_operators() {
        let mut calc = make_calculator();
        new_operator!(calc.operators, "?", [], { 10.0 });
        let result = calc.evaluate("? 2 +");
        assert!(result.is_ok(), "Should return ok as input is valid");
        assert_eq!(12.0, *calc.top().unwrap(), "Should have returned result of 10.0 + 2 at the top");
    }

    #[test]
    fn should_be_possible_to_add_operator_that_operates_on_stack() {
        let mut calc = make_calculator();
        new_operator!(calc.operators, "?", s, { s.pop(); Ok(()) });
        let result = calc.evaluate("2 3 ?");
        assert!(result.is_ok());
        assert_eq!(2.0, *calc.top().unwrap(), "top should be popped");
    }

    #[test]
    fn should_not_pop_without_enough_operands() {
        let mut calc = make_calculator();
        calc.evaluate("1.0").expect("Should push to the stack");
        let result = calc.evaluate("+");
        assert!(result.is_err(), "Should return error because '+' expects two operands");
        match result {
            Err(RpnCalculatorError::NotEnoughOperands) => (),
            _ => assert!(false, "Should return NotEnoughOperands error"),
        }
        assert_eq!(1.0, *calc.top().expect("Stack should not be popped since there was not enough operands"),
                   "Stack should not be popped since there was not enough operands");
    }

    fn check_evaluation(input: &str, expected: f64) {
        let mut calc = make_calculator();
        let result = calc.evaluate(input);
        assert!(result.is_ok());
        let result = *calc.top().expect("Should have a result");
        let delta = expected - result;
        let expected_delta = 0.00001;
        assert!(expected_delta > delta, "{} - {} > {}", expected, result, expected_delta);
    }

    #[test]
    fn should_calculate_subtraction() {
        check_evaluation("6 2 -", 4.0);
    }

    #[test]
    fn should_calculate_division() {
        check_evaluation("6 2 /", 3.0);
    }

    #[test]
    fn should_calculate_multiplication() {
        check_evaluation("6 2 *", 12.0);
    }

    #[test]
    fn should_calculate_the_example_from_the_site() {
        check_evaluation("19 2.14 + 4.5 2 4.3 / - *", 85.2974);
    }
}
