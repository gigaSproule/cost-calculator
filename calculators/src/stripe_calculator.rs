use super::{Calculator, Charge, ChargeAmount, Config, Material, Sale, SaleBreakdown};

const PAYMENT_PROCESSING_PERCENTAGE: f64 = 0.015;
const EU_PAYMENT_PROCESSING_PERCENTAGE: f64 = 0.025;
const INTERNATIONAL_PAYMENT_PROCESSING_PERCENTAGE: f64 = 0.035;
const PAYMENT_PROCESSING_FEE: f64 = 0.2;

#[derive(Debug)]
pub enum StripeLocation {
    Local,
    EU,
    International,
}

pub struct StripeSale {
    pub cost: f64,
    pub delivery_costs: f64,
    pub location: StripeLocation,
}

impl StripeSale {
    fn get_location(&self) -> &StripeLocation {
        &self.location
    }
}

impl Sale for StripeSale {
    fn get_cost(&self) -> f64 {
        self.cost
    }

    fn get_delivery_costs(&self) -> f64 {
        self.delivery_costs
    }
}

pub struct StripeCharge {
    pub number_of_minutes: f64,
    pub material_costs: Vec<Material>,
    pub delivery_costs: f64,
    pub location: StripeLocation,
}

impl StripeCharge {
    fn get_location(&self) -> &StripeLocation {
        &self.location
    }
}

impl Charge for StripeCharge {
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

pub struct StripeCalculator {}

impl Calculator<StripeSale, StripeCharge> for StripeCalculator {
    fn based_on_sale(&self, config: &dyn Config, sale: StripeSale) -> SaleBreakdown {
        let sale_total = sale.cost + sale.delivery_costs;
        let payment_processing_percentage = match sale.location {
            StripeLocation::EU => EU_PAYMENT_PROCESSING_PERCENTAGE,
            StripeLocation::International => INTERNATIONAL_PAYMENT_PROCESSING_PERCENTAGE,
            _ => PAYMENT_PROCESSING_PERCENTAGE,
        };
        let payment_processing_cost =
            (sale_total * payment_processing_percentage) + PAYMENT_PROCESSING_FEE;
        let tax = sale.cost * (config.get_tax_rate() / 100.0);
        let revenue = sale.cost - payment_processing_cost - tax;
        let percentage_kept = (revenue / sale.cost) * 100.0;
        let max_working_hours = revenue / config.get_hourly_rate();
        SaleBreakdown {
            sale: sale.cost,
            delivery_costs: sale.delivery_costs,
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

    fn how_much_to_charge(&self, config: &dyn Config, charge: StripeCharge) -> ChargeAmount {
        let total_material_costs: f64 = charge
            .material_costs
            .iter()
            .map(|material| material.value)
            .sum();
        let base_charge = ((charge.number_of_minutes / 60.0) * config.get_hourly_rate())
            + total_material_costs
            + charge.delivery_costs;
        let payment_processing_percentage = match charge.location {
            StripeLocation::EU => EU_PAYMENT_PROCESSING_PERCENTAGE,
            StripeLocation::International => INTERNATIONAL_PAYMENT_PROCESSING_PERCENTAGE,
            _ => PAYMENT_PROCESSING_PERCENTAGE,
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
}
