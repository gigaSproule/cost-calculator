use std::{io, process::exit};

use crate::calculator::{
    sumup_calculator::{self, PaymentOption, SubscriptionOption},
    Material,
};

pub(crate) fn sumup_calculator() {
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

            println!("Is this paid with a card reader (y/n)?");
            let mut card_reader = String::new();
            io::stdin()
                .read_line(&mut card_reader)
                .expect("failed to readline");
            println!();

            let pos_lite: String = if card_reader.trim() != "y" {
                let mut pos_lite_input = String::new();
                println!("Is this paid with a POS Lite (y/n)?");
                io::stdin()
                    .read_line(&mut pos_lite_input)
                    .expect("failed to readline");
                println!();
                pos_lite_input
            } else {
                "n".to_string()
            };

            let tap_to_pay_iphone: String = if card_reader.trim() != "y" && pos_lite.trim() != "y" {
                let mut tap_to_pay_iphone_input = String::new();
                println!("Is this paid with tap to pay iPhone (y/n)?");
                io::stdin()
                    .read_line(&mut tap_to_pay_iphone_input)
                    .expect("failed to readline");
                println!();
                tap_to_pay_iphone_input
            } else {
                "n".to_string()
            };

            let remote_payment: String = if card_reader.trim() != "y"
                && pos_lite.trim() != "y"
                && tap_to_pay_iphone.trim() != "y"
            {
                let mut remote_payment_input = String::new();
                println!("Is this paid with remote payment (y/n)?");
                io::stdin()
                    .read_line(&mut remote_payment_input)
                    .expect("failed to readline");
                println!();
                remote_payment_input
            } else {
                "n".to_string()
            };

            println!("Do you have a SumUp One subscription (y/n)?");
            let mut sumup_one_subscription = String::new();
            io::stdin()
                .read_line(&mut sumup_one_subscription)
                .expect("failed to readline");
            println!();

            let sale_breakdown = sumup_calculator::based_on_sale(
                sale.trim()
                    .parse::<f64>()
                    .unwrap_or_else(|_| panic!("Unable to parse {}", sale.trim())),
                if card_reader.trim() == "y" {
                    PaymentOption::CardReader
                } else if pos_lite.trim() == "y" {
                    PaymentOption::PosLite
                } else if tap_to_pay_iphone.trim() == "y" {
                    PaymentOption::TapToPayIPhone
                } else if remote_payment.trim() == "y" {
                    PaymentOption::RemotePayment
                } else {
                    PaymentOption::QrCode
                },
                if sumup_one_subscription.trim() == "y" {
                    SubscriptionOption::SumUpOne
                } else {
                    SubscriptionOption::NoContract
                },
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

            println!("Is this paid with a card reader (y/n)?");
            let mut card_reader = String::new();
            io::stdin()
                .read_line(&mut card_reader)
                .expect("failed to readline");
            println!();

            let pos_lite: String = if card_reader.trim() != "y" {
                let mut pos_lite_input = String::new();
                println!("Is this paid with a POS Lite (y/n)?");
                io::stdin()
                    .read_line(&mut pos_lite_input)
                    .expect("failed to readline");
                println!();
                pos_lite_input
            } else {
                "n".to_string()
            };

            let tap_to_pay_iphone: String = if card_reader.trim() != "y" && pos_lite.trim() != "y" {
                let mut tap_to_pay_iphone_input = String::new();
                println!("Is this paid with tap to pay iPhone (y/n)?");
                io::stdin()
                    .read_line(&mut tap_to_pay_iphone_input)
                    .expect("failed to readline");
                println!();
                tap_to_pay_iphone_input
            } else {
                "n".to_string()
            };

            let remote_payment: String = if card_reader.trim() != "y"
                && pos_lite.trim() != "y"
                && tap_to_pay_iphone.trim() != "y"
            {
                let mut remote_payment_input = String::new();
                println!("Is this paid with remote payment (y/n)?");
                io::stdin()
                    .read_line(&mut remote_payment_input)
                    .expect("failed to readline");
                println!();
                remote_payment_input
            } else {
                "n".to_string()
            };

            println!("Do you have a SumUp One subscription (y/n)?");
            let mut sumup_one_subscription = String::new();
            io::stdin()
                .read_line(&mut sumup_one_subscription)
                .expect("failed to readline");
            println!();

            let charge_amount = sumup_calculator::how_much_to_charge(
                minutes
                    .trim()
                    .parse::<f64>()
                    .unwrap_or_else(|_| panic!("Unable to parse {}", minutes.trim())),
                vec![Material {
                    name: String::from("Material costs"),
                    value: material_costs
                        .trim()
                        .parse::<f64>()
                        .unwrap_or_else(|_| panic!("Unable to parse {}", material_costs.trim())),
                }],
                if card_reader.trim() == "y" {
                    PaymentOption::CardReader
                } else if pos_lite.trim() == "y" {
                    PaymentOption::PosLite
                } else if tap_to_pay_iphone.trim() == "y" {
                    PaymentOption::TapToPayIPhone
                } else if remote_payment.trim() == "y" {
                    PaymentOption::RemotePayment
                } else {
                    PaymentOption::QrCode
                },
                if sumup_one_subscription.trim() == "y" {
                    SubscriptionOption::SumUpOne
                } else {
                    SubscriptionOption::NoContract
                },
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
