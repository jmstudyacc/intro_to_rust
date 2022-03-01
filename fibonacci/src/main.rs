use std::collections::HashMap;

fn fib(num: u128, results: &mut HashMap<u128, u128>) -> u128 {
    match num {
        // base cases to exit out of recursion - prevent infinite recursion
        // assuming the classical sequence
        0 => 0,
        1 => 0,
        2 => 1,
        num => {
            // check if the HashMap already contains the value - computation not required
            if results.contains_key(&num) {
                // result is already present - retrieve the result from the hashmap - unwrap() as get() returns an Option<>
                *results.get(&num).unwrap()
            }
            // num is not present in table - computation required
            else {
                // .overflowing_add() on u128 required as u128 will overflow around iteration sequence ~186
                let value = fib(num - 1, results).overflowing_add(fib(num - 2, results));
                results.insert(num, value.0);
                value.0
            }
        }
    }
}

fn main() {
    // creating a HashMap to store the results of the sequence to improve speed
    let mut results: HashMap<u128, u128> = HashMap::new();

    // getting first arg from the user - not the program itself
    let mut arg = std::env::args().nth(1);

    // calculating the length of the args provided to enable check below
    let args_len = std::env::args().len();

    //  matching on the value in nth(1)
    match args_len {
        3 => {
            /*
            bind the value of .nth(1) to num1 -
            - 1st unwrap() - .nth() returns an Option<Self::Item>
            - parse() - user input is always a String, even when representing an integer
            - 2nd unwrap() - parse() returns a Result<F, Err>
            */
            let num1: u128 = arg.unwrap().parse().unwrap();

            // rebind the value of arg to .nth(2)
            arg = std::env::args().nth(2);
            // repeat as with num1
            let num2: u128 = arg.unwrap().parse().unwrap();

            println!("i   |   Result");
            println!("--------------");
            // for loop starts from num1 to num2 (parsed user input variables)
            for i in num1..num2 {
                // print the iteration and the result
                println!("{}   |   {}", i, fib(i, &mut results));
            }
        }
        // if the length of the args passed is not 3 then error
        _ => eprintln!(
            "\nERROR: Please try again, remember you should pass 2 args when running 'cargo run 'Integer' 'Integer'.\n"
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib_logic_test() {
        let mut test_results = HashMap::new();
        for i in 0..=5 {
            println!("{}   |   {}", i, fib(i, &mut test_results));
        }
    }
}
