/*
 Statement:

 Transform the pseudocode shown below into code,
 corresponding to the calculation of the volume of a cylinder given its height and diameter.

  1. Start

  2. Show "Enter the diameter, in metres" : Ask D

  3. Show "Enter the height, in metres" : Ask H

  4.  R = D/2

  5.  Pi = 3.141593

  6.  V = Pi * (R ^ 2) * H

  7.  Show "The volume of the cylinder is", V, "cubic metres".

  8.  End

Solution
  Instead of V = Pi * (R ^ 2) * H we can also use
  V = Pi * (D/2) ^ 2 * H or V = Pi * ((D ^ 2)/4) * H with the same result as they
  are different expressions of the same calculation.
*/

mod helper;
use helper::input;
use std::io::Write;

const PI: f32 = 3.141593;

fn main() {
    let mut stdout = std::io::stdout();
    let d: f32 = input(&mut stdout, "Enter the diameter in metres: ");
    let h: f32 = input(&mut stdout, "Enter the height in metres: ");

    let v = {
        let r = d / 2.0;
        PI * r.powf(2.0) * h
    };

    writeln!(stdout, "The volume of the cylinder is {} cubic metres", v).unwrap();
}
