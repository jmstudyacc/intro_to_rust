use chrono::prelude::*;

fn main() {
    let current = Utc::now();
    // print with {:02} to achieve 2 decimal places with leading 0s to pad
    println!(
        "The current time is: {:02}:{:02}:{:02}",
        current.hour(),
        current.minute(),
        current.second()
    );
    println!(
        "It has been, {}, seconds since midnight.",
        current.num_seconds_from_midnight()
    );

    /*
    - DateTime alone (as per the slide) doesn't have .format() available (chrono@0.4.19)
    - I was unclear on the instruction 4 - instruction 5 seems to align with below?
    - Interesting to note - %a needs whitespace or %b crowds it - %b & %e do not need whitespace
    */
    println!("{}", current.format("%a %b%e %T %Y").to_string());
}
