@file:OptIn(ExperimentalMaterial3Api::class)

package com.benjaminsproule.costcalculator.ui

import android.util.Log
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.grid.GridCells
import androidx.compose.foundation.lazy.grid.LazyVerticalGrid
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.unit.dp
import androidx.lifecycle.viewmodel.compose.viewModel
import com.benjaminsproule.costcalculator.calculator.ChargeAmount
import com.benjaminsproule.costcalculator.calculator.EtsyCalculator
import com.benjaminsproule.costcalculator.calculator.Material
import com.benjaminsproule.costcalculator.calculator.SaleBreakdown
import com.benjaminsproule.costcalculator.ui.decimal.DecimalField
import com.benjaminsproule.costcalculator.ui.integer.IntegerField

@Composable
fun EtsyUi() {
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
fun CostOfSale(configViewModel: ConfigViewModel = viewModel(factory = ConfigViewModel.Factory)) {
    var costOfSale by remember { mutableStateOf("0.00") }
    var costOfDelivery by remember { mutableStateOf("0.00") }
    var offsiteAdsUsed by remember { mutableStateOf(false) }
    var saleBreakdown: SaleBreakdown? by remember { mutableStateOf(null) }
    val config by configViewModel.config.collectAsState()

    val etsyCalculator = EtsyCalculator(config)

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
            Text("Offsite ads used?")
            Checkbox(
                checked = offsiteAdsUsed,
                onCheckedChange = { offsiteAdsUsed = it },
            )
        }
        Row(horizontalArrangement = Arrangement.End, modifier = Modifier.fillMaxWidth()) {
            Button(modifier = Modifier.padding(end = 16.dp), onClick = {
                costOfSale = "0.00"
                costOfDelivery = "0.00"
                offsiteAdsUsed = false
            }) {
                Text("Clear")
            }
            Button(onClick = {
                saleBreakdown =
                    etsyCalculator.basedOnSale(costOfSale.toFloat(), costOfDelivery.toFloat(), offsiteAdsUsed)
            }) {
                Text("Calculate")
            }
        }
        LazyVerticalGrid(
            columns = GridCells.Fixed(2),
            verticalArrangement = Arrangement.Center
        ) {
            item {
                Text(
                    modifier = Modifier.padding(end = 8.dp),
                    text = "Sale"
                )
            }
            item {
                Text(
                    if (saleBreakdown == null) {
                        ""
                    } else {
                        "${config.currency}%.2f".format(saleBreakdown!!.sale)
                    }
                )
            }
            item {
                Text(
                    modifier = Modifier.padding(end = 8.dp), text = "Delivery costs"
                )
            }
            item {
                Text(
                    if (saleBreakdown == null) {
                        ""
                    } else {
                        "${config.currency}%.2f".format(saleBreakdown!!.deliveryCosts)
                    }
                )
            }
            item {
                Text(modifier = Modifier.padding(end = 8.dp), text = "Transaction cost")
            }
            item {
                Text(
                    if (saleBreakdown == null) {
                        ""
                    } else {
                        "${config.currency}%.2f".format(saleBreakdown!!.transactionCost)
                    }
                )
            }
            item {
                Text(
                    modifier = Modifier.padding(end = 8.dp),
                    text = "Payment processing cost"
                )
            }
            item {
                Text(
                    if (saleBreakdown == null) {
                        ""
                    } else {
                        "${config.currency}%.2f".format(saleBreakdown!!.paymentProcessingCost)
                    }
                )
            }
            item {
                Text(modifier = Modifier.padding(end = 8.dp), text = "Offsite ads cost")
            }
            item {
                Text(
                    if (saleBreakdown == null) {
                        ""
                    } else {
                        "${config.currency}%.2f".format(saleBreakdown!!.offsiteAdsCost)
                    }
                )
            }
            item {
                Text(modifier = Modifier.padding(end = 8.dp), text = "Regulatory operating fee")
            }
            item {
                Text(
                    if (saleBreakdown == null) {
                        ""
                    } else {
                        "${config.currency}%.2f".format(saleBreakdown!!.regulatoryOperatingFee)
                    }
                )
            }
            item {
                Text(modifier = Modifier.padding(end = 8.dp), text = "VAT paid by buyer")
            }
            item {
                Text(
                    if (saleBreakdown == null) {
                        ""
                    } else {
                        "${config.currency}%.2f".format(saleBreakdown!!.vatPaidByBuyer)
                    }
                )
            }
            item {
                Text(modifier = Modifier.padding(end = 8.dp), text = "VAT on seller fees")
            }
            item {
                Text(
                    if (saleBreakdown == null) {
                        ""
                    } else {
                        "${config.currency}%.2f".format(saleBreakdown!!.vatOnSellerFees)
                    }
                )
            }
            item {
                Text(modifier = Modifier.padding(end = 8.dp), text = "Total fees")
            }
            item {
                Text(
                    if (saleBreakdown == null) {
                        ""
                    } else {
                        "${config.currency}%.2f".format(saleBreakdown!!.totalFees)
                    }
                )
            }
            item {
                Text(modifier = Modifier.padding(end = 8.dp), text = "Total fees with VAT")
            }
            item {
                Text(
                    if (saleBreakdown == null) {
                        ""
                    } else {
                        "${config.currency}%.2f".format(saleBreakdown!!.totalFeesWithVat)
                    }
                )
            }
            item {
                Text(modifier = Modifier.padding(end = 8.dp), text = "Tax")
            }
            item {
                Text(
                    if (saleBreakdown == null) {
                        ""
                    } else {
                        "${config.currency}%.2f".format(saleBreakdown!!.tax)
                    }
                )
            }
            item {
                Text(modifier = Modifier.padding(end = 8.dp), text = "Revenue")
            }
            item {
                Text(
                    if (saleBreakdown == null) {
                        ""
                    } else {
                        "${config.currency}%.2f".format(saleBreakdown!!.revenue)
                    }
                )
            }
            item {
                Text(modifier = Modifier.padding(end = 8.dp), text = "Percentage kept")
            }
            item {
                Text(
                    if (saleBreakdown == null) {
                        ""
                    } else {
                        "%.2f%%".format(saleBreakdown!!.percentageKept)
                    }
                )
            }
            item {
                Text(modifier = Modifier.padding(end = 8.dp), text = "Max working hours")
            }
            item {
                Text(
                    if (saleBreakdown == null) {
                        ""
                    } else {
                        "%d:%02d".format(
                            saleBreakdown!!.maxWorkingHours.toInt(),
                            ((saleBreakdown!!.maxWorkingHours - (saleBreakdown!!.maxWorkingHours.toInt())) * 60).toInt()
                        )
                    }
                )
            }
        }
    }
}

