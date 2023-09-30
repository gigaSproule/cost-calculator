use std::{io, process::exit};

mod etsy_cli;
mod options_cli;
mod paypal_cli;
mod shopify_cli;
mod stripe_cli;

pub(crate) fn load_cli() {
    loop {
        println!("What is your input?");
        println!("1. Etsy");
        println!("2. PayPal");
        println!("3. Shopify");
        println!("4. Stripe");
        println!("5. Options");
        println!("6. Quit");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to readline");
        println!();

        match input.trim() {
            "1" => etsy_cli::etsy_calculator(),
            "2" => paypal_cli::paypal_calculator(),
            "3" => shopify_cli::shopify_calculator(),
            "4" => stripe_cli::stripe_calculator(),
            "5" => options_cli::set_config_options(),
            "6" => exit(0),
            _ => {
                println!("Invalid input provided");
                continue;
            }
        };
        println!();
    }
}
