package com.benjaminsproule.costcalculator.calculator

import com.benjaminsproule.costcalculator.store.Config
import kotlin.math.ceil
import kotlin.math.round
import kotlin.math.roundToInt

data class EtsySale(
    override val cost: Float,
    override val deliveryCosts: Float,
    val offsiteAds: Boolean
) : Sale()

data class EtsyCharge(
    override val numberOfMinutes: Float,
    override val materialCosts: List<Material>,
    override val deliveryCosts: Float,
    val offsiteAds: Boolean
) : Charge()

class EtsyCalculator(private val config: Config) : Calculator<EtsySale, EtsyCharge> {
    companion object {
        const val TRANSACTION_FEE: Float = 0.065f
        const val PAYMENT_PROCESSING_PERCENTAGE: Float = 0.04f
        const val PAYMENT_PROCESSING_FEE: Float = 0.2f
        const val OFFSITE_ADS_FEE: Float = 0.15f
        const val LISTING_FEE: Float = 0.16f // Actually $0.20 USD, but need it in GBP
        const val REGULATOR_OPERATING_FEE: Float = 0.0032f
    }

    override fun basedOnSale(sale: EtsySale): SaleBreakdown {
        val vatMultiplier = (config.vat / 100.0f) + 1.0f
        val saleTotal = sale.cost + sale.deliveryCosts
        val saleTotalWithTax = saleTotal * vatMultiplier
        val transactionCost = saleTotal * TRANSACTION_FEE
        val paymentProcessingCost = (saleTotalWithTax * PAYMENT_PROCESSING_PERCENTAGE) + PAYMENT_PROCESSING_FEE
        val offsiteAdsCost = if (sale.offsiteAds) {
            saleTotalWithTax * OFFSITE_ADS_FEE
        } else {
            0.0f
        }
        val regulatoryOperatingFee = ((saleTotal * REGULATOR_OPERATING_FEE) * 100.0f).roundToInt() / 100.0f
        val totalFees = transactionCost + paymentProcessingCost + offsiteAdsCost + LISTING_FEE + regulatoryOperatingFee
        val tax = sale.cost * (config.taxRate / 100.0f)
        val vatPaidByBuyer = sale.cost * (config.vat / 100.0f)
        val vatOnSellerFees = totalFees * (config.vat / 100.0f)
        val totalFeesWithVat = totalFees + vatOnSellerFees

        val revenue = sale.cost - totalFeesWithVat - tax
        val percentageKept = (revenue / sale.cost) * 100.0f
        val maxWorkingHours = revenue / config.hourlyRate

        return SaleBreakdown(
            sale.cost,
            sale.deliveryCosts,
            transactionCost,
            paymentProcessingCost,
            offsiteAdsCost,
            regulatoryOperatingFee,
            vatPaidByBuyer,
            vatOnSellerFees,
            totalFees,
            totalFeesWithVat,
            tax,
            revenue,
            percentageKept,
            maxWorkingHours,
        )
    }

    override fun howMuchToCharge(
        charge: EtsyCharge
    ): ChargeAmount {
        val totalMaterialCosts = charge.materialCosts.map {
            it.value
        }.sum()
        val baseCharge =
            ((charge.numberOfMinutes / 60.0f) * config.hourlyRate) + totalMaterialCosts + charge.deliveryCosts
        val offsiteAdsCost = if (charge.offsiteAds) {
            baseCharge * OFFSITE_ADS_FEE
        } else {
            0.0f
        }
        val transactionCost = baseCharge * TRANSACTION_FEE
        val paymentProcessingCost =
            (baseCharge * PAYMENT_PROCESSING_PERCENTAGE) + PAYMENT_PROCESSING_FEE
        val chargeWithFees =
            baseCharge + transactionCost + paymentProcessingCost + offsiteAdsCost + LISTING_FEE
        val regulatoryOperatingFee = (round(chargeWithFees * REGULATOR_OPERATING_FEE) * 100.0f) / 100.0f

        val markupPercentage = (100.0f + config.markupPercentage) / 100.0f
        val totalToCharge = ceil((chargeWithFees + regulatoryOperatingFee) * markupPercentage)
        val withVat = totalToCharge * ((config.vat / 100.0f) + 1.0f)
        return ChargeAmount(
            totalToCharge,
            withVat,
        )
    }
}
