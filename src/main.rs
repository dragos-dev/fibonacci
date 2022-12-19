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

        let mut first_value: u128 = 1;
        let mut second_value: u128 = 1;

        for _ in (0..number -2).rev() {
            sum_value = first_value + second_value;
            first_value = second_value;
            second_value = sum_value;
        }
    }
}

