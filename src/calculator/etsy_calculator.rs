use crate::{
    calculator::{ChargeAmount, SaleBreakdown},
    config,
};

use super::Material;

const TRANSACTION_FEE: f64 = 0.065;
const PAYMENT_PROCESSING_PERCENTAGE: f64 = 0.04;
const PAYMENT_PROCESSING_FEE: f64 = 0.2;
const OFFSITE_ADS_FEE: f64 = 0.15;
const LISTING_FEE: f64 = 0.16; // Actually $0.20 USD, but need it in GBP
const REGULATOR_OPERATING_FEE: f64 = 0.0032;

pub(crate) fn based_on_sale(sale: f64, delivery_costs: f64, offsite_ads: bool) -> SaleBreakdown {
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

    SaleBreakdown {
        sale,
        delivery_costs,
        transaction_cost,
        payment_processing_cost,
        offsite_ads_cost,
        regulatory_operating_fee,
        vat_paid_by_buyer,
        vat_on_seller_fees,
        total_fees,
        total_fees_with_vat,
        tax,
        revenue,
        percentage_kept,
        max_working_hours,
    }
}

pub(crate) fn how_much_to_charge(
    number_of_minutes: f64,
    material_costs: Vec<Material>,
    delivery_costs: f64,
    offsite_ads: bool,
) -> ChargeAmount {
    let config = config::get_config();

    let total_material_costs: f64 = material_costs.iter().map(|material| material.value).sum();
    let base_charge =
        ((number_of_minutes / 60.0) * config.hourly_rate) + total_material_costs + delivery_costs;
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

    ChargeAmount {
        total_to_charge,
        with_vat,
    }
}
