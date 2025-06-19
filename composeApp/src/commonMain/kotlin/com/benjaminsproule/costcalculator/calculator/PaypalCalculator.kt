package com.benjaminsproule.costcalculator.calculator

import com.benjaminsproule.costcalculator.store.Config
import kotlin.math.ceil

enum class PaypalLocation(override val label: String) : Option {
    Local("Local"),
    EEA("EEA"),
    International("International"),
}

data class PaypalSale(
    override val cost: Float,
    override val deliveryCosts: Float,
    val location: PaypalLocation
) : Sale()

data class PaypalCharge(
    override val numberOfMinutes: Float,
    override val materialCosts: List<Material>,
    override val deliveryCosts: Float,
    val location: PaypalLocation
) : Charge()

class PaypalCalculator(private val config: Config) : Calculator<PaypalSale, PaypalCharge> {
    companion object {
        const val PAYMENT_PROCESSING_PERCENTAGE: Float = 0.015f
        const val EEA_PAYMENT_PROCESSING_PERCENTAGE: Float = 0.025f
        const val INTERNATIONAL_PAYMENT_PROCESSING_PERCENTAGE: Float = 0.035f
        const val PAYMENT_PROCESSING_FEE: Float = 0.3f
    }

    override fun basedOnSale(sale: PaypalSale): SaleBreakdown {
        val saleTotal = sale.cost + sale.deliveryCosts
        val additionalPaymentProcessingPercentage = if (sale.location == PaypalLocation.EEA) {
            EEA_PAYMENT_PROCESSING_PERCENTAGE
        } else if (sale.location == PaypalLocation.International) {
            INTERNATIONAL_PAYMENT_PROCESSING_PERCENTAGE
        } else {
            0.0f
        }
        val baseProcessingPayment = (saleTotal * PAYMENT_PROCESSING_PERCENTAGE) + PAYMENT_PROCESSING_FEE
        val additionalProcessingPayment = saleTotal * additionalPaymentProcessingPercentage
        val paymentProcessingCost = baseProcessingPayment + additionalProcessingPayment;
        val tax = sale.cost * (config.taxRate / 100.0f)
        val revenue = sale.cost - paymentProcessingCost - tax
        val percentageKept = (revenue / sale.cost) * 100.0f
        val maxWorkingHours = revenue / config.hourlyRate
        return SaleBreakdown(
            sale = sale.cost,
            deliveryCosts = sale.deliveryCosts,
            transactionCost = 0.0f,
            paymentProcessingCost = paymentProcessingCost,
            offsiteAdsCost = 0.0f,
            regulatoryOperatingFee = 0.0f,
            vatPaidByBuyer = 0.0f,
            vatOnSellerFees = 0.0f,
            totalFees = paymentProcessingCost,
            totalFeesWithVat = paymentProcessingCost,
            tax = tax,
            revenue = revenue,
            percentageKept = percentageKept,
            maxWorkingHours = maxWorkingHours,
        )
    }

    override fun howMuchToCharge(
        charge: PaypalCharge
    ): ChargeAmount {
        val totalMaterialCosts = charge.materialCosts.map { material -> material.value }.sum();
        val baseCharge =
            ((charge.numberOfMinutes / 60.0f) * config.hourlyRate) + totalMaterialCosts + charge.deliveryCosts;
        val additionalPaymentProcessingPercentage = if (charge.location == PaypalLocation.EEA) {
            EEA_PAYMENT_PROCESSING_PERCENTAGE
        } else if (charge.location == PaypalLocation.International) {
            INTERNATIONAL_PAYMENT_PROCESSING_PERCENTAGE
        } else {
            0.0f
        }
        val baseProcessingPayment =
            (baseCharge * PAYMENT_PROCESSING_PERCENTAGE) + PAYMENT_PROCESSING_FEE;
        val additionalProcessingPayment =
            baseCharge * additionalPaymentProcessingPercentage;
        val paymentProcessingCost = baseProcessingPayment + additionalProcessingPayment;
        val chargeWithFees = baseCharge + paymentProcessingCost;

        val markupPercentage = (100.0f + config.markupPercentage) / 100.0f

        val totalToCharge = ceil(chargeWithFees * markupPercentage)
        return ChargeAmount(
            totalToCharge = totalToCharge,
            withVat = totalToCharge * 1.2f,
        )
    }
}
