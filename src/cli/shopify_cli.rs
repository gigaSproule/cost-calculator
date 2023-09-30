use std::{io, process::exit};

use crate::calculator::shopify_calculator;

pub(crate) fn shopify_calculator() {
    println!("What is you want to do?");
    println!("1. Cost of sale");
    println!("2. How much to charge?");
    println!("3. Go back");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to readline");
    println!();
    match input.trim() {
        "1" => {
            println!("Enter cost of sale:");
            let mut sale = String::new();
            io::stdin()
                .read_line(&mut sale)
                .expect("failed to readline");
            println!();

            println!("Enter cost of delivery:");
            let mut delivery_costs = String::new();
            io::stdin()
                .read_line(&mut delivery_costs)
                .expect("failed to readline");
            println!();

            println!("Is this an international sale or using AmEx (y/n)?");
            let mut international_or_amex = String::new();
            io::stdin()
                .read_line(&mut international_or_amex)
                .expect("failed to readline");
            println!();

            shopify_calculator::based_on_sale(
                sale.trim()
                    .parse::<f64>()
                    .unwrap_or_else(|_| panic!("Unable to parse {}", sale.trim())),
                delivery_costs
                    .trim()
                    .parse::<f64>()
                    .unwrap_or_else(|_| panic!("Unable to parse {}", delivery_costs.trim())),
                international_or_amex.trim() == "y",
            );
        }
        "2" => {
            println!("Enter number of minutes it took:");
            let mut minutes = String::new();
            io::stdin()
                .read_line(&mut minutes)
                .expect("failed to readline");
            println!();

            println!("What were the material costs?");
            let mut material_costs = String::new();
            io::stdin()
                .read_line(&mut material_costs)
                .expect("failed to readline");
            println!();

            println!("What were the delivery costs?");
            let mut delivery_costs = String::new();
            io::stdin()
                .read_line(&mut delivery_costs)
                .expect("failed to readline");
            println!();

            println!("Is this an international sale or using AmEx (y/n)?");
            let mut international_or_amex = String::new();
            io::stdin()
                .read_line(&mut international_or_amex)
                .expect("failed to readline");
            println!();

            shopify_calculator::how_much_to_charge(
                minutes
                    .trim()
                    .parse::<f64>()
                    .unwrap_or_else(|_| panic!("Unable to parse {}", minutes.trim())),
                material_costs
                    .trim()
                    .parse::<f64>()
                    .unwrap_or_else(|_| panic!("Unable to parse {}", material_costs.trim())),
                delivery_costs
                    .trim()
                    .parse::<f64>()
                    .unwrap_or_else(|_| panic!("Unable to parse {}", delivery_costs.trim())),
                international_or_amex.trim() == "y",
            );
        }
        "3" => exit(0),
        _ => {
            println!("Invalid input provided");
        }
    };
    println!();
}
