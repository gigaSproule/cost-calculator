use crate::config;

const TRANSACTION_FEE: f32 = 0.065;
const PAYMENT_PROCESSING_PERCENTAGE: f32 = 0.04;
const PAYMENT_PROCESSING_FEE: f32 = 0.2;
const OFFSITE_ADS_FEE: f32 = 0.15;
const LISTING_FEE: f32 = 0.16; // Actually $0.20 USD, but need it in GBP
const REGULATOR_OPERATING_FEE: f32 = 0.0032;

pub(crate) fn based_on_sale(sale: f32, delivery_costs: f32, offsite_ads: bool) {
    let config = config::get_config();

    let vat_multiplier = (config.vat / 100.0) + 1.0;
    let sale_total = sale + delivery_costs;
    let sale_total_with_tax = sale_total * vat_multiplier;
    let transaction_cost = sale_total * TRANSACTION_FEE;
    let payment_processing_cost =
        (sale_total_with_tax * PAYMENT_PROCESSING_PERCENTAGE) + PAYMENT_PROCESSING_FEE;
    let offsite_ads_cost = if offsite_ads {
        sale_total_with_tax * OFFSITE_ADS_FEE
    } else {
        0.0
    };
    let regulatory_operating_fee = ((sale_total * REGULATOR_OPERATING_FEE) * 100.0).round() / 100.0;
    let total_fees = transaction_cost
        + payment_processing_cost
        + offsite_ads_cost
        + LISTING_FEE
        + regulatory_operating_fee;
    let tax = sale * (config.tax_rate / 100.0);
    let vat_paid_by_buyer = sale * (config.vat / 100.0);
    let vat_on_seller_fees = total_fees * (config.vat / 100.0);
    let total_fees_with_vat = total_fees + vat_on_seller_fees;

    let revenue = sale - total_fees_with_vat - tax;
    let percentage_kept = (revenue / sale) * 100.0;
    let max_working_hours = revenue / config.hourly_rate;

    println!("Sale: £{:.2}", sale);
    println!("Delivery costs: £{:.2}", delivery_costs);
    println!("Transaction cost: £{:.2}", transaction_cost);
    println!("Payment processing fee: £{:.2}", payment_processing_cost);
    println!("Offsite ads fee: £{:.2}", offsite_ads_cost);
    println!("Regulatory operating fee: £{:2}", regulatory_operating_fee);
    println!("VAT paid by buyer: £{:.2}", vat_paid_by_buyer);
    println!("VAT on seller fees: £{:.2}", vat_on_seller_fees);
    println!("Total fees: £{:.2}", total_fees);
    println!("Total fees with VAT: £{:.2}", total_fees_with_vat);
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
    let regulatory_operating_fee = ((charge * REGULATOR_OPERATING_FEE) * 100.0).round() / 100.0;

    let markup_percentage = (100.0 + config.markup_percentage) / 100.0;
    let total_to_charge = ((charge + regulatory_operating_fee) * markup_percentage).ceil();
    let with_vat = total_to_charge * ((config.vat / 100.0) + 1.0);

    println!(
        "Charge: £{:.0} (with VAT £{:.0})",
        total_to_charge, with_vat
    );
}
