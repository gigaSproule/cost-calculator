package com.benjaminsproule.costcalculator.ui.decimal

import androidx.compose.ui.text.AnnotatedString
import androidx.compose.ui.text.input.TransformedText
import androidx.compose.ui.text.input.VisualTransformation
import com.benjaminsproule.costcalculator.ui.FixedCursorOffsetMapping

class DecimalVisualTransformation(private val decimalFormatter: DecimalFormatter = DecimalFormatter()) :
    VisualTransformation {
    override fun filter(text: AnnotatedString): TransformedText {
        val inputText = text.text
        val formattedNumber = decimalFormatter.formatForVisual(inputText)
        val newText = AnnotatedString(
            text = formattedNumber,
            spanStyles = text.spanStyles,
            paragraphStyles = text.paragraphStyles
        )
        val offsetMapping = FixedCursorOffsetMapping(inputText.length, formattedNumber.length)
        return TransformedText(newText, offsetMapping)
    }
}
