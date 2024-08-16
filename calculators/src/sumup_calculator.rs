use super::{Calculator, Charge, Material, Sale};
use super::{ChargeAmount, Config, SaleBreakdown};

const TRANSACTION_FEE_CARD_READER_NO_CONTRACT: f64 = 0.0169;
const TRANSACTION_FEE_POS_LITE_NO_CONTRACT: f64 = 0.0169;
const TRANSACTION_FEE_TAP_TO_PAY_IPHONE_NO_CONTRACT: f64 = 0.0169;
const TRANSACTION_FEE_REMOTE_PAYMENT_NO_CONTRACT: f64 = 0.025;
const TRANSACTION_FEE_QR_CODE_NO_CONTRACT: f64 = 0.0;
const TRANSACTION_FEE_CARD_READER_SUMUP_ONE: f64 = 0.0099;
const TRANSACTION_FEE_POS_LITE_SUMUP_ONE: f64 = 0.0099;
const TRANSACTION_FEE_TAP_TO_PAY_IPHONE_SUMUP_ONE: f64 = 0.0099;
const TRANSACTION_FEE_REMOTE_PAYMENT_SUMUP_ONE: f64 = 0.0099;
const TRANSACTION_FEE_QR_CODE_SUMUP_ONE: f64 = 0.0;

#[derive(Debug)]
pub enum PaymentOption {
    CardReader,
    PosLite,
    TapToPayIPhone,
    RemotePayment,
    QrCode,
}

#[derive(Debug)]
pub enum SubscriptionOption {
    NoContract,
    SumUpOne,
}

pub struct SumUpSale {
    pub cost: f64,
    pub payment_option: PaymentOption,
    pub subscription_option: SubscriptionOption,
}

impl SumUpSale {
    fn get_payment_option(&self) -> &PaymentOption {
        &self.payment_option
    }

    fn get_subscription_option(&self) -> &SubscriptionOption {
        &self.subscription_option
    }
}

impl Sale for SumUpSale {
    fn get_cost(&self) -> f64 {
        self.cost
    }

    fn get_delivery_costs(&self) -> f64 {
        0.0
    }
}

pub struct SumUpCharge {
    pub number_of_minutes: f64,
    pub material_costs: Vec<Material>,
    pub payment_option: PaymentOption,
    pub subscription_option: SubscriptionOption,
}

impl SumUpCharge {
    fn get_payment_option(&self) -> &PaymentOption {
        &self.payment_option
    }

    fn get_subscription_option(&self) -> &SubscriptionOption {
        &self.subscription_option
    }
}

impl Charge for SumUpCharge {
    fn get_number_of_minutes(&self) -> f64 {
        self.number_of_minutes
    }

    fn get_material_costs(&self) -> &Vec<Material> {
        &self.material_costs
    }

    fn get_delivery_costs(&self) -> f64 {
        0.0
    }
}

pub struct SumUpCalculator {}

