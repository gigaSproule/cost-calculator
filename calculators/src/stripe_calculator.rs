use super::{ChargeAmount, Config, Material, SaleBreakdown};

const PAYMENT_PROCESSING_PERCENTAGE: f64 = 0.015;
const EU_PAYMENT_PROCESSING_PERCENTAGE: f64 = 0.025;
const INTERNATIONAL_PAYMENT_PROCESSING_PERCENTAGE: f64 = 0.035;
const PAYMENT_PROCESSING_FEE: f64 = 0.2;

pub fn based_on_sale(
    config: &dyn Config,
    sale: f64,
    delivery_costs: f64,
    eu: bool,
    international: bool,
) -> SaleBreakdown {
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
    let tax = sale * (config.get_tax_rate() / 100.0);
    let revenue = sale - payment_processing_cost - tax;
    let percentage_kept = (revenue / sale) * 100.0;
    let max_working_hours = revenue / config.get_hourly_rate();
    SaleBreakdown {
        sale,
        delivery_costs,
        transaction_cost: 0.0,
        payment_processing_cost,
        offsite_ads_cost: 0.0,
        regulatory_operating_fee: 0.0,
        vat_paid_by_buyer: 0.0,
        vat_on_seller_fees: 0.0,
        total_fees: payment_processing_cost,
        total_fees_with_vat: payment_processing_cost,
        tax,
        revenue,
        percentage_kept,
        max_working_hours,
    }
}

pub fn how_much_to_charge(
    config: &dyn Config,
    number_of_minutes: f64,
    material_costs: Vec<Material>,
    delivery_costs: f64,
    eu: bool,
    international: bool,
) -> ChargeAmount {
    let total_material_costs: f64 = material_costs.iter().map(|material| material.value).sum();
    let base_charge = ((number_of_minutes / 60.0) * config.get_hourly_rate())
        + total_material_costs
        + delivery_costs;
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

    let markup_percentage = (100.0 + config.get_markup_percentage()) / 100.0;

    let total_to_charge = (charge * markup_percentage).ceil();
    ChargeAmount {
        total_to_charge,
        with_vat: total_to_charge * 1.2,
    }
}