pub mod etsy_calculator;
pub mod paypal_calculator;
pub mod shopify_calculator;
pub mod stripe_calculator;
pub mod sumup_calculator;

pub struct SaleBreakdown {
    pub sale: f64,
    pub delivery_costs: f64,
    pub transaction_cost: f64,
    pub payment_processing_cost: f64,
    pub offsite_ads_cost: f64,
    pub regulatory_operating_fee: f64,
    pub vat_paid_by_buyer: f64,
    pub vat_on_seller_fees: f64,
    pub total_fees: f64,
    pub total_fees_with_vat: f64,
    pub tax: f64,
    pub revenue: f64,
    pub percentage_kept: f64,
    pub max_working_hours: f64,
}

pub struct ChargeAmount {
    pub total_to_charge: f64,
    pub with_vat: f64,
}

pub struct Material {
    pub name: String,
    pub value: f64,
}

pub trait Calculator<S, C> {
    fn based_on_sale(&self, config: &dyn Config, sale: S) -> SaleBreakdown;
    fn how_much_to_charge(&self, config: &dyn Config, charge: C) -> ChargeAmount;
}

pub trait Sale {
    fn get_cost(&self) -> f64;
    fn get_delivery_costs(&self) -> f64;
}

pub trait Charge {
    fn get_number_of_minutes(&self) -> f64;
    fn get_material_costs(&self) -> &Vec<Material>;
    fn get_delivery_costs(&self) -> f64;
}

pub trait Config {
    fn get_markup_percentage(&self) -> f64;
    fn get_hourly_rate(&self) -> f64;
    fn get_tax_rate(&self) -> f64;
    fn get_vat(&self) -> f64;
}
