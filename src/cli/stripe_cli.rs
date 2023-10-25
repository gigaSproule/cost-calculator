use std::{io, process::exit};

use crate::calculator::stripe_calculator;

pub(crate) fn stripe_calculator() {
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

            println!("Is this an EU sale (y/n)?");
            let mut eu = String::new();
            io::stdin().read_line(&mut eu).expect("failed to readline");
            println!();

            let international: String = if eu.trim() != "y" {
                let mut international_input = String::new();
                println!("Is this an international sale (y/n)?");
                io::stdin()
                    .read_line(&mut international_input)
                    .expect("failed to readline");
                println!();
                international_input
            } else {
                "n".to_string()
            };

            let sale_breakdown = stripe_calculator::based_on_sale(
                sale.trim()
                    .parse::<f64>()
                    .unwrap_or_else(|_| panic!("Unable to parse {}", sale.trim())),
                delivery_costs
                    .trim()
                    .parse::<f64>()
                    .unwrap_or_else(|_| panic!("Unable to parse {}", delivery_costs.trim())),
                eu.trim() == "y",
                international.trim() == "y",
            );
            println!("Sale: £{:.2}", sale_breakdown.sale);
            println!("Delivery costs: £{:.2}", sale_breakdown.delivery_costs);
            println!(
                "Payment processing fee: £{:.2}",
                sale_breakdown.payment_processing_cost
            );
            println!("Tax: £{:.2}", sale_breakdown.tax);
            println!("Revenue: £{:.2}", sale_breakdown.revenue);
            println!("Percentage kept: {:.2}%", sale_breakdown.percentage_kept);
            println!(
                "Max working hours: {}:{:02}",
                sale_breakdown.max_working_hours as i64,
                ((sale_breakdown.max_working_hours
                    - ((sale_breakdown.max_working_hours as i64) as f64))
                    * 60.0) as i64
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

            println!("Is this an EU sale (y/n)?");
            let mut eu = String::new();
            io::stdin().read_line(&mut eu).expect("failed to readline");
            println!();

            let international: String = if eu.trim() != "y" {
                let mut international_input = String::new();
                println!("Is this an international sale (y/n)?");
                io::stdin()
                    .read_line(&mut international_input)
                    .expect("failed to readline");
                println!();
                international_input
            } else {
                "n".to_string()
            };

            let charge_amount = stripe_calculator::how_much_to_charge(
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
                eu.trim() == "y",
                international.trim() == "y",
            );
            println!(
                "Charge: £{:.2} (with VAT £{:.2})",
                charge_amount.total_to_charge, charge_amount.with_vat
            );
        }
        "3" => exit(0),
        _ => {
            println!("Invalid input provided");
        }
    };
    println!();
}
