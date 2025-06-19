package com.benjaminsproule.costcalculator.store

import com.benjaminsproule.costcalculator.calculator.Config as CalculatorConfig

data class Config(
    override val markupPercentage: Float = ConfigDefaults.MARKUP_PERCENTAGE,
    override val hourlyRate: Float = ConfigDefaults.MINIMUM_WAGE,
    override val taxRate: Float = ConfigDefaults.BASIC_RATE,
    override val vat: Float = ConfigDefaults.VAT,
    val currency: String = ConfigDefaults.CURRENCY,
) : CalculatorConfig()

object ConfigDefaults {
    val MARKUP_PERCENTAGE = 0.0f
    val MINIMUM_WAGE = 10.42f
    val BASIC_RATE = 20.0f
    val VAT = 20.0f
    val CURRENCY = "£"
}