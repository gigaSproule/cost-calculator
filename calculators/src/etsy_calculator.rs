use super::{Calculator, Charge, ChargeAmount, Config, SaleBreakdown};
use super::{Material, Sale};

const TRANSACTION_FEE: f64 = 0.065;
const PAYMENT_PROCESSING_PERCENTAGE: f64 = 0.04;
const PAYMENT_PROCESSING_FEE: f64 = 0.2;
const OFFSITE_ADS_FEE: f64 = 0.15;
const LISTING_FEE: f64 = 0.16; // Actually $0.20 USD, but need it in GBP
const REGULATOR_OPERATING_FEE: f64 = 0.0032;

pub struct EtsySale {
    pub cost: f64,
    pub delivery_costs: f64,
    pub offsite_ads: bool,
}

impl EtsySale {
    fn get_offsite_ads(&self) -> bool {
        self.offsite_ads
    }
}

impl Sale for EtsySale {
    fn get_cost(&self) -> f64 {
        self.cost
    }

    fn get_delivery_costs(&self) -> f64 {
        self.delivery_costs
    }
}

pub struct EtsyCharge {
    pub number_of_minutes: f64,
    pub material_costs: Vec<Material>,
    pub delivery_costs: f64,
    pub offsite_ads: bool,
}

impl EtsyCharge {
    fn get_offsite_ads(&self) -> bool {
        self.offsite_ads
    }
}

impl Charge for EtsyCharge {
    fn get_number_of_minutes(&self) -> f64 {
        self.number_of_minutes
    }

    fn get_material_costs(&self) -> &Vec<Material> {
        &self.material_costs
    }

    fn get_delivery_costs(&self) -> f64 {
        self.delivery_costs
    }
}

pub struct EtsyCalculator {}

impl Calculator<EtsySale, EtsyCharge> for EtsyCalculator {
    fn based_on_sale(&self, config: &dyn Config, sale: EtsySale) -> SaleBreakdown {
        let vat_multiplier = (config.get_vat() / 100.0) + 1.0;
        let sale_total = sale.cost + sale.delivery_costs;
        let sale_total_with_tax = sale_total * vat_multiplier;
        let transaction_cost = sale_total * TRANSACTION_FEE;
        let payment_processing_cost =
            (sale_total_with_tax * PAYMENT_PROCESSING_PERCENTAGE) + PAYMENT_PROCESSING_FEE;
        let offsite_ads_cost = if sale.offsite_ads {
            sale_total_with_tax * OFFSITE_ADS_FEE
        } else {
            0.0
        };
        let regulatory_operating_fee =
            ((sale_total * REGULATOR_OPERATING_FEE) * 100.0).round() / 100.0;
        let total_fees = transaction_cost
            + payment_processing_cost
            + offsite_ads_cost
            + LISTING_FEE
            + regulatory_operating_fee;
        let tax = sale.cost * (config.get_tax_rate() / 100.0);
        let vat_paid_by_buyer = sale.cost * (config.get_vat() / 100.0);
        let vat_on_seller_fees = total_fees * (config.get_vat() / 100.0);
        let total_fees_with_vat = total_fees + vat_on_seller_fees;

        let revenue = sale.cost - total_fees_with_vat - tax;
        let percentage_kept = (revenue / sale.cost) * 100.0;
        let max_working_hours = revenue / config.get_hourly_rate();

        SaleBreakdown {
            sale: sale.cost,
            delivery_costs: sale.delivery_costs,
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

    fn how_much_to_charge(&self, config: &dyn Config, charge: EtsyCharge) -> ChargeAmount {
        let total_material_costs: f64 = charge
            .material_costs
            .iter()
            .map(|material| material.value)
            .sum();
        let base_charge = ((charge.number_of_minutes / 60.0) * config.get_hourly_rate())
            + total_material_costs
            + charge.delivery_costs;
        let offsite_ads_cost = if charge.offsite_ads {
            base_charge * OFFSITE_ADS_FEE
        } else {
            0.0
        };
        let transaction_cost = base_charge * TRANSACTION_FEE;
        let payment_processing_cost =
            (base_charge * PAYMENT_PROCESSING_PERCENTAGE) + PAYMENT_PROCESSING_FEE;
        let charge = base_charge
            + transaction_cost
            + payment_processing_cost
            + offsite_ads_cost
            + LISTING_FEE;
        let regulatory_operating_fee = ((charge * REGULATOR_OPERATING_FEE) * 100.0).round() / 100.0;

        let markup_percentage = (100.0 + config.get_markup_percentage()) / 100.0;
        let total_to_charge = ((charge + regulatory_operating_fee) * markup_percentage).ceil();
        let with_vat = total_to_charge * ((config.get_vat() / 100.0) + 1.0);

        ChargeAmount {
            total_to_charge,
            with_vat,
        }
    }
}
