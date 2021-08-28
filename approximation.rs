/*
Since the value of e^x can be calculated by approximation
of the following sum:
  e = 1 + x+x^2/2! + x^3/3! + ….. + x^n/n!

Create a program that takes a value for X and calculates e^x until x^n/n!
(error or approximation) is less than 0.00001

*/

mod helper;
use helper::input;
use std::io::Write;

fn main() {
    let mut stdout = std::io::stdout();
    let x: f32 = input(&mut stdout, "Enter the value of X: ");

    let mut e = 1f32;
    let mut num: f32;
    let mut den = 1f32;
    let mut i = 1f32;

    loop {
        num = x.powf(i); // x ^ i
        den = den * i;
        i += 1.;
        e = e + num / den;

        if num / den < 0.000001 {
            break;
        }
    }

    writeln!(stdout, "`e` raised to `{}` is `{}`", x, e).unwrap();
}
