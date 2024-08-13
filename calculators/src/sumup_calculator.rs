use super::Material;
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

pub enum PaymentOption {
    CardReader,
    PosLite,
    TapToPayIPhone,
    RemotePayment,
    QrCode,
}

pub enum SubscriptionOption {
    NoContract,
    SumUpOne,
}

pub fn based_on_sale(
    config: &dyn Config,
    sale: f64,
    payment_option: PaymentOption,
    subscription_option: SubscriptionOption,
) -> SaleBreakdown {
    let transaction_fee = match payment_option {
        PaymentOption::CardReader => {
            if matches!(subscription_option, SubscriptionOption::NoContract) {
                TRANSACTION_FEE_CARD_READER_NO_CONTRACT
            } else {
                TRANSACTION_FEE_CARD_READER_SUMUP_ONE
            }
        }
        PaymentOption::PosLite => {
            if matches!(subscription_option, SubscriptionOption::NoContract) {
                TRANSACTION_FEE_POS_LITE_NO_CONTRACT
            } else {
                TRANSACTION_FEE_POS_LITE_SUMUP_ONE
            }
        }
        PaymentOption::TapToPayIPhone => {
            if matches!(subscription_option, SubscriptionOption::NoContract) {
                TRANSACTION_FEE_TAP_TO_PAY_IPHONE_NO_CONTRACT
            } else {
                TRANSACTION_FEE_TAP_TO_PAY_IPHONE_SUMUP_ONE
            }
        }
        PaymentOption::RemotePayment => {
            if matches!(subscription_option, SubscriptionOption::NoContract) {
                TRANSACTION_FEE_REMOTE_PAYMENT_NO_CONTRACT
            } else {
                TRANSACTION_FEE_REMOTE_PAYMENT_SUMUP_ONE
            }
        }
        PaymentOption::QrCode => {
            if matches!(subscription_option, SubscriptionOption::NoContract) {
                TRANSACTION_FEE_QR_CODE_NO_CONTRACT
            } else {
                TRANSACTION_FEE_QR_CODE_SUMUP_ONE
            }
        }
    };

    let transaction_cost = sale * transaction_fee;

    let tax = sale * (config.get_tax_rate() / 100.0);

    let revenue = sale - transaction_cost - tax;
    let percentage_kept = (revenue / sale) * 100.0;
    let max_working_hours = revenue / config.get_hourly_rate();

    SaleBreakdown {
        sale,
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

pub fn how_much_to_charge(
    config: &dyn Config,
    number_of_minutes: f64,
    material_costs: Vec<Material>,
    payment_option: PaymentOption,
    subscription_option: SubscriptionOption,
) -> ChargeAmount {
    let total_material_costs: f64 = material_costs.iter().map(|material| material.value).sum();
    let base_charge =
        ((number_of_minutes / 60.0) * config.get_hourly_rate()) + total_material_costs;

    let transaction_fee = match payment_option {
        PaymentOption::CardReader => {
            if matches!(subscription_option, SubscriptionOption::NoContract) {
                TRANSACTION_FEE_CARD_READER_NO_CONTRACT
            } else {
                TRANSACTION_FEE_CARD_READER_SUMUP_ONE
            }
        }
        PaymentOption::PosLite => {
            if matches!(subscription_option, SubscriptionOption::NoContract) {
                TRANSACTION_FEE_POS_LITE_NO_CONTRACT
            } else {
                TRANSACTION_FEE_POS_LITE_SUMUP_ONE
            }
        }
        PaymentOption::TapToPayIPhone => {
            if matches!(subscription_option, SubscriptionOption::NoContract) {
                TRANSACTION_FEE_TAP_TO_PAY_IPHONE_NO_CONTRACT
            } else {
                TRANSACTION_FEE_TAP_TO_PAY_IPHONE_SUMUP_ONE
            }
        }
        PaymentOption::RemotePayment => {
            if matches!(subscription_option, SubscriptionOption::NoContract) {
                TRANSACTION_FEE_REMOTE_PAYMENT_NO_CONTRACT
            } else {
                TRANSACTION_FEE_REMOTE_PAYMENT_SUMUP_ONE
            }
        }
        PaymentOption::QrCode => {
            if matches!(subscription_option, SubscriptionOption::NoContract) {
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
