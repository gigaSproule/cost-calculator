package com.benjaminsproule.costcalculator.ui

import androidx.compose.ui.text.AnnotatedString
import androidx.compose.ui.text.input.OffsetMapping
import androidx.compose.ui.text.input.TransformedText
import androidx.compose.ui.text.input.VisualTransformation

class CurrencyVisualTransformation : VisualTransformation {
    override fun filter(text: AnnotatedString): TransformedText {
        val newText = AnnotatedString(
            text = "%.02f".format(text.text.toFloat()),
            spanStyles = text.spanStyles,
            paragraphStyles = text.paragraphStyles
        )
        val maxLength = if (text.text.length < newText.text.length) {
            text.text.length
        } else {
            newText.text.length
        }
        val offsetMapping = FixedCursorOffsetMapping(maxLength)
        return TransformedText(newText, offsetMapping)
    }
}

class FixedCursorOffsetMapping(val maxLength: Int) : OffsetMapping {
    override fun originalToTransformed(offset: Int): Int =
        if (offset > maxLength) {
            maxLength
        } else {
            offset
        }

    override fun transformedToOriginal(offset: Int): Int =
        if (offset > maxLength) {
            maxLength
        } else {
            offset
        }
}
