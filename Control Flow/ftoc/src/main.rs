use std::io;


fn main() {
    println!("Which Temperature scale do you want?");

    let mut input = String::new(); 

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    if input.trim() == "c" {
        println!("Which temperature in celsius?"); 
        
        let mut celsius = String::new();
        io::stdin().read_line(&mut celsius);

        let celsius: i32 = celsius.trim().parse().expect("Please type a number");
        let convertedc: i32 = celsius * 9/5 + 32;
        println!("{} degrees celsius is {} degrees fahrenheit", celsius, convertedc);

    } else if input.trim() == "f" {
        println!("Which temperature in fahrenheit?");

        let mut fahrenheit = String::new();
        io::stdin().read_line(&mut fahrenheit);

        let fahrenheit: i32 = fahrenheit.trim().parse().expect("Please type a number");
        let convertedf: i32 = (fahrenheit - 32) * 5/9;
        println!("{} degrees fahrenheit is {} degrees celsius", fahrenheit, convertedf);

    } else {
        println!("Please enter either \"c\" for \"celsius\" or \"f\" for \"fahrenheit\""); 
    }
}
