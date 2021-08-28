/*
Design a program that prompts for two integers and displays their addition,
subtraction, multiplication, division and the remainder (modulo) of the division.
If the operation is not commutative, the result will also be displayed by
inverting the operators.

PROGRAMME: Arithmetic_operations
  MODULE: Main
START
  DATA:
    VARIABLES
    num1 Numeric Integer
    num2 Numeric Integer
  ALGORITHM:
    Read num1, num2
    Write "n1 + n2 = ", num1+num2
    Write "n1 - n2 = ", num1-num2
    Write "n2 - n1 = ", num2-num1
    Write "n1 * n2 = ", num1*num2
    Write "n1 / n2 = ", num1/num2
    Type "n2 / n1 = ", num2/num1
    Type "n1 mod n2 = ", num1 mod num2
    Write "n2 mod n1 = ", num2 mod num1
END

*/

mod helper;
use helper::input;

fn main() {
    let mut stdout = std::io::stdout();
    let (num1, num2): (i32, i32) = (input(&mut stdout, "Number One (n1): "), input(&mut stdout, "Number Two (n2): "));
    println!("n1 + n2 = {}", num1 + num2);
    println!("n1 - n2 = {}", num1 - num2);
    println!("n2 - n1 = {}", num2 - num1);
    println!("n1 * n2 = {}", num1 * num2);
    println!("n1 / n2 = {}", num1 / num2);
    println!("n2 / n1 = {}", num2 / num1);
    println!("n1 % n2 = {}", num1 % num2);
    println!("n2 % n1 = {}", num2 % num1);
}
