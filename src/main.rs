use std::{io, process::exit};

mod config;
mod etsy_calculator;
mod paypal_calculator;
mod shopify_calculator;
mod stripe_calculator;

fn set_config_options() {
    let mut config = config::get_config();
    println!("Markup percentage: {}%", config.markup_percentage);
    println!("Hourly rate: £{}", config.hourly_rate);
    println!("Tax rate: {}%", config.tax_rate);
    println!("VAT: {}%", config.vat);
    println!();

    println!("What do you want to set?");
    println!("1. Markup percentage");
    println!("2. Hourly rate");
    println!("3. Tax rate");
    println!("4. VAT");
    println!("5. Go back");
    let mut option = String::new();
    io::stdin()
        .read_line(&mut option)
        .expect("failed to readline");
    println!();

    match option.trim() {
        "1" => {
            println!("Enter value:");
            let mut value = String::new();
            io::stdin()
                .read_line(&mut value)
                .expect("failed to readline");
            println!();

            config.markup_percentage = value
                .trim()
                .parse::<f32>()
                .expect(format!("Unable to parse {}", value.trim()).as_str());
            config::store_config(&config);
        }
        "2" => {
            println!("Enter value:");
            let mut value = String::new();
            io::stdin()
                .read_line(&mut value)
                .expect("failed to readline");
            println!();

            config.hourly_rate = value
                .trim()
                .parse::<f32>()
                .expect(format!("Unable to parse {}", value.trim()).as_str());
            config::store_config(&config);
        }
        "3" => {
            println!("Enter value:");
            let mut value = String::new();
            io::stdin()
                .read_line(&mut value)
                .expect("failed to readline");
            println!();

            config.tax_rate = value
                .trim()
                .parse::<f32>()
                .expect(format!("Unable to parse {}", value.trim()).as_str());
            config::store_config(&config);
        }
        "4" => {
            println!("Enter value:");
            let mut value = String::new();
            io::stdin()
                .read_line(&mut value)
                .expect("failed to readline");
            println!();

            config.vat = value
                .trim()
                .parse::<f32>()
                .expect(format!("Unable to parse {}", value.trim()).as_str());
            config::store_config(&config);
        }
        "5" => return,
        _ => println!("Invalid input provided"),
    }
}

fn etsy_calculator() {
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

            println!("Are offsite ads going to be used (y/n)?");
            let mut ads = String::new();
            io::stdin().read_line(&mut ads).expect("failed to readline");
            println!();

            etsy_calculator::based_on_sale(
                sale.trim()
                    .parse::<f32>()
                    .expect(format!("Unable to parse {}", sale.trim()).as_str()),
                delivery_costs
                    .trim()
                    .parse::<f32>()
                    .expect(format!("Unable to parse {}", delivery_costs.trim()).as_str()),
                ads.trim() == "y",
            );
        }
        "2" => {
            println!("Enter number of hours it took:");
            let mut hours = String::new();
            io::stdin()
                .read_line(&mut hours)
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

            println!("Are offsite ads going to be used (y/n)?");
            let mut ads = String::new();
            io::stdin().read_line(&mut ads).expect("failed to readline");
            println!();

            etsy_calculator::how_much_to_charge(
                hours
                    .trim()
                    .parse::<f32>()
                    .expect(format!("Unable to parse {}", hours.trim()).as_str()),
                material_costs
                    .trim()
                    .parse::<f32>()
                    .expect(format!("Unable to parse {}", material_costs.trim()).as_str()),
                delivery_costs
                    .trim()
                    .parse::<f32>()
                    .expect(format!("Unable to parse {}", delivery_costs.trim()).as_str()),
                ads.trim() == "y",
            );
        }
        "3" => exit(0),
        _ => {
            println!("Invalid input provided");
        }
    };
    println!();
}

