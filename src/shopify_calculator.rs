use crate::config;

const PAYMENT_PROCESSING_PERCENTAGE: f32 = 0.02;
const INTERNATIONAL_AMEX_PAYMENT_PROCESSING_PERCENTAGE: f32 = 0.031;
const PAYMENT_PROCESSING_FEE: f32 = 0.25;

pub(crate) fn based_on_sale(sale: f32, delivery_costs: f32, international_or_amex: bool) {
    let config = config::get_config();

    let sale_total = sale + delivery_costs;
    let payment_processing_percentage = if international_or_amex {
        INTERNATIONAL_AMEX_PAYMENT_PROCESSING_PERCENTAGE
    } else {
        PAYMENT_PROCESSING_PERCENTAGE
    };
    let payment_processing_cost =
        (sale_total * payment_processing_percentage) + PAYMENT_PROCESSING_FEE;
    let tax = sale * (config.tax_rate / 100.0);
    let revenue = sale - payment_processing_cost - tax;
    let percentage_kept = (revenue / sale) * 100.0;
    let max_working_hours = revenue / config.hourly_rate;
    println!("Sale: £{:.2}", sale);
    println!("Delivery costs: £{:.2}", delivery_costs);
    println!("Payment processing fee: £{:.2}", payment_processing_cost);
    println!("Tax: £{:.2}", tax);
    println!("Revenue: £{:.2}", revenue);
    println!("Percentage kept: {:.2}%", percentage_kept);
    println!(
        "Max working hours: {}:{:02}",
        max_working_hours as i32,
        ((max_working_hours - ((max_working_hours as i32) as f32)) * 60.0) as i32
    );
}

pub(crate) fn how_much_to_charge(
    number_of_hours: f32,
    material_costs: f32,
    delivery_costs: f32,
    international_or_amex: bool,
) {
    let config = config::get_config();

    let base_charge = (number_of_hours * config.hourly_rate) + material_costs + delivery_costs;
    let payment_processing_percentage = if international_or_amex {
        INTERNATIONAL_AMEX_PAYMENT_PROCESSING_PERCENTAGE
    } else {
        PAYMENT_PROCESSING_PERCENTAGE
    };
    let payment_processing_cost =
        (base_charge * payment_processing_percentage) + PAYMENT_PROCESSING_FEE;
    let charge =
        base_charge + payment_processing_cost;

    let markup_percentage = (100.0 + config.markup_percentage) / 100.0;

    println!("Charge: £{:.0}", (charge * markup_percentage).ceil());
}
