pub(crate) mod etsy_calculator;
pub(crate) mod paypal_calculator;
pub(crate) mod shopify_calculator;
pub(crate) mod stripe_calculator;

pub(crate) struct SaleBreakdown {
    pub(crate) sale: f64,
    pub(crate) delivery_costs: f64,
    pub(crate) transaction_cost: f64,
    pub(crate) payment_processing_cost: f64,
    pub(crate) offsite_ads_cost: f64,
    pub(crate) regulatory_operating_fee: f64,
    pub(crate) vat_paid_by_buyer: f64,
    pub(crate) vat_on_seller_fees: f64,
    pub(crate) total_fees: f64,
    pub(crate) total_fees_with_vat: f64,
    pub(crate) tax: f64,
    pub(crate) revenue: f64,
    pub(crate) percentage_kept: f64,
    pub(crate) max_working_hours: f64,
}

pub(crate) struct ChargeAmount {
    pub(crate) total_to_charge: f64,
    pub(crate) with_vat: f64,
}

trait Calculator {
    fn based_on_sale() -> SaleBreakdown;
    fn how_much_to_charge() -> ChargeAmount;
}
