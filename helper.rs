use std::fmt::Debug;
use std::io::Write;
use std::str::FromStr;

/// "input" displays a console message and returns the received text converted to "any type".
pub fn input<T: FromStr>(stdout: &mut std::io::Stdout, msg: &str) -> T
where
    <T as FromStr>::Err: Debug,
{
    let mut value = String::new();
    stdout.write_all(&msg.as_bytes()).unwrap(); // Save the message
    stdout.flush().unwrap(); // Write the message to the console
    std::io::stdin().read_line(&mut value).unwrap();
    // take the input and clean it (trim), convert it to T (parse) and if valid return it
    value.trim().parse().unwrap() // or value.trim().parse::<T>().unwrap()
}

/*
Note: Generally Rust infers any type, the "input" function does not have a
"specific type to return", so Rust cannot infer the type of the value to return
and the user must be specific about "what type they want to receive" when using
the "input" function.
*/
