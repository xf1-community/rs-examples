/*
A student's simple average needs to be obtained from his or her three midterm
grades.

Solution:
  DATA                     Identifiers
Output
  Input  Average               A
  First  Partial Grade         G1
  Second Partial Grade         G2
  Third  Partial Grade         G3
Start
  Read G1
  Read G2
  Read G3
  P = (N1 + N2 + N3) / 3
  Write A
End
*/

mod helper;
use helper::input;
use std::io::Write;

fn main() {
    let mut stdout = std::io::stdout();
    let mut average = 0i8;
    const GRADES: i8 = 3;

    for i in 1..GRADES + 1 {
        average += input::<i8>(&mut stdout, &format!("Nota {}: ", i));
    }
    average /= GRADES;

    writeln!(stdout, "Simple average: {}", average).unwrap();
}
