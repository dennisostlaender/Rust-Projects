fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter > 1000 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
}
