/*
Statement:
  Transform the pseudocode shown below into code corresponding to providing the
  average price of a product, calculated from its price in three different
  establishments. From the price of the product in three different establishments.
  The prices for the calculation are entered by the user.

    1. Start

    2. Display "Enter the price of the product in shop number 1, in Euro" : Order price1.
    1, in Euro" : Ask price1

    3. Display "Enter the price of the product in shop number 2, in euros" : Order price2.
    2, in Euro" : Ask price2

    4.  Display "Enter the price of the product in shop number 3, in Euro" : Ask for price2 4.
    3, in Euro" : Ask for price3

    5. average = (price1 + price2 + price3) / 3

    6.  Display "The average price of the product is", average, "Euro".

    7.  End

Solution:
  The exercise can be solved using an array of prices or using simple  variables.
  We will solve it with simple variables.
*/

mod helper;
use helper::input;
use std::io::Write;

fn main() {
    let mut stdout = std::io::stdout();

    let price1: f32 = input(
        &mut stdout,
        "Enter the price of the product in shop number 1, in euro: ",
    );
    let price2: f32 = input(
        &mut stdout,
        "Enter the price of the product in shop number 2, in euro: ",
    );
    let price3: f32 = input(
        &mut stdout,
        "Enter the price of the product in shop number 3, in euro: ",
    );

    let average = (price1 + price2 + price3) / 3.0;

    writeln!(stdout, "The average price of the product is: € {}", average).unwrap();
}
