use std::io;

use crate::config;

pub(crate) fn set_config_options() {
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
                .parse::<f64>()
                .unwrap_or_else(|_| panic!("Unable to parse {}", value.trim()));
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
                .parse::<f64>()
                .unwrap_or_else(|_| panic!("Unable to parse {}", value.trim()));
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
                .parse::<f64>()
                .unwrap_or_else(|_| panic!("Unable to parse {}", value.trim()));
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
                .parse::<f64>()
                .unwrap_or_else(|_| panic!("Unable to parse {}", value.trim()));
            config::store_config(&config);
        }
        "5" => (),
        _ => println!("Invalid input provided"),
    }
}
