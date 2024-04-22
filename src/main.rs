use random_str as random;
use std::io::prelude::*;
use std::fs::File;

// random password generator in Rust
// I know it's basic but i have to start somewhere you know

fn main() {
  
    // open the text file with my sexy logo in it
    let mut file = File::open("/home/pizza/projects/random_password_generator/art").expect("Can't find the file!");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Can't read the file!");
    println!("{}", content);

    // some variables for the random password
    let lowercase = true;
    let uppercase = true;
    let length = 16;
    let numbers = true;
    let symbols = true;

    // spit it out to the screen
    let ran_pass = random::get_string(length, lowercase, uppercase, numbers, symbols);
    println!("Random password: {}", ran_pass);

}

        

    




