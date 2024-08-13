package com.benjaminsproule.costcalculator.ui.integer

import androidx.compose.ui.text.AnnotatedString
import androidx.compose.ui.text.input.TransformedText
import androidx.compose.ui.text.input.VisualTransformation
import com.benjaminsproule.costcalculator.ui.FixedCursorOffsetMapping

class IntegerVisualTransformation(private val integerFormatter: IntegerFormatter = IntegerFormatter()) :
    VisualTransformation {
    override fun filter(text: AnnotatedString): TransformedText {
        val inputText = text.text
        val formattedNumber = integerFormatter.formatForVisual(inputText)
        val newText = AnnotatedString(
            text = formattedNumber,
            spanStyles = text.spanStyles,
            paragraphStyles = text.paragraphStyles
        )
        val offsetMapping = FixedCursorOffsetMapping(inputText.length, formattedNumber.length)
        return TransformedText(newText, offsetMapping)
    }
}
