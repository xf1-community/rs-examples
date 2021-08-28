/*
Design a program that reads the value corresponding to a distance in nautical
miles and writes them in metres. Knowing that 1 nautical mile is equal to 1852 metres.

                    Pseudocódigo
PROGRAMME: Miles and Metres
MODULE: Main

START
  DATA:
    CONSTANTS
      NauticalMile Numeric Integer = 1852
    VARIABLES
      miles Integer Numeric Integer
      meters Integer Numeric Integer
  ALGORITHM:
    Type "Distance in miles: "
    Read miles
    meters = miles * NauticalMile
    Write "Distance in meters:", meters
END
*/


mod helper;
use helper::input;
use std::io::Write;

const NAUTICAL_MILE: u32 = 1852;

fn main() {
    let mut stdout = std::io::stdout();
    let miles: u32 = input(&mut stdout, "Distance in miles: ");

    let meters = miles * NAUTICAL_MILE;

    writeln!(stdout, "Distance in metres: {}", meters).unwrap();
}
