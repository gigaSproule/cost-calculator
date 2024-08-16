package com.benjaminsproule.costcalculator.calculator

import com.benjaminsproule.costcalculator.store.Config
import kotlin.math.ceil

data class ShopifySale(
    override val cost: Float,
    override val deliveryCosts: Float,
    val internationalOrAmex: Boolean
) : Sale()

data class ShopifyCharge(
    override val numberOfMinutes: Float,
    override val materialCosts: List<Material>,
    override val deliveryCosts: Float,
    val internationalOrAmex: Boolean
) : Charge()

class ShopifyCalculator(private val config: Config) : Calculator<ShopifySale, ShopifyCharge> {
    companion object {
        const val PAYMENT_PROCESSING_PERCENTAGE: Float = 0.02f
        const val INTERNATIONAL_AMEX_PAYMENT_PROCESSING_PERCENTAGE: Float = 0.031f
        const val PAYMENT_PROCESSING_FEE: Float = 0.25f
    }

    override fun basedOnSale(sale: ShopifySale): SaleBreakdown {
        val saleTotal = sale.cost + sale.deliveryCosts
        val paymentProcessingPercentage = if (sale.internationalOrAmex) {
            INTERNATIONAL_AMEX_PAYMENT_PROCESSING_PERCENTAGE
        } else {
            PAYMENT_PROCESSING_PERCENTAGE
        }
        val paymentProcessingCost = (saleTotal * paymentProcessingPercentage) + PAYMENT_PROCESSING_FEE
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
        charge: ShopifyCharge
    ): ChargeAmount {
        val totalMaterialCosts = charge.materialCosts.map { material -> material.value }.sum()
        val baseCharge =
            ((charge.numberOfMinutes / 60.0f) * config.hourlyRate) + totalMaterialCosts + charge.deliveryCosts
        val paymentProcessingPercentage = if (charge.internationalOrAmex) {
            INTERNATIONAL_AMEX_PAYMENT_PROCESSING_PERCENTAGE
        } else {
            PAYMENT_PROCESSING_PERCENTAGE
        }
        val paymentProcessingCost =
            (baseCharge * paymentProcessingPercentage) + PAYMENT_PROCESSING_FEE
        val chargeWithFees = baseCharge + paymentProcessingCost

        val markupPercentage = (100.0f + config.markupPercentage) / 100.0f

        val totalToCharge = ceil(chargeWithFees * markupPercentage)
        return ChargeAmount(
            totalToCharge = totalToCharge,
            withVat = totalToCharge * 1.2f,
        )
    }
}
