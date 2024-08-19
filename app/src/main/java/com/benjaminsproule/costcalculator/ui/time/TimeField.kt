package com.benjaminsproule.costcalculator.ui.time

import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material3.TextField
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.onFocusChanged
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.unit.dp
import kotlin.math.ceil

@Composable
fun TimeField(value: String, onValueChange: (String) -> Unit, label: @Composable (() -> Unit)? = null) {
    val timeFormatter = TimeFormatter()

    TextField(
        label = label,
        modifier = Modifier.fillMaxWidth().padding(bottom = 16.dp).onFocusChanged {
            if (!it.isFocused) {
                val chunks = value.split(":")
                val minutes = if (chunks.size > 1) {
                    (chunks[0].toInt() * 60) + chunks[1].toInt()
                } else if (chunks.size == 1) {
                    chunks[0].toIntOrNull() ?: 0
                } else {
                    0
                }
                onValueChange(
                    "%02d:%02d".format(
                        *((minutes / 60.0).let { hours ->
                            arrayOf(hours.toInt(), ceil((hours - hours.toInt()) * 60.0).toInt())
                        })
                    )
                )
            }
        },
        value = value,
        onValueChange = {
            onValueChange(timeFormatter.cleanup(it))
        },
        keyboardOptions = KeyboardOptions.Default.copy(keyboardType = KeyboardType.Decimal),
        visualTransformation = TimeVisualTransformation(timeFormatter)
    )
}
