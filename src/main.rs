use std::{io, process::exit};

mod calculator;

fn main() {
    loop {
        let mut input = String::new();
        println!("What is your input?");
        println!("1. Cost of sale");
        println!("2. How much to charge?");
        println!("3. Quit");
        io::stdin()
            .read_line(&mut input)
            .expect("failed to readline");
        println!("");

        match input.trim() {
            "1" => {
                let mut sale = String::new();
                println!("Enter cost of sale:");
                io::stdin()
                    .read_line(&mut sale)
                    .expect("failed to readline");
                println!("");

                calculator::based_on_sale(
                    sale.trim()
                        .parse::<f32>()
                        .expect(format!("Unable to parse {}", sale.trim()).as_str()),
                );
            }
            "2" => {
                let mut hours = String::new();
                println!("Enter number of hours it took:");
                io::stdin()
                    .read_line(&mut hours)
                    .expect("failed to readline");
                println!("");

                let mut markup = String::new();
                println!("What percentage markup?");
                io::stdin()
                    .read_line(&mut markup)
                    .expect("failed to readline");
                println!("");

                let mut ads = String::new();
                println!("Are offsite ads going to be used (y/n)?");
                io::stdin().read_line(&mut ads).expect("failed to readline");
                println!("");

                calculator::how_much_to_charge(
                    hours
                        .trim()
                        .parse::<f32>()
                        .expect(format!("Unable to parse {}", hours.trim()).as_str()),
                    markup
                        .trim()
                        .parse::<f32>()
                        .expect(format!("Unable to parse {}", markup.trim()).as_str()),
                    ads.trim() == "y",
                );
            }
            "3" => exit(0),
            _ => {
                println!("Invalid input provided");
                continue;
            }
        };
        println!("");
    }
}