@Composable
fun HowMuchToCharge(
    configViewModel: ConfigViewModel = viewModel(factory = ConfigViewModel.Factory),
    materialsViewModel: MaterialsViewModel = viewModel(factory = MaterialsViewModel.Factory)
) {
    var timeTaken by remember { mutableFloatStateOf(0.0f) }
    var materialCostsEntries: List<Material> by remember { mutableStateOf(emptyList()) }
    var costOfDelivery by remember { mutableStateOf("0.00") }
    var offsiteAdsUsed by remember { mutableStateOf(false) }
    var chargeAmount: ChargeAmount? by remember { mutableStateOf(null) }
    val config by configViewModel.config.collectAsState()
    val materials by materialsViewModel.materials.collectAsState()

    val materialCosts by remember(materials) { mutableStateOf(materials) }

    val etsyCalculator = EtsyCalculator(config)

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
        Card(
            modifier = Modifier.background(Color.Transparent).fillMaxWidth()
                .padding(bottom = 16.dp).border(1.dp, Color.Black)
        ) {
            Text(modifier = Modifier.padding(8.dp), text = "Cost of materials")
            materialCosts.map { material ->
                Row(modifier = Modifier.fillMaxWidth()) {
                    var materialCost by remember { mutableStateOf("0") }
                    IntegerField(
                        label = { Text(material.name) },
                        value = materialCost,
                        onValueChange = {
                            materialCost = it
                            materialCostsEntries = materialCostsEntries.filter {
                                it.name != material.name
                            } + Material(material.name, (materialCost.toIntOrNull() ?: 0) * material.value)
                        },
                    )
                }
            }
        }
        DecimalField(
            label = { Text("Cost of delivery") },
            value = costOfDelivery,
            onValueChange = { costOfDelivery = it },
        )
        Row(
            verticalAlignment = Alignment.CenterVertically,
        ) {
            Text("Offsite ads used?")
            Checkbox(
                checked = offsiteAdsUsed,
                onCheckedChange = { offsiteAdsUsed = it },
            )
        }
        Row(horizontalArrangement = Arrangement.End, modifier = Modifier.fillMaxWidth()) {
            Button(modifier = Modifier.padding(end = 16.dp), onClick = {
                timeTaken = 0.0f
                costOfDelivery = "0.00"
                offsiteAdsUsed = false
            }) {
                Text("Clear")
            }
            Button(onClick = {
                chargeAmount =
                    etsyCalculator.howMuchToCharge(
                        timeTaken,
                        materialCostsEntries,
                        costOfDelivery.toFloat(),
                        offsiteAdsUsed
                    )
            }) {
                Text("Calculate")
            }
        }
        Text(
            if (chargeAmount == null) {
                ""
            } else {
                "%s%.02f (with VAT %s%.02f)".format(
                    config.currency,
                    chargeAmount!!.totalToCharge,
                    config.currency,
                    chargeAmount!!.withVat
                )
            }
        )
    }
}