impl Calculator<SumUpSale, SumUpCharge> for SumUpCalculator {
    fn based_on_sale(&self, config: &dyn Config, sale: SumUpSale) -> SaleBreakdown {
        let transaction_fee = match sale.payment_option {
            PaymentOption::CardReader => {
                if matches!(sale.subscription_option, SubscriptionOption::NoContract) {
                    TRANSACTION_FEE_CARD_READER_NO_CONTRACT
                } else {
                    TRANSACTION_FEE_CARD_READER_SUMUP_ONE
                }
            }
            PaymentOption::PosLite => {
                if matches!(sale.subscription_option, SubscriptionOption::NoContract) {
                    TRANSACTION_FEE_POS_LITE_NO_CONTRACT
                } else {
                    TRANSACTION_FEE_POS_LITE_SUMUP_ONE
                }
            }
            PaymentOption::TapToPayIPhone => {
                if matches!(sale.subscription_option, SubscriptionOption::NoContract) {
                    TRANSACTION_FEE_TAP_TO_PAY_IPHONE_NO_CONTRACT
                } else {
                    TRANSACTION_FEE_TAP_TO_PAY_IPHONE_SUMUP_ONE
                }
            }
            PaymentOption::RemotePayment => {
                if matches!(sale.subscription_option, SubscriptionOption::NoContract) {
                    TRANSACTION_FEE_REMOTE_PAYMENT_NO_CONTRACT
                } else {
                    TRANSACTION_FEE_REMOTE_PAYMENT_SUMUP_ONE
                }
            }
            PaymentOption::QrCode => {
                if matches!(sale.subscription_option, SubscriptionOption::NoContract) {
                    TRANSACTION_FEE_QR_CODE_NO_CONTRACT
                } else {
                    TRANSACTION_FEE_QR_CODE_SUMUP_ONE
                }
            }
        };

        let transaction_cost = sale.cost * transaction_fee;

        let tax = sale.cost * (config.get_tax_rate() / 100.0);

        let revenue = sale.cost - transaction_cost - tax;
        let percentage_kept = (revenue / sale.cost) * 100.0;
        let max_working_hours = revenue / config.get_hourly_rate();

        SaleBreakdown {
            sale: sale.cost,
            delivery_costs: 0.0,
            transaction_cost,
            payment_processing_cost: 0.0,
            offsite_ads_cost: 0.0,
            regulatory_operating_fee: 0.0,
            vat_paid_by_buyer: 0.0,
            vat_on_seller_fees: 0.0,
            total_fees: transaction_cost,
            total_fees_with_vat: transaction_cost,
            tax,
            revenue,
            percentage_kept,
            max_working_hours,
        }
    }

    fn how_much_to_charge(&self, config: &dyn Config, charge: SumUpCharge) -> ChargeAmount {
        let total_material_costs: f64 = charge
            .material_costs
            .iter()
            .map(|material| material.value)
            .sum();
        let base_charge =
            ((charge.number_of_minutes / 60.0) * config.get_hourly_rate()) + total_material_costs;

        let transaction_fee = match charge.payment_option {
            PaymentOption::CardReader => {
                if matches!(charge.subscription_option, SubscriptionOption::NoContract) {
                    TRANSACTION_FEE_CARD_READER_NO_CONTRACT
                } else {
                    TRANSACTION_FEE_CARD_READER_SUMUP_ONE
                }
            }
            PaymentOption::PosLite => {
                if matches!(charge.subscription_option, SubscriptionOption::NoContract) {
                    TRANSACTION_FEE_POS_LITE_NO_CONTRACT
                } else {
                    TRANSACTION_FEE_POS_LITE_SUMUP_ONE
                }
            }
            PaymentOption::TapToPayIPhone => {
                if matches!(charge.subscription_option, SubscriptionOption::NoContract) {
                    TRANSACTION_FEE_TAP_TO_PAY_IPHONE_NO_CONTRACT
                } else {
                    TRANSACTION_FEE_TAP_TO_PAY_IPHONE_SUMUP_ONE
                }
            }
            PaymentOption::RemotePayment => {
                if matches!(charge.subscription_option, SubscriptionOption::NoContract) {
                    TRANSACTION_FEE_REMOTE_PAYMENT_NO_CONTRACT
                } else {
                    TRANSACTION_FEE_REMOTE_PAYMENT_SUMUP_ONE
                }
            }
            PaymentOption::QrCode => {
                if matches!(charge.subscription_option, SubscriptionOption::NoContract) {
                    TRANSACTION_FEE_QR_CODE_NO_CONTRACT
                } else {
                    TRANSACTION_FEE_QR_CODE_SUMUP_ONE
                }
            }
        };

        let transaction_cost = base_charge * transaction_fee;
        let charge = base_charge + transaction_cost;

        let markup_percentage = (100.0 + config.get_markup_percentage()) / 100.0;
        let total_to_charge = (charge * markup_percentage).ceil();
        let with_vat = total_to_charge * ((config.get_vat() / 100.0) + 1.0);

        ChargeAmount {
            total_to_charge,
            with_vat,
        }
    }
}
