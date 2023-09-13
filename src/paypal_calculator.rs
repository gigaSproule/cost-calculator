use crate::config;

const PAYMENT_PROCESSING_PERCENTAGE: f32 = 0.015;
const EEA_PAYMENT_PROCESSING_PERCENTAGE: f32 = 0.025;
const INTERNATIONAL_PAYMENT_PROCESSING_PERCENTAGE: f32 = 0.035;
const PAYMENT_PROCESSING_FEE: f32 = 0.3;

pub(crate) fn based_on_sale(sale: f32, delivery_costs: f32, eea: bool, international: bool) {
    let config = config::get_config();

    let sale_total = sale + delivery_costs;
    let additional_payment_processing_percentage = if eea {
        EEA_PAYMENT_PROCESSING_PERCENTAGE
    } else if international {
        INTERNATIONAL_PAYMENT_PROCESSING_PERCENTAGE
    } else {
        0.0
    };
    let base_processing_payment =
        (sale_total * PAYMENT_PROCESSING_PERCENTAGE) + PAYMENT_PROCESSING_FEE;
    let additional_processing_payment = base_processing_payment * additional_payment_processing_percentage;
    let payment_processing_cost = base_processing_payment + additional_processing_payment;
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
    eea: bool,
    international: bool,
) {
    let config = config::get_config();

    let base_charge = (number_of_hours * config.hourly_rate) + material_costs + delivery_costs;
    let additional_payment_processing_percentage = if eea {
        EEA_PAYMENT_PROCESSING_PERCENTAGE
    } else if international {
        INTERNATIONAL_PAYMENT_PROCESSING_PERCENTAGE
    } else {
        0.0
    };
    let base_processing_payment =
        (base_charge * PAYMENT_PROCESSING_PERCENTAGE) + PAYMENT_PROCESSING_FEE;
    let additional_processing_payment = base_processing_payment * additional_payment_processing_percentage;
    let payment_processing_cost = base_processing_payment + additional_processing_payment;
    let charge =
        base_charge + payment_processing_cost;

    let markup_percentage = (100.0 + config.markup_percentage) / 100.0;

    println!("Charge: £{:.0}", (charge * markup_percentage).ceil());
}
