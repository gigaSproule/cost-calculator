package com.benjaminsproule.costcalculator.calculator

import kotlinx.serialization.Serializable

data class SaleBreakdown(
    val sale: Float,
    val deliveryCosts: Float,
    val transactionCost: Float,
    val paymentProcessingCost: Float,
    val offsiteAdsCost: Float,
    val regulatoryOperatingFee: Float,
    val vatPaidByBuyer: Float,
    val vatOnSellerFees: Float,
    val totalFees: Float,
    val totalFeesWithVat: Float,
    val tax: Float,
    val revenue: Float,
    val percentageKept: Float,
    val maxWorkingHours: Float
)

data class ChargeAmount(
    val totalToCharge: Float,
    val withVat: Float,
)

@Serializable
data class Material(
    val name: String,
    val value: Float,
)

interface Calculator {
    fun basedOnSale(sale: Float, deliveryCosts: Float, offsiteAds: Boolean): SaleBreakdown
    fun howMuchToCharge(
        numberOfMinutes: Float,
        materialCosts: List<Material>,
        deliveryCosts: Float,
        offsiteAds: Boolean
    ): ChargeAmount
}
