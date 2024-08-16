use super::{Calculator, Charge, Material, Sale};
use super::{ChargeAmount, Config, SaleBreakdown};

const PAYMENT_PROCESSING_PERCENTAGE: f64 = 0.015;
const EEA_PAYMENT_PROCESSING_PERCENTAGE: f64 = 0.025;
const INTERNATIONAL_PAYMENT_PROCESSING_PERCENTAGE: f64 = 0.035;
const PAYMENT_PROCESSING_FEE: f64 = 0.3;

#[derive(Debug)]
pub enum PaypalLocation {
    Local,
    EEA,
    International,
}

pub struct PaypalSale {
    pub cost: f64,
    pub delivery_costs: f64,
    pub location: PaypalLocation,
}

impl PaypalSale {
    fn get_location(&self) -> &PaypalLocation {
        &self.location
    }
}

impl Sale for PaypalSale {
    fn get_cost(&self) -> f64 {
        self.cost
    }

    fn get_delivery_costs(&self) -> f64 {
        self.delivery_costs
    }
}

pub struct PaypalCharge {
    pub number_of_minutes: f64,
    pub material_costs: Vec<Material>,
    pub delivery_costs: f64,
    pub location: PaypalLocation,
}

impl PaypalCharge {
    fn get_location(&self) -> &PaypalLocation {
        &self.location
    }
}

impl Charge for PaypalCharge {
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

pub struct PaypalCalculator {}

impl Calculator<PaypalSale, PaypalCharge> for PaypalCalculator {
    fn based_on_sale(&self, config: &dyn Config, sale: PaypalSale) -> SaleBreakdown {
        let sale_total = sale.cost + sale.delivery_costs;
        let additional_payment_processing_percentage = match sale.location {
            PaypalLocation::EEA => EEA_PAYMENT_PROCESSING_PERCENTAGE,
            PaypalLocation::International => INTERNATIONAL_PAYMENT_PROCESSING_PERCENTAGE,
            _ => 0.0,
        };
        let base_processing_payment =
            (sale_total * PAYMENT_PROCESSING_PERCENTAGE) + PAYMENT_PROCESSING_FEE;
        let additional_processing_payment = sale_total * additional_payment_processing_percentage;
        let payment_processing_cost = base_processing_payment + additional_processing_payment;
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

    fn how_much_to_charge(&self, config: &dyn Config, charge: PaypalCharge) -> ChargeAmount {
        let total_material_costs: f64 = charge
            .material_costs
            .iter()
            .map(|material| material.value)
            .sum();
        let base_charge = ((charge.number_of_minutes / 60.0) * config.get_hourly_rate())
            + total_material_costs
            + charge.delivery_costs;
        let additional_payment_processing_percentage = match charge.location {
            PaypalLocation::EEA => EEA_PAYMENT_PROCESSING_PERCENTAGE,
            PaypalLocation::International => INTERNATIONAL_PAYMENT_PROCESSING_PERCENTAGE,
            _ => 0.0,
        };
        let base_processing_payment =
            (base_charge * PAYMENT_PROCESSING_PERCENTAGE) + PAYMENT_PROCESSING_FEE;
        let additional_processing_payment = base_charge * additional_payment_processing_percentage;
        let payment_processing_cost = base_processing_payment + additional_processing_payment;
        let charge = base_charge + payment_processing_cost;

        let markup_percentage = (100.0 + config.get_markup_percentage()) / 100.0;

        let total_to_charge = (charge * markup_percentage).ceil();
        ChargeAmount {
            total_to_charge,
            with_vat: total_to_charge * 1.2,
        }
    }
}
