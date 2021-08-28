/*
 Statement:

  A company that works with vehicles wishes to calculate the fuel requirements
  (amount of fuel needed to fill the tanks of all their vehicles) for which
  they have provided us with this calculation scheme.

We would like to create a programme so that they can carry out the calculation
in an automated way.

--- - [CALCULATION SCHEME FOR THE PROGRAM] - --- ---

    1. Home

    2.  [Definition of vehicles]

    3.  Turismos = 32

    4.  Todoterrenos = 11

    5.  [End of definition of vehicles]

    6.  [Definition of tank capacity]

    7.  Capturismos = 40

    8.  Captodot = 65

    9.  [End of tank capacity definition]

    10.  [Calculation of fuel requirements]

    11.  fuel_requirement = Turismos * Capturismos + Todoterrenos * Captodot

    12.  [End of fuel requirements calculation]

    13.  End
*/

/*
Solución:

As programmers we have to treat everything that can vary as variables (redundant).
Will the number of vehicles in the company be constant? In all likelihood no,
they will probably retire some vehicles and buy others as time goes by. In the
proposed scheme we are told that there are 32 cars, but we will generate a
program that asks what is the number of cars and what is the tank capacity of the
cars in order to make a calculation based on variables and not on fixed parameters.

*/
mod helper;
use helper::input;
use std::io::Write;

fn main() {
    let mut stdout = std::io::stdout();

    let turismos: u32 = input(&mut stdout, "Enter the number of cars: ");
    let todoterrenos: u32 = input(&mut stdout, "Enter the number of todoterrenos: ");
    let cap_turismos: u32 = input(
        &mut stdout,
        "Enter the capacity of passenger cars (litres): ",
    );
    let cap_todot: u32 = input(
        &mut stdout,
        "Enter the capacity of the todoterrenos (litres): ",
    );
    let fuel_requirement = turismos * cap_turismos + todoterrenos * cap_todot;

    writeln!(
        stdout,
        "The total fuel requirements are (litres): {}",
        fuel_requirement
    )
    .unwrap();
}
