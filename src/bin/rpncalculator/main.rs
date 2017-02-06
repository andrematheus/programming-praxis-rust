#[macro_use]
extern crate pprust;
use pprust::rpncalculator::*;
use std::io::prelude::*;
use std::io;

fn repl_step(calc: &mut RpnCalculator) -> CalcResult {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Fudeu");
    calc.evaluate(&input)
}

fn main() {
    let mut ops = default_operators();
    new_operator!(ops, "q", _s, { Result::Err(RpnCalculatorError::Quit) });
    let mut calc = RpnCalculator::new_with_operators(ops);
    loop {
        io::stdout().flush().ok().expect("Could not flush stdout");
        let res = repl_step(&mut calc);
        match res {
            Result::Err(RpnCalculatorError::Quit) => break,
            Result::Ok(_) => {
                println!("{}", *calc.top().unwrap());
            }
            Result::Err(x) => {
                println!("Erro: {:?}", x);
                break;
            }
        }
    }
}
