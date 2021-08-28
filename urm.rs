/*
We wish to calculate the distance travelled (m) by a mobile with a constant
speed (m/s) during a time T (Sg), considering that it is a URM (Uniform
Rectilinear Motion).

Solution:
DATA

                                                    Identifiers
  Output
    Distance Travelled (m)                              D
  Input
    Constant Speed (m/s)                                S
    Time (Sg)                                           T

  Start
    Read S
    Read T
    D = S * T
    Write D
  End

*/


mod helper;
use helper::input;
use std::io::Write;

fn main() {
    let mut stdout = std::io::stdout();

    let v: u32 = input(&mut stdout, "Enter the Speed: ");
    let t: u32 = input(&mut stdout, "Enter the Time: ");
    let d = v * t;

    writeln!(stdout, "Distance travelled: {}", d).unwrap();
}
