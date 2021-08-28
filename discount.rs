/*
Design a program that writes the discounted percentage on a purchase,
by entering on the keyboard the price of the rate and the price paid.


PROGRAM: Discount
MODULE: Main

START
  DATA:
    VARIABLES
      Tariff Whole Numeric
      Price Whole Numeric
      Discount Numeric Integer
      DP Numeric Actual
  ALGORITHM:
    Read Tariff
    Read Price
    Discount = Tariff - Price
    DP = Discount * 100 / Tariff
    Write "Discount percentage:", DP
FIN
*/

mod helper;
use helper::input;
use std::io::Write;

fn main() {
    let mut stdout = std::io::stdout();

    let tariff: i32 = input(&mut stdout, "Tariff: ");
    let price: i32 = input(&mut stdout, "Price: ");
    let discount = tariff - price;
    let dp = discount * 100 / tariff;

    writeln!(stdout, "Percentage discount: {}", dp).unwrap();
}
