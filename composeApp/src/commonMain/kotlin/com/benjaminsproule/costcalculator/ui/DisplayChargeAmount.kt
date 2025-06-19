package com.benjaminsproule.costcalculator.ui

import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import com.benjaminsproule.costcalculator.calculator.ChargeAmount
import com.benjaminsproule.costcalculator.store.Config

@Composable
fun DisplayChargeAmount(
    chargeAmount: ChargeAmount?,
    config: Config
) {
    Text(
        if (chargeAmount == null) {
            ""
        } else {
            "%s%.02f (with VAT %s%.02f)".format(
                config.currency,
                chargeAmount.totalToCharge,
                config.currency,
                chargeAmount.withVat
            )
        }
    )
}
