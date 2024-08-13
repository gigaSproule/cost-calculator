package com.benjaminsproule.costcalculator.ui.decimal

import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material3.TextField
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.onFocusChanged
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.unit.dp

@Composable
fun DecimalField(value: String, onValueChange: (String) -> Unit, label: @Composable (() -> Unit)? = null) {
    val decimalFormatter = DecimalFormatter()

    TextField(
        label = label,
        modifier = Modifier.fillMaxWidth().padding(bottom = 16.dp).onFocusChanged {
            if (!it.isFocused) {
                onValueChange("%.02f".format(value.toFloatOrNull() ?: 0.0f))
            }
        },
        value = value,
        onValueChange = {
            onValueChange(decimalFormatter.cleanup(it))
        },
        keyboardOptions = KeyboardOptions.Default.copy(keyboardType = KeyboardType.Decimal),
        visualTransformation = DecimalVisualTransformation(decimalFormatter)
    )
}
