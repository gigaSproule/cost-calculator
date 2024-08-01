package com.benjaminsproule.costcalculator.calculator

import com.benjaminsproule.costcalculator.store.Config
import kotlin.math.ceil
import kotlin.math.round
import kotlin.math.roundToInt

class EtsyCalculator(private val config: Config) : Calculator {
    companion object {
        const val TRANSACTION_FEE: Float = 0.065f
        const val PAYMENT_PROCESSING_PERCENTAGE: Float = 0.04f
        const val PAYMENT_PROCESSING_FEE: Float = 0.2f
        const val OFFSITE_ADS_FEE: Float = 0.15f
        const val LISTING_FEE: Float = 0.16f // Actually $0.20 USD, but need it in GBP
        const val REGULATOR_OPERATING_FEE: Float = 0.0032f
    }

    override fun basedOnSale(sale: Float, deliveryCosts: Float, offsiteAds: Boolean): SaleBreakdown {
        val vatMultiplier = (config.vat / 100.0f) + 1.0f
        val saleTotal = sale + deliveryCosts
        val saleTotalWithTax = saleTotal * vatMultiplier
        val transactionCost = saleTotal * TRANSACTION_FEE
        val paymentProcessingCost = (saleTotalWithTax * PAYMENT_PROCESSING_PERCENTAGE) + PAYMENT_PROCESSING_FEE
        val offsiteAdsCost = if (offsiteAds) {
            saleTotalWithTax * OFFSITE_ADS_FEE
        } else {
            0.0f
        }
        val regulatoryOperatingFee = ((saleTotal * REGULATOR_OPERATING_FEE) * 100.0f).roundToInt() / 100.0f
        val totalFees = transactionCost + paymentProcessingCost + offsiteAdsCost + LISTING_FEE + regulatoryOperatingFee
        val tax = sale * (config.taxRate / 100.0f)
        val vatPaidByBuyer = sale * (config.vat / 100.0f)
        val vatOnSellerFees = totalFees * (config.vat / 100.0f)
        val totalFeesWithVat = totalFees + vatOnSellerFees

        val revenue = sale - totalFeesWithVat - tax
        val percentageKept = (revenue / sale) * 100.0f
        val maxWorkingHours = revenue / config.hourlyRate

        return SaleBreakdown(
            sale,
            deliveryCosts,
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
        numberOfMinutes: Float,
        materialCosts: List<Material>,
        deliveryCosts: Float,
        offsiteAds: Boolean
    ): ChargeAmount {
        val totalMaterialCosts = materialCosts.map {
            it.value
        }.sum()
        val baseCharge = ((numberOfMinutes / 60.0f) * config.hourlyRate) + totalMaterialCosts + deliveryCosts
        val offsiteAdsCost = if (offsiteAds) {
            baseCharge * OFFSITE_ADS_FEE
        } else {
            0.0f
        }
        val transactionCost = baseCharge * TRANSACTION_FEE
        val paymentProcessingCost =
            (baseCharge * PAYMENT_PROCESSING_PERCENTAGE) + PAYMENT_PROCESSING_FEE
        val charge =
            baseCharge + transactionCost + paymentProcessingCost + offsiteAdsCost + LISTING_FEE
        val regulatoryOperatingFee = (round(charge * REGULATOR_OPERATING_FEE) * 100.0f) / 100.0f

        val markupPercentage = (100.0f + config.markupPercentage) / 100.0f
        val totalToCharge = ceil((charge + regulatoryOperatingFee) * markupPercentage)
        val withVat = totalToCharge * ((config.vat / 100.0f) + 1.0f)
        return ChargeAmount(
            totalToCharge,
            withVat,
        )
    }
}
