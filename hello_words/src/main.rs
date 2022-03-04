use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let path = Path::new("WarAndPeace.txt");
    let file = File::open(path);

    let mut binding = match file {
        Ok(file) => file,
        Err(_) => panic!(),
    };

    let mut string1 = String::new();
    match binding.read_to_string(&mut string1) {
        Err(e) => eprintln!("{}", e),
        Ok(_) => (),
    };

    let words: Vec<&str> = string1.split(' ').collect();

    let mut book: HashMap<&str, i32> = HashMap::new();

    for word in words {
        *book.entry(word).or_insert(0) += 1;
        // let mut count = book.entry(word).or_insert(0);
        // *count += 1;
    }

    for (key, value) in &book {
        println!("{} {}", key, value)
    }

    println!(
        "{:?}",
        book.get_key_value("and").expect("Unable to find key.")
    );
}
