package com.benjaminsproule.costcalculator.calculator

import com.benjaminsproule.costcalculator.store.Config
import kotlin.math.ceil

interface Option {
    val label: String
}

enum class PaymentOption(override val label: String) : Option {
    CardReader("Card reader"),
    PosLite("POS lite"),
    TapToPayIPhone("Tap to pay iPhone"),
    RemotePayment("Remote payment"),
    QrCode("QR code"),
}

enum class SubscriptionOption(override val label: String) : Option {
    NoContract("No contract"),
    SumUpOne("SumUp One"),
}

data class SumUpSale(
    override val cost: Float,
    val paymentOption: PaymentOption,
    val subscriptionOption: SubscriptionOption
) : Sale() {
    override val deliveryCosts: Float
        get() = 0.0f
}

data class SumUpCharge(
    override val numberOfMinutes: Float,
    override val materialCosts: List<Material>,
    val paymentOption: PaymentOption,
    val subscriptionOption: SubscriptionOption
) : Charge() {
    override val deliveryCosts: Float
        get() = 0.0f
}

class SumUpCalculator(private val config: Config) : Calculator<SumUpSale, SumUpCharge> {
    companion object {
        const val TRANSACTION_FEE_CARD_READER_NO_CONTRACT: Float = 0.0169f
        const val TRANSACTION_FEE_POS_LITE_NO_CONTRACT: Float = 0.0169f
        const val TRANSACTION_FEE_TAP_TO_PAY_IPHONE_NO_CONTRACT: Float = 0.0169f
        const val TRANSACTION_FEE_REMOTE_PAYMENT_NO_CONTRACT: Float = 0.025f
        const val TRANSACTION_FEE_QR_CODE_NO_CONTRACT: Float = 0.0f
        const val TRANSACTION_FEE_CARD_READER_SUMUP_ONE: Float = 0.0099f
        const val TRANSACTION_FEE_POS_LITE_SUMUP_ONE: Float = 0.0099f
        const val TRANSACTION_FEE_TAP_TO_PAY_IPHONE_SUMUP_ONE: Float = 0.0099f
        const val TRANSACTION_FEE_REMOTE_PAYMENT_SUMUP_ONE: Float = 0.0099f
        const val TRANSACTION_FEE_QR_CODE_SUMUP_ONE: Float = 0.0f
    }

    override fun basedOnSale(sale: SumUpSale): SaleBreakdown {
        val transactionFee = when (sale.paymentOption) {
            PaymentOption.CardReader -> {
                if (sale.subscriptionOption == SubscriptionOption.NoContract) {
                    TRANSACTION_FEE_CARD_READER_NO_CONTRACT
                } else {
                    TRANSACTION_FEE_CARD_READER_SUMUP_ONE
                }
            }

            PaymentOption.PosLite -> {
                if (sale.subscriptionOption == SubscriptionOption.NoContract) {
                    TRANSACTION_FEE_POS_LITE_NO_CONTRACT
                } else {
                    TRANSACTION_FEE_POS_LITE_SUMUP_ONE
                }
            }

            PaymentOption.TapToPayIPhone -> {
                if (sale.subscriptionOption == SubscriptionOption.NoContract) {
                    TRANSACTION_FEE_TAP_TO_PAY_IPHONE_NO_CONTRACT
                } else {
                    TRANSACTION_FEE_TAP_TO_PAY_IPHONE_SUMUP_ONE
                }
            }

            PaymentOption.RemotePayment -> {
                if (sale.subscriptionOption == SubscriptionOption.NoContract) {
                    TRANSACTION_FEE_REMOTE_PAYMENT_NO_CONTRACT
                } else {
                    TRANSACTION_FEE_REMOTE_PAYMENT_SUMUP_ONE
                }
            }

            PaymentOption.QrCode -> {
                if (sale.subscriptionOption == SubscriptionOption.NoContract) {
                    TRANSACTION_FEE_QR_CODE_NO_CONTRACT
                } else {
                    TRANSACTION_FEE_QR_CODE_SUMUP_ONE
                }
            }
        }

        val transactionCost = sale.cost * transactionFee;

        val tax = sale.cost * (config.taxRate / 100.0f);

        val revenue = sale.cost - transactionCost - tax;
        val percentageKept = (revenue / sale.cost) * 100.0f;
        val maxWorkingHours = revenue / config.hourlyRate;

        return SaleBreakdown(
            sale = sale.cost,
            deliveryCosts = 0.0f,
            transactionCost = transactionCost,
            paymentProcessingCost = 0.0f,
            offsiteAdsCost = 0.0f,
            regulatoryOperatingFee = 0.0f,
            vatPaidByBuyer = 0.0f,
            vatOnSellerFees = 0.0f,
            totalFees = transactionCost,
            totalFeesWithVat = transactionCost,
            tax = tax,
            revenue = revenue,
            percentageKept = percentageKept,
            maxWorkingHours = maxWorkingHours,
        )
    }

    override fun howMuchToCharge(
        charge: SumUpCharge
    ): ChargeAmount {
        val totalMaterialCosts = charge.materialCosts.map { material -> material.value }.sum();
        val baseCharge = ((charge.numberOfMinutes / 60.0f) * config.hourlyRate) + totalMaterialCosts;

        val transactionFee = when (charge.paymentOption) {
            PaymentOption.CardReader -> {
                if (charge.subscriptionOption == SubscriptionOption.NoContract) {
                    TRANSACTION_FEE_CARD_READER_NO_CONTRACT
                } else {
                    TRANSACTION_FEE_CARD_READER_SUMUP_ONE
                }
            }

            PaymentOption.PosLite -> {
                if (charge.subscriptionOption == SubscriptionOption.NoContract) {
                    TRANSACTION_FEE_POS_LITE_NO_CONTRACT
                } else {
                    TRANSACTION_FEE_POS_LITE_SUMUP_ONE
                }
            }

            PaymentOption.TapToPayIPhone -> {
                if (charge.subscriptionOption == SubscriptionOption.NoContract) {
                    TRANSACTION_FEE_TAP_TO_PAY_IPHONE_NO_CONTRACT
                } else {
                    TRANSACTION_FEE_TAP_TO_PAY_IPHONE_SUMUP_ONE
                }
            }

            PaymentOption.RemotePayment -> {
                if (charge.subscriptionOption == SubscriptionOption.NoContract) {
                    TRANSACTION_FEE_REMOTE_PAYMENT_NO_CONTRACT
                } else {
                    TRANSACTION_FEE_REMOTE_PAYMENT_SUMUP_ONE
                }
            }

            PaymentOption.QrCode -> {
                if (charge.subscriptionOption == SubscriptionOption.NoContract) {
                    TRANSACTION_FEE_QR_CODE_NO_CONTRACT
                } else {
                    TRANSACTION_FEE_QR_CODE_SUMUP_ONE
                }
            }
        }

        val transactionCost = baseCharge * transactionFee;
        val chargeWithFees = baseCharge + transactionCost;

        val markupPercentage = (100.0f + config.markupPercentage) / 100.0f;
        val totalToCharge = ceil(chargeWithFees * markupPercentage);
        val withVat = totalToCharge * ((config.vat / 100.0f) + 1.0f);

        return ChargeAmount(
            totalToCharge = totalToCharge,
            withVat = withVat,
        )
    }
}
