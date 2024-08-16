package com.benjaminsproule.costcalculator.ui

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Card
import androidx.compose.material3.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import com.benjaminsproule.costcalculator.calculator.Material
import com.benjaminsproule.costcalculator.store.StoredMaterial
import com.benjaminsproule.costcalculator.ui.integer.IntegerField

@Composable
fun CostOfMaterials(
    materialCosts: List<StoredMaterial>,
    onValueChanged: (Material) -> Unit
) {
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
                        onValueChanged(Material(material.name, (materialCost.toIntOrNull() ?: 0) * material.value))
                    },
                )
            }
        }
    }
}
