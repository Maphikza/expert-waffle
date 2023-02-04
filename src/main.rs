use std::io;

fn main() {
    let name = get_name("What's your name");
    let prof = get_name("Who is your professor?");

    println!("Hello, {} I am working on cs50!", name);
    println!("My Professors is: {}.", prof);

    println!("Testing the Calculator:");

    let x = get_int("x ");
    let y = get_int("y ");
    let answer = x + y;
    println!("{} + {} = {}", x, y, answer)
}

fn get_name(string:&str) -> String {

    println!("{}:", string.to_string());

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("No Data Found.");
    
    let name: String = name.trim().parse().expect("No Data input.");

    name
}

fn get_int(string:&str) -> i8 {
    println!("{}:", string.to_string());

    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("No Data Found.");
    
    let int_num: i8 = num.trim().parse().expect("No Data input.");

    int_num
}