use std::{io, process::exit};

mod calculator;
mod config;

fn set_options() {
    let mut config = config::get_config();
    println!("Markup percentage: {}%", config.markup_percentage);
    println!("Hourly rate: £{}", config.hourly_rate);
    println!("Tax rate: {}%", config.tax_rate);
    println!("");

    println!("What do you want to set?");
    println!("1. Markup percentage");
    println!("2. Hourly rate");
    println!("3. Tax rate");
    println!("4. Go back");
    let mut option = String::new();
    io::stdin()
        .read_line(&mut option)
        .expect("failed to readline");
    println!("");

    match option.trim() {
        "1" => {
            println!("Enter value:");
            let mut value = String::new();
            io::stdin()
                .read_line(&mut value)
                .expect("failed to readline");
            println!("");

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
            println!("");

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
            println!("");

            config.tax_rate = value
                .trim()
                .parse::<f32>()
                .expect(format!("Unable to parse {}", value.trim()).as_str());
            config::store_config(&config);
        }
        "4" => return,
        _ => println!("Invalid input provided"),
    }
}

fn main() {
    loop {
        println!("What is your input?");
        println!("1. Cost of sale");
        println!("2. How much to charge?");
        println!("3. Options");
        println!("4. Quit");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to readline");
        println!("");

        match input.trim() {
            "1" => {
                println!("Enter cost of sale:");
                let mut sale = String::new();
                io::stdin()
                    .read_line(&mut sale)
                    .expect("failed to readline");
                println!("");

                println!("Enter cost of delivery:");
                let mut delivery_costs = String::new();
                io::stdin()
                    .read_line(&mut delivery_costs)
                    .expect("failed to readline");
                println!("");

                println!("Are offsite ads going to be used (y/n)?");
                let mut ads = String::new();
                io::stdin().read_line(&mut ads).expect("failed to readline");
                println!("");

                calculator::based_on_sale(
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
                println!("");

                println!("What were the material costs?");
                let mut material_costs = String::new();
                io::stdin()
                    .read_line(&mut material_costs)
                    .expect("failed to readline");
                println!("");

                println!("What were the delivery costs?");
                let mut delivery_costs = String::new();
                io::stdin()
                    .read_line(&mut delivery_costs)
                    .expect("failed to readline");
                println!("");

                println!("Are offsite ads going to be used (y/n)?");
                let mut ads = String::new();
                io::stdin().read_line(&mut ads).expect("failed to readline");
                println!("");

                calculator::how_much_to_charge(
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
            "3" => set_options(),
            "4" => exit(0),
            _ => {
                println!("Invalid input provided");
                continue;
            }
        };
        println!("");
    }
}
