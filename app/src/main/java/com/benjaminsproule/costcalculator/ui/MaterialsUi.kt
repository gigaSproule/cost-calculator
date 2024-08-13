@file:OptIn(ExperimentalMaterial3Api::class)

package com.benjaminsproule.costcalculator.ui

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.unit.dp
import androidx.lifecycle.viewmodel.compose.viewModel
import com.benjaminsproule.costcalculator.store.StoredMaterial
import com.benjaminsproule.costcalculator.ui.decimal.DecimalVisualTransformation
import kotlinx.coroutines.launch

@Composable
fun MaterialsUi(
    materialsViewModel: MaterialsViewModel = viewModel(factory = MaterialsViewModel.Factory)
) {
    val coroutineScope = rememberCoroutineScope()
    val materials by materialsViewModel.materials.collectAsState()

    var editableMaterials by remember(materials) { mutableStateOf(materials) }
    var updatedMaterials by remember(materials) { mutableStateOf(materials) }
    fun onUpdateMaterial(updatedMaterial: StoredMaterial) {
        updatedMaterials = updatedMaterials.filter {
            it.id != updatedMaterial.id
        } + updatedMaterial
    }

    Column(modifier = Modifier.padding(14.dp).verticalScroll(rememberScrollState())) {
        editableMaterials.map {
            MaterialRow(material = it, onUpdateMaterial = ::onUpdateMaterial)
        }
        MaterialRow(onUpdateMaterial = ::onUpdateMaterial)
        Row(horizontalArrangement = Arrangement.End, modifier = Modifier.fillMaxWidth()) {
            Button(
                onClick = {
                    coroutineScope.launch {
                        materialsViewModel.storeMaterials(
                            updatedMaterials
                        )
                    }
                }) {
                Text("Save")
            }
        }
    }
}

@Composable
private fun MaterialRow(material: StoredMaterial = StoredMaterial(), onUpdateMaterial: (StoredMaterial) -> Unit) {
    var rememberedMaterial by remember(material) { mutableStateOf(material) }
    var name by remember(material) { mutableStateOf(material.name) }
    var quantityPerPack by remember(material) { mutableFloatStateOf(material.quantityPerPack) }
    var pricePerPack by remember(material) { mutableFloatStateOf(material.pricePerPack) }

    Row(modifier = Modifier.padding(bottom = 16.dp)) {
        Card(modifier = Modifier.fillMaxWidth()) {
            TextField(
                modifier = Modifier.fillMaxWidth(),
                label = { Text("Name") },
                value = name,
                onValueChange = {
                    name = it
                    rememberedMaterial.name = it
                    onUpdateMaterial(rememberedMaterial)
                }
            )
            TextField(
                modifier = Modifier.fillMaxWidth(),
                label = { Text("Quantity per pack") },
                value = "$quantityPerPack",
                onValueChange = {
                    quantityPerPack = it.toFloatOrNull() ?: 0.0f
                    rememberedMaterial.quantityPerPack = quantityPerPack
                    onUpdateMaterial(rememberedMaterial)
                },
                keyboardOptions = KeyboardOptions.Default.copy(keyboardType = KeyboardType.Decimal),
                visualTransformation = DecimalVisualTransformation()
            )
            TextField(
                modifier = Modifier.fillMaxWidth(),
                label = { Text("Price per pack") },
                value = "$pricePerPack",
                onValueChange = {
                    pricePerPack = it.toFloatOrNull() ?: 0.0f
                    rememberedMaterial.pricePerPack = pricePerPack
                    onUpdateMaterial(rememberedMaterial)
                },
                keyboardOptions = KeyboardOptions.Default.copy(keyboardType = KeyboardType.Decimal),
                visualTransformation = DecimalVisualTransformation()
            )
        }
    }
}
