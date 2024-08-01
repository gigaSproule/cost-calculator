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
import kotlinx.coroutines.launch

@Composable
fun OptionsUi(
    configViewModel: ConfigViewModel = viewModel(factory = ConfigViewModel.Factory)
) {
    val coroutineScope = rememberCoroutineScope()
    val config by configViewModel.config.collectAsState()

    var markupPercentage by remember(config.markupPercentage) { mutableFloatStateOf(config.markupPercentage) }
    var hourlyRate by remember(config.hourlyRate) { mutableFloatStateOf(config.hourlyRate) }
    var taxRate by remember(config.taxRate) { mutableFloatStateOf(config.taxRate) }
    var vat by remember(config.vat) { mutableFloatStateOf(config.vat) }
    var currency by remember(config.currency) { mutableStateOf(config.currency) }

    return Column(modifier = Modifier.padding(14.dp)) {
        TextField(
            label = { Text("Markup percentage") },
            modifier = Modifier.fillMaxWidth().padding(bottom = 16.dp),
            value = "$markupPercentage",
            onValueChange = { markupPercentage = it.toFloatOrNull() ?: markupPercentage },
            keyboardOptions = KeyboardOptions.Default.copy(keyboardType = KeyboardType.Decimal),
            visualTransformation = CurrencyVisualTransformation()
        )
        TextField(
            label = { Text("Hourly rate") },
            modifier = Modifier.fillMaxWidth().padding(bottom = 16.dp),
            value = "$hourlyRate",
            onValueChange = { hourlyRate = it.toFloatOrNull() ?: hourlyRate },
            keyboardOptions = KeyboardOptions.Default.copy(keyboardType = KeyboardType.Decimal),
            visualTransformation = CurrencyVisualTransformation()
        )
        TextField(
            label = { Text("Tax rate") },
            modifier = Modifier.fillMaxWidth().padding(bottom = 16.dp),
            value = "$taxRate",
            onValueChange = { taxRate = it.toFloatOrNull() ?: taxRate },
            keyboardOptions = KeyboardOptions.Default.copy(keyboardType = KeyboardType.Decimal),
            visualTransformation = CurrencyVisualTransformation()
        )
        TextField(
            label = { Text("VAT") },
            modifier = Modifier.fillMaxWidth().padding(bottom = 16.dp),
            value = "$vat",
            onValueChange = { vat = it.toFloatOrNull() ?: vat },
            keyboardOptions = KeyboardOptions.Default.copy(keyboardType = KeyboardType.Decimal),
            visualTransformation = CurrencyVisualTransformation()
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
                            markupPercentage = markupPercentage,
                            hourlyRate = hourlyRate,
                            taxRate = taxRate,
                            vat = vat,
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