fn paypal_calculator() {
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

            println!("Is this an EEA sale (y/n)?");
            let mut eea = String::new();
            io::stdin().read_line(&mut eea).expect("failed to readline");
            println!();

            let international: String = if !(eea.trim() == "y") {
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

            paypal_calculator::based_on_sale(
                sale.trim()
                    .parse::<f32>()
                    .expect(format!("Unable to parse {}", sale.trim()).as_str()),
                delivery_costs
                    .trim()
                    .parse::<f32>()
                    .expect(format!("Unable to parse {}", delivery_costs.trim()).as_str()),
                eea.trim() == "y",
                international.trim() == "y",
            );
        }
        "2" => {
            println!("Enter number of hours it took:");
            let mut hours = String::new();
            io::stdin()
                .read_line(&mut hours)
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

            println!("Is this an EEA sale (y/n)?");
            let mut eea = String::new();
            io::stdin().read_line(&mut eea).expect("failed to readline");
            println!();

            let international: String = if !(eea.trim() == "y") {
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

            paypal_calculator::how_much_to_charge(
                hours
                    .trim()
                    .parse::<f32>()
                    .expect(format!("Unable to parse {}", hours.trim()).as_str()),
                material_costs
                    .trim()
                    .parse::<f32>()
                    .expect(format!("Unable to parse {}", material_costs.trim()).as_str()),
                delivery_costs
                    .trim()
                    .parse::<f32>()
                    .expect(format!("Unable to parse {}", delivery_costs.trim()).as_str()),
                eea.trim() == "y",
                international.trim() == "y",
            );
        }
        "3" => exit(0),
        _ => {
            println!("Invalid input provided");
        }
    };
    println!();
}

fn shopify_calculator() {
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
                    .parse::<f32>()
                    .expect(format!("Unable to parse {}", sale.trim()).as_str()),
                delivery_costs
                    .trim()
                    .parse::<f32>()
                    .expect(format!("Unable to parse {}", delivery_costs.trim()).as_str()),
                international_or_amex.trim() == "y",
            );
        }
        "2" => {
            println!("Enter number of hours it took:");
            let mut hours = String::new();
            io::stdin()
                .read_line(&mut hours)
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
                hours
                    .trim()
                    .parse::<f32>()
                    .expect(format!("Unable to parse {}", hours.trim()).as_str()),
                material_costs
                    .trim()
                    .parse::<f32>()
                    .expect(format!("Unable to parse {}", material_costs.trim()).as_str()),
                delivery_costs
                    .trim()
                    .parse::<f32>()
                    .expect(format!("Unable to parse {}", delivery_costs.trim()).as_str()),
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

fn stripe_calculator() {
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

            let international: String = if !(eu.trim() == "y") {
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

            stripe_calculator::based_on_sale(
                sale.trim()
                    .parse::<f32>()
                    .expect(format!("Unable to parse {}", sale.trim()).as_str()),
                delivery_costs
                    .trim()
                    .parse::<f32>()
                    .expect(format!("Unable to parse {}", delivery_costs.trim()).as_str()),
                eu.trim() == "y",
                international.trim() == "y",
            );
        }
        "2" => {
            println!("Enter number of hours it took:");
            let mut hours = String::new();
            io::stdin()
                .read_line(&mut hours)
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

            let international: String = if !(eu.trim() == "y") {
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

            stripe_calculator::how_much_to_charge(
                hours
                    .trim()
                    .parse::<f32>()
                    .expect(format!("Unable to parse {}", hours.trim()).as_str()),
                material_costs
                    .trim()
                    .parse::<f32>()
                    .expect(format!("Unable to parse {}", material_costs.trim()).as_str()),
                delivery_costs
                    .trim()
                    .parse::<f32>()
                    .expect(format!("Unable to parse {}", delivery_costs.trim()).as_str()),
                eu.trim() == "y",
                international.trim() == "y",
            );
        }
        "3" => exit(0),
        _ => {
            println!("Invalid input provided");
        }
    };
    println!();
}

fn main() {
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
            "1" => etsy_calculator(),
            "2" => paypal_calculator(),
            "3" => shopify_calculator(),
            "4" => stripe_calculator(),
            "5" => set_config_options(),
            "6" => exit(0),
            _ => {
                println!("Invalid input provided");
                continue;
            }
        };
        println!();
    }
}
