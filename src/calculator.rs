const TRANSACTION_FEE: f32 = 0.065;
const PAYMENT_PROCESSING_PERCENTAGE: f32 = 0.04;
const PAYMENT_PROCESSING_FEE: f32 = 0.2;
const OFFSITE_ADS_FEE: f32 = 0.15;
const LISTING_FEE: f32 = 0.15; // Actually $0.20 USD, but need it in GBP
const MINIMUM_WAGE: f32 = 10.42;

pub(crate) fn based_on_sale(sale: f32) {
    let transaction_cost = sale * TRANSACTION_FEE;
    let payment_processing_fee = (sale * PAYMENT_PROCESSING_PERCENTAGE) + PAYMENT_PROCESSING_FEE;
    let offsite_ads_fee = sale * OFFSITE_ADS_FEE;
    let total_fees = transaction_cost + payment_processing_fee + offsite_ads_fee + LISTING_FEE;
    let actual_revenue = sale - total_fees;
    let percentage_kept = (actual_revenue / sale) * 100.0;
    let max_working_hours = actual_revenue / MINIMUM_WAGE;
    println!("Sale: £{:.2}", sale);
    println!("Transaction cost: £{:.2}", transaction_cost);
    println!("Payment processing fee: £{:.2}", payment_processing_fee);
    println!("Offsite ads fee: £{:.2}", offsite_ads_fee);
    println!("Total fees: £{:.2}", total_fees);
    println!("Actual revenue: £{:.2}", actual_revenue);
    println!("Percentage kept: {:.2}%", percentage_kept);
    println!(
        "Max working hours: {}:{}",
        max_working_hours as i32,
        ((max_working_hours - ((max_working_hours as i32) as f32)) * 60.0) as i32
    );
}

pub(crate) fn how_much_to_charge(number_of_hours: f32, markup_percentage: f32, offsite_ads: bool) {
    let base_charge = number_of_hours * MINIMUM_WAGE;
    let ad_charge = if offsite_ads {
        base_charge * OFFSITE_ADS_FEE
    } else {
        0.0
    };

    println!(
        "Charge: £{:.0}",
        ((base_charge + ad_charge) * ((100.0 + markup_percentage) / 100.0)).ceil()
    );
}
