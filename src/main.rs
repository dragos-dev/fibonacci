use std::io;

fn main() {
    loop {
        let mut line = String::new();

        io::stdin()
            .read_line(&mut line)
            .expect("Invalid input");

        let number: u128 = match line.parse() {
            Ok(n) => if n > 1 { n } else {
                println!("Enter a number greater than 1.");
                continue
            },
            Err(err) => {
                println!("Error: {}", err);
                continue
            }
        };

    }
}

