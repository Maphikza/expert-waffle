use std::io;

fn main() {
    const NAME: &str = "Siphiwe";
    println!("Welcome to {}'s tests.", NAME);
    let name = get_name("What's your name");
    let prof = get_name("Who is your professor?");

    println!("Hello, {} I am working on cs50!", name);
    println!("My Professors is: {}.", prof);

    println!("Testing the Calculator:");

    let x = get_int("x ");
    let y = get_int("y ");
    let answer = x + y;
    println!("{} + {} = {}", x, y, answer);

    if answer % 2 == 0 {
        println!("{}  is even.", answer);
    } else {
        println!("{} is odd.", answer);
    }
}

fn get_name(string:&str) -> String {

    println!("{}:", string.to_string());

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Data must be a string.");
    
    let name: String = name.trim().parse().expect("No Data input.");

    name
}

fn get_int(string:&str) -> usize {
    println!("{}:", string.to_string());

    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Data must be a string.");
    
    let int_num: usize = num.trim().parse().expect("No Data input.");

    int_num
}