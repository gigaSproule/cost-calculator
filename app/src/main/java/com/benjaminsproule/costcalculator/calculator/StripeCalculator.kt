package com.benjaminsproule.costcalculator.calculator

import com.benjaminsproule.costcalculator.store.Config
import kotlin.math.ceil

enum class StripeLocation(override val label: String) : Option {
    Local("Local"),
    EU("EU"),
    International("International"),
}

data class StripeSale(
    override val cost: Float,
    override val deliveryCosts: Float,
    val location: StripeLocation
) : Sale()

data class StripeCharge(
    override val numberOfMinutes: Float,
    override val materialCosts: List<Material>,
    override val deliveryCosts: Float,
    val location: StripeLocation
) : Charge()

class StripeCalculator(private val config: Config) : Calculator<StripeSale, StripeCharge> {
    companion object {
        const val PAYMENT_PROCESSING_PERCENTAGE: Float = 0.015f;
        const val EU_PAYMENT_PROCESSING_PERCENTAGE: Float = 0.025f;
        const val INTERNATIONAL_PAYMENT_PROCESSING_PERCENTAGE: Float = 0.035f;
        const val PAYMENT_PROCESSING_FEE: Float = 0.2f;
    }

    override fun basedOnSale(sale: StripeSale): SaleBreakdown {
        val saleTotal = sale.cost + sale.deliveryCosts;
        val paymentProcessingPercentage = if (sale.location == StripeLocation.EU) {
            EU_PAYMENT_PROCESSING_PERCENTAGE
        } else if (sale.location == StripeLocation.International) {
            INTERNATIONAL_PAYMENT_PROCESSING_PERCENTAGE
        } else {
            PAYMENT_PROCESSING_PERCENTAGE
        };
        val paymentProcessingCost = (saleTotal * paymentProcessingPercentage) + PAYMENT_PROCESSING_FEE;
        val tax = sale.cost * (config.taxRate / 100.0f);
        val revenue = sale.cost - paymentProcessingCost - tax;
        val percentageKept = (revenue / sale.cost) * 100.0f;
        val maxWorkingHours = revenue / config.hourlyRate;
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
        charge: StripeCharge
    ): ChargeAmount {
        val totalMaterialCosts = charge.materialCosts.map { material -> material.value }.sum();
        val baseCharge =
            ((charge.numberOfMinutes / 60.0f) * config.hourlyRate) + totalMaterialCosts + charge.deliveryCosts
        val paymentProcessingPercentage = if (charge.location == StripeLocation.EU) {
            EU_PAYMENT_PROCESSING_PERCENTAGE
        } else if (charge.location == StripeLocation.International) {
            INTERNATIONAL_PAYMENT_PROCESSING_PERCENTAGE
        } else {
            PAYMENT_PROCESSING_PERCENTAGE
        };
        val paymentProcessingCost = (baseCharge * paymentProcessingPercentage) + PAYMENT_PROCESSING_FEE;
        val chargeWithFees = baseCharge + paymentProcessingCost;

        val markupPercentage = (100.0f + config.markupPercentage) / 100.0f;

        val totalToCharge = ceil(chargeWithFees * markupPercentage);
        return ChargeAmount(
            totalToCharge = totalToCharge,
            withVat = totalToCharge * 1.2f,
        )
    }
}
