package com.benjaminsproule.costcalculator.ui.integer

import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material3.TextField
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.onFocusChanged
import androidx.compose.ui.text.input.KeyboardType

@Composable
fun IntegerField(value: String, onValueChange: (String) -> Unit, label: @Composable (() -> Unit)? = null) {
    val integerFormatter = IntegerFormatter()

    TextField(
        label = label,
        modifier = Modifier.fillMaxWidth().onFocusChanged {
            if (!it.isFocused) {
                onValueChange((value.toIntOrNull() ?: 0).toString())
            }
        },
        value = value,
        onValueChange = {
            onValueChange(integerFormatter.cleanup(it))
        },
        keyboardOptions = KeyboardOptions.Default.copy(keyboardType = KeyboardType.Decimal),
        visualTransformation = IntegerVisualTransformation(integerFormatter)
    )
}
