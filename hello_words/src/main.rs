use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    // creating a binding that holds a file location & file
    let path = Path::new("WarAndPeace.txt");

    // opening the file at the location in above binding
    let file = File::open(path).expect("Unable to open file.");

    // assign the result of the match statement to binding
    let mut binding = match file {
        Ok(file) => file,
        Err(_) => panic!("Something went wrong with the file!"),
    };

    // create a new string, this will hold the contents of the file path
    let mut string1 = String::new();

    // read the contents of the file (binding) into the new String variable
    match binding.read_to_string(&mut string1) {
        // any error leads to a panic and print to stderr
        Err(e) => eprintln!("{}", e),
        // a success just continues with no impact
        Ok(_) => (),
    };

    // splitting the string into a vector & collecting - makes it easier to iterate
    let words: Vec<&str> = string1.split(' ').collect();

    // creating a hashmap to store word & word counts
    let mut book: HashMap<&str, i32> = HashMap::new();

    // iterate over the vector containing the words of the file
    for word in words {
        // if word is not present add it with a value of 0
        // if it is present, deref the value and increment it by 1
        *book.entry(word).or_insert(0) += 1;
    }

    // iterate over the hashmap's (key, value) pairs and print out each (k,v) pair
    for (key, value) in &book {
        println!("{} {}", key, value)
    }

    println!(
        "{:?}",
        book.get_key_value("and").expect("Unable to find key.")
    );
}
