/*
What and how many are the prime numbers between 1 and 1000?

Algorithm n_primes
Const
   first = 1
   limit = 1000
Var
   cont, i, j: integer
   prime: boolean
Start
  Cont <- 0
  From i = first to limit
    prime <- true
    j <- 2
    while (i > j) and (prime = true)
      If (i mod j = 0) then
        prime <- false
      else
        j <- j + 1
      End if
    End while
    If prime = true then
      write(i, " is prime")
      Cont <- Cont + 1
    End if
  End from
write("Between ", first, " and " , limit, " is ", Cont, " number of primes")
End
*/

fn main() {
    println!("Prime numbers between 1 and 1000.");

    let first: i16 = 2;
    let limit: i16 = 1000;
    let mut cont = 0;

    for i in first..limit {
        let mut primo = true;
        let mut j = 2;
        while i > j && primo {
            if i % j == 0 {
                // since Number(i) and 1 are not considered
                // If another number divides i then it would already have three divisors.
                // three divisors. therefore, it is not prime.
                primo = false;
                break; // For this reason the cycle is broken
            } else {
                j += 1;
            }
        }

        if primo {
            println!("{} is prime", i);
            cont += 1;
        }
    }

    println!("Between {} and {} there are {} prime numbers", first, limit, cont);
}
