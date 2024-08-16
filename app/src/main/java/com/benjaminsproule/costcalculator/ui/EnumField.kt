package com.benjaminsproule.costcalculator.ui

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.benjaminsproule.costcalculator.calculator.Option

@OptIn(ExperimentalMaterial3Api::class)
@Composable
inline fun <reified T> EnumField(
    label: String,
    value: T,
    crossinline onValueChange: (T) -> Unit
) where T : Enum<out T>, T : Option {
    var expanded by remember { mutableStateOf(false) }
    Column {
        Text(label)
        ExposedDropdownMenuBox(expanded = expanded, onExpandedChange = {
            expanded = !expanded
        }, modifier = Modifier.fillMaxWidth().padding(vertical = 8.dp)) {
            TextField(
                value = value.label,
                onValueChange = {},
                readOnly = true,
                trailingIcon = { ExposedDropdownMenuDefaults.TrailingIcon(expanded = expanded) },
                modifier = Modifier.menuAnchor().fillMaxWidth()
            )
            DropdownMenu(
                expanded = expanded,
                onDismissRequest = { expanded = false },
                modifier = Modifier.fillMaxWidth()
            ) {
                T::class.java.enumConstants!!.map { option ->
                    DropdownMenuItem(
                        text = { Text(option.label) },
                        onClick = {
                            onValueChange(option)
                            expanded = false
                        }
                    )
                }
            }
        }
    }
}
