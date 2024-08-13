@file:OptIn(ExperimentalMaterial3Api::class)

package com.benjaminsproule.costcalculator.ui

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material3.Button
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Text
import androidx.compose.material3.TextField
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.unit.dp
import androidx.lifecycle.viewmodel.compose.viewModel
import com.benjaminsproule.costcalculator.store.Config
import com.benjaminsproule.costcalculator.ui.decimal.DecimalField
import kotlinx.coroutines.launch

@Composable
fun OptionsUi(
    configViewModel: ConfigViewModel = viewModel(factory = ConfigViewModel.Factory)
) {
    val coroutineScope = rememberCoroutineScope()
    val config by configViewModel.config.collectAsState()

    var markupPercentage by remember(config.markupPercentage) { mutableStateOf(config.markupPercentage.toString()) }
    var hourlyRate by remember(config.hourlyRate) { mutableStateOf(config.hourlyRate.toString()) }
    var taxRate by remember(config.taxRate) { mutableStateOf(config.taxRate.toString()) }
    var vat by remember(config.vat) { mutableStateOf(config.vat.toString()) }
    var currency by remember(config.currency) { mutableStateOf(config.currency) }

    return Column(modifier = Modifier.padding(14.dp)) {
        DecimalField(
            label = { Text("Markup percentage") },
            value = markupPercentage,
            onValueChange = { markupPercentage = it },
        )
        DecimalField(
            label = { Text("Hourly rate") },
            value = hourlyRate,
            onValueChange = { hourlyRate = it },
        )
        DecimalField(
            label = { Text("Tax rate") },
            value = taxRate,
            onValueChange = { taxRate = it },
        )
        DecimalField(
            label = { Text("VAT") },
            value = vat,
            onValueChange = { vat = it },
        )
        TextField(
            label = { Text("Currency") },
            modifier = Modifier.fillMaxWidth().padding(bottom = 16.dp),
            value = currency,
            onValueChange = { currency = it.ifEmpty { currency }.take(1) },
            keyboardOptions = KeyboardOptions.Default.copy(keyboardType = KeyboardType.Text),
        )
        Row(horizontalArrangement = Arrangement.End, modifier = Modifier.fillMaxWidth()) {
            Button(modifier = Modifier.padding(end = 16.dp), onClick = {
                coroutineScope.launch {
                    configViewModel.resetToDefaults()
                }
            }) {
                Text("Reset to defaults")
            }
            Button(onClick = {
                coroutineScope.launch {
                    configViewModel.storeConfig(
                        Config(
                            markupPercentage = markupPercentage.toFloat(),
                            hourlyRate = hourlyRate.toFloat(),
                            taxRate = taxRate.toFloat(),
                            vat = vat.toFloat(),
                            currency = currency
                        )
                    )
                }
            }) {
                Text("Save")
            }
        }
    }
}
