use crate::config;

const TRANSACTION_FEE: f32 = 0.065;
const PAYMENT_PROCESSING_PERCENTAGE: f32 = 0.04;
const PAYMENT_PROCESSING_FEE: f32 = 0.2;
const OFFSITE_ADS_FEE: f32 = 0.15;
const LISTING_FEE: f32 = 0.15; // Actually $0.20 USD, but need it in GBP

pub(crate) fn based_on_sale(sale: f32, delivery_costs: f32, offsite_ads: bool) {
    let config = config::get_config();

    let sale_total = sale + delivery_costs;
    let transaction_cost = sale_total * TRANSACTION_FEE;
    let payment_processing_cost =
        (sale_total * PAYMENT_PROCESSING_PERCENTAGE) + PAYMENT_PROCESSING_FEE;
    let offsite_ads_cost = if offsite_ads {
        sale_total * OFFSITE_ADS_FEE
    } else {
        0.0
    };
    let total_fees = transaction_cost + payment_processing_cost + offsite_ads_cost + LISTING_FEE;
    let tax = sale * (config.tax_rate / 100.0);
    let revenue = sale - total_fees - tax;
    let percentage_kept = (revenue / sale) * 100.0;
    let max_working_hours = revenue / config.hourly_rate;
    println!("Sale: £{:.2}", sale);
    println!("Delivery costs: £{:.2}", delivery_costs);
    println!("Transaction cost: £{:.2}", transaction_cost);
    println!("Payment processing fee: £{:.2}", payment_processing_cost);
    println!("Offsite ads fee: £{:.2}", offsite_ads_cost);
    println!("Total fees: £{:.2}", total_fees);
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
    offsite_ads: bool,
) {
    let config = config::get_config();

    let base_charge = (number_of_hours * config.hourly_rate) + material_costs + delivery_costs;
    let offsite_ads_cost = if offsite_ads {
        base_charge * OFFSITE_ADS_FEE
    } else {
        0.0
    };
    let transaction_cost = base_charge * TRANSACTION_FEE;
    let payment_processing_cost =
        (base_charge * PAYMENT_PROCESSING_PERCENTAGE) + PAYMENT_PROCESSING_FEE;
    let charge =
        base_charge + transaction_cost + payment_processing_cost + offsite_ads_cost + LISTING_FEE;

    let markup_percentage = (100.0 + config.markup_percentage) / 100.0;

    println!("Charge: £{:.0}", (charge * markup_percentage).ceil());
}
