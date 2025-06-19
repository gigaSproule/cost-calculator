package com.benjaminsproule.costcalculator.ui.time

import androidx.compose.ui.text.AnnotatedString
import androidx.compose.ui.text.input.TransformedText
import androidx.compose.ui.text.input.VisualTransformation
import com.benjaminsproule.costcalculator.ui.FixedCursorOffsetMapping

class TimeVisualTransformation(private val timeFormatter: TimeFormatter = TimeFormatter()) :
    VisualTransformation {
    override fun filter(text: AnnotatedString): TransformedText {
        val inputText = text.text
        val formattedNumber = timeFormatter.formatForVisual(inputText)
        val newText = AnnotatedString(
            text = formattedNumber,
            spanStyles = text.spanStyles,
            paragraphStyles = text.paragraphStyles
        )
        val offsetMapping = FixedCursorOffsetMapping(inputText.length, formattedNumber.length)
        return TransformedText(newText, offsetMapping)
    }
}
