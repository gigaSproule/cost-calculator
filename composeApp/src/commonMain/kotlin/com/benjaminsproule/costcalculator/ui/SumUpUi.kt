@file:OptIn(ExperimentalMaterial3Api::class)

package com.benjaminsproule.costcalculator.ui

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import androidx.lifecycle.viewmodel.compose.viewModel
import com.benjaminsproule.costcalculator.calculator.*
import com.benjaminsproule.costcalculator.ui.decimal.DecimalField
import com.benjaminsproule.costcalculator.ui.time.TimeField

@Composable
fun SumUpUi() {
    var state by remember { mutableStateOf(0) }

    Column {
        PrimaryTabRow(selectedTabIndex = state) {
            Tab(
                selected = state == 0,
                onClick = { state = 0 },
                text = { Text(text = "Cost of Sale") }
            )
            Tab(
                selected = state == 1,
                onClick = { state = 1 },
                text = { Text(text = "How much to charge") }
            )
        }
        if (state == 0) {
            CostOfSale()
        } else {
            HowMuchToCharge()
        }
    }
}

@Composable
private fun CostOfSale(configViewModel: ConfigViewModel = viewModel(factory = ConfigViewModel.Factory)) {
    var costOfSale by remember { mutableStateOf("0.00") }
    var paymentOption by remember { mutableStateOf(PaymentOption.CardReader) }
    var subscriptionOption by remember { mutableStateOf(SubscriptionOption.NoContract) }
    var saleBreakdown: SaleBreakdown? by remember { mutableStateOf(null) }
    val config by configViewModel.config.collectAsState()

    val sumUpCalculator = SumUpCalculator(config)

    Column(modifier = Modifier.padding(14.dp)) {
        DecimalField(
            label = { Text("Cost of sale") },
            value = costOfSale,
            onValueChange = { costOfSale = it },
        )
        Row(
            verticalAlignment = Alignment.CenterVertically,
        ) {
            EnumField("Payment option", paymentOption) { paymentOption = it }
        }
        Row(
            verticalAlignment = Alignment.CenterVertically,
        ) {
            EnumField("Subscription option", subscriptionOption) { subscriptionOption = it }
        }
        Row(horizontalArrangement = Arrangement.End, modifier = Modifier.fillMaxWidth()) {
            Button(modifier = Modifier.padding(end = 16.dp), onClick = {
                costOfSale = "0.00"
                paymentOption = PaymentOption.CardReader
                subscriptionOption = SubscriptionOption.NoContract
            }) {
                Text("Clear")
            }
            Button(onClick = {
                saleBreakdown =
                    sumUpCalculator.basedOnSale(
                        SumUpSale(
                            cost = costOfSale.toFloat(),
                            paymentOption = paymentOption,
                            subscriptionOption = subscriptionOption
                        )
                    )
            }) {
                Text("Calculate")
            }
        }
        DisplaySaleBreakdown(saleBreakdown, config)
    }
}

@Composable
private fun HowMuchToCharge(
    configViewModel: ConfigViewModel = viewModel(factory = ConfigViewModel.Factory),
    materialsViewModel: MaterialsViewModel = viewModel(factory = MaterialsViewModel.Factory)
) {
    var timeTaken by remember { mutableStateOf("00:00") }
    var materialCostsEntries: List<Material> by remember { mutableStateOf(emptyList()) }
    var paymentOption by remember { mutableStateOf(PaymentOption.CardReader) }
    var subscriptionOption by remember { mutableStateOf(SubscriptionOption.NoContract) }
    var chargeAmount: ChargeAmount? by remember { mutableStateOf(null) }
    val config by configViewModel.config.collectAsState()
    val materials by materialsViewModel.materials.collectAsState()

    val materialCosts by remember(materials) { mutableStateOf(materials) }

    val sumUpCalculator = SumUpCalculator(config)

    Column(modifier = Modifier.verticalScroll(rememberScrollState()).padding(14.dp)) {
        TimeField(
            label = {
                Text("Time taken")
            },
            value = timeTaken,
            onValueChange = { value ->
                timeTaken = value
            })
        CostOfMaterials(materialCosts) { material ->
            materialCostsEntries = materialCostsEntries.filter {
                it.name != material.name
            } + material
        }
        Row(
            verticalAlignment = Alignment.CenterVertically,
        ) {
            EnumField("Payment option", paymentOption) { paymentOption = it }
        }
        Row(
            verticalAlignment = Alignment.CenterVertically,
        ) {
            EnumField("Subscription option", subscriptionOption) {
                subscriptionOption = it
            }
        }
        Row(horizontalArrangement = Arrangement.End, modifier = Modifier.fillMaxWidth()) {
            Button(modifier = Modifier.padding(end = 16.dp), onClick = {
                timeTaken = "00:00"
                paymentOption = PaymentOption.CardReader
                subscriptionOption = SubscriptionOption.NoContract
            }) {
                Text("Clear")
            }
            Button(onClick = {
                val split = timeTaken.split(":")
                chargeAmount =
                    sumUpCalculator.howMuchToCharge(
                        SumUpCharge(
                            numberOfMinutes = ((split[0].toInt() * 60) + split[1].toInt()).toFloat(),
                            materialCosts = materialCostsEntries,
                            paymentOption = paymentOption,
                            subscriptionOption = subscriptionOption
                        )
                    )
            }) {
                Text("Calculate")
            }
        }
        DisplayChargeAmount(chargeAmount, config)
    }
}
