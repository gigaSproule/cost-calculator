@file:OptIn(ExperimentalMaterial3Api::class)

package com.benjaminsproule.costcalculator.ui

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.unit.dp
import androidx.lifecycle.viewmodel.compose.viewModel
import com.benjaminsproule.costcalculator.calculator.*
import com.benjaminsproule.costcalculator.ui.decimal.DecimalField

@Composable
fun PaypalUi() {
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
                text = { Text(text = "How to much to charge") }
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
    var costOfDelivery by remember { mutableStateOf("0.00") }
    var location by remember { mutableStateOf(PaypalLocation.Local) }
    var saleBreakdown: SaleBreakdown? by remember { mutableStateOf(null) }
    val config by configViewModel.config.collectAsState()

    val paypalCalculator = PaypalCalculator(config)

    Column(modifier = Modifier.padding(14.dp)) {
        DecimalField(
            label = { Text("Cost of sale") },
            value = costOfSale,
            onValueChange = { costOfSale = it },
        )
        DecimalField(
            label = { Text("Cost of delivery") },
            value = costOfDelivery,
            onValueChange = { costOfDelivery = it },
        )
        Row(
            verticalAlignment = Alignment.CenterVertically,
        ) {
            EnumField("Location of sale", location) { location = it }
        }
        Row(horizontalArrangement = Arrangement.End, modifier = Modifier.fillMaxWidth()) {
            Button(modifier = Modifier.padding(end = 16.dp), onClick = {
                costOfSale = "0.00"
                costOfDelivery = "0.00"
                location = PaypalLocation.Local
            }) {
                Text("Clear")
            }
            Button(onClick = {
                saleBreakdown =
                    paypalCalculator.basedOnSale(
                        PaypalSale(
                            cost = costOfSale.toFloat(),
                            deliveryCosts = costOfDelivery.toFloat(),
                            location = location
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
    var timeTaken by remember { mutableFloatStateOf(0.0f) }
    var materialCostsEntries: List<Material> by remember { mutableStateOf(emptyList()) }
    var costOfDelivery by remember { mutableStateOf("0.00") }
    var location by remember { mutableStateOf(PaypalLocation.Local) }
    var chargeAmount: ChargeAmount? by remember { mutableStateOf(null) }
    val config by configViewModel.config.collectAsState()
    val materials by materialsViewModel.materials.collectAsState()

    val materialCosts by remember(materials) { mutableStateOf(materials) }

    val paypalCalculator = PaypalCalculator(config)

    Column(modifier = Modifier.verticalScroll(rememberScrollState()).padding(14.dp)) {
        TextField(
            label = { Text("Time taken") },
            modifier = Modifier.fillMaxWidth().padding(bottom = 16.dp),
            value = "%02d:%02d".format(
                *((timeTaken / 60.0).let { hours ->
                    arrayOf(hours.toInt(), ((hours - hours.toInt()) * 60).toInt())
                })
            ),
            onValueChange = {
                val split = it.split(":")
                timeTaken = split.let {
                    return@let if (it.size != 2) {
                        0.0f
                    } else {
                        (it.get(0).replace(" ", "").toInt() * 60.0f) + it.get(1).replace(" ", "").toInt()
                    }
                }
            },
            keyboardOptions = KeyboardOptions.Default.copy(
                keyboardType = KeyboardType.Number
            ),
        )
        CostOfMaterials(materialCosts) { material ->
            materialCostsEntries = materialCostsEntries.filter {
                it.name != material.name
            } + material
        }
        DecimalField(
            label = { Text("Cost of delivery") },
            value = costOfDelivery,
            onValueChange = { costOfDelivery = it },
        )
        Row(
            verticalAlignment = Alignment.CenterVertically,
        ) {
            EnumField("Location of sale", location) { location = it }
        }
        Row(horizontalArrangement = Arrangement.End, modifier = Modifier.fillMaxWidth()) {
            Button(modifier = Modifier.padding(end = 16.dp), onClick = {
                timeTaken = 0.0f
                costOfDelivery = "0.00"
                location = PaypalLocation.Local
            }) {
                Text("Clear")
            }
            Button(onClick = {
                chargeAmount =
                    paypalCalculator.howMuchToCharge(
                        PaypalCharge(
                            numberOfMinutes = timeTaken,
                            materialCosts = materialCostsEntries,
                            deliveryCosts = costOfDelivery.toFloat(),
                            location = location
                        )
                    )
            }) {
                Text("Calculate")
            }
        }
        DisplayChargeAmount(chargeAmount, config)
    }
}
