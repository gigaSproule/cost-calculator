use crate::config;

use super::{ChargeAmount, SaleBreakdown};

const PAYMENT_PROCESSING_PERCENTAGE: f64 = 0.015;
const EU_PAYMENT_PROCESSING_PERCENTAGE: f64 = 0.025;
const INTERNATIONAL_PAYMENT_PROCESSING_PERCENTAGE: f64 = 0.035;
const PAYMENT_PROCESSING_FEE: f64 = 0.2;

pub(crate) fn based_on_sale(
    sale: f64,
    delivery_costs: f64,
    eu: bool,
    international: bool,
) -> SaleBreakdown {
    let config = config::get_config();

    let sale_total = sale + delivery_costs;
    let payment_processing_percentage = if eu {
        EU_PAYMENT_PROCESSING_PERCENTAGE
    } else if international {
        INTERNATIONAL_PAYMENT_PROCESSING_PERCENTAGE
    } else {
        PAYMENT_PROCESSING_PERCENTAGE
    };
    let payment_processing_cost =
        (sale_total * payment_processing_percentage) + PAYMENT_PROCESSING_FEE;
    let tax = sale * (config.tax_rate / 100.0);
    let revenue = sale - payment_processing_cost - tax;
    let percentage_kept = (revenue / sale) * 100.0;
    let max_working_hours = revenue / config.hourly_rate;
    SaleBreakdown {
        sale,
        delivery_costs,
        transaction_cost: 0.0,
        payment_processing_cost,
        offsite_ads_cost: 0.0,
        regulatory_operating_fee: 0.0,
        vat_paid_by_buyer: 0.0,
        vat_on_seller_fees: 0.0,
        total_fees: 0.0,
        total_fees_with_vat: 0.0,
        tax,
        revenue,
        percentage_kept,
        max_working_hours,
    }
}

pub(crate) fn how_much_to_charge(
    number_of_minutes: f64,
    material_costs: f64,
    delivery_costs: f64,
    eu: bool,
    international: bool,
) -> ChargeAmount {
    let config = config::get_config();

    let base_charge =
        ((number_of_minutes / 60.0) * config.hourly_rate) + material_costs + delivery_costs;
    let payment_processing_percentage = if eu {
        EU_PAYMENT_PROCESSING_PERCENTAGE
    } else if international {
        INTERNATIONAL_PAYMENT_PROCESSING_PERCENTAGE
    } else {
        PAYMENT_PROCESSING_PERCENTAGE
    };
    let payment_processing_cost =
        (base_charge * payment_processing_percentage) + PAYMENT_PROCESSING_FEE;
    let charge = base_charge + payment_processing_cost;

    let markup_percentage = (100.0 + config.markup_percentage) / 100.0;

    let total_to_charge = (charge * markup_percentage).ceil();
    ChargeAmount {
        total_to_charge,
        with_vat: total_to_charge * 1.2,
    }
}
