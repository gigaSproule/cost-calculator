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

interface Calculator<S : Sale, C : Charge> {
    fun basedOnSale(sale: S): SaleBreakdown
    fun howMuchToCharge(charge: C): ChargeAmount
}

abstract class Sale {
    abstract val cost: Float
    abstract val deliveryCosts: Float
}

abstract class Charge {
    abstract val numberOfMinutes: Float
    abstract val materialCosts: List<Material>
    abstract val deliveryCosts: Float
}

abstract class Config {
    abstract val markupPercentage: Float
    abstract val hourlyRate: Float
    abstract val taxRate: Float
    abstract val vat: Float
}
