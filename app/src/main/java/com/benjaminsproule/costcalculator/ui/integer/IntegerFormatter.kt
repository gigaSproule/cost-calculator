package com.benjaminsproule.costcalculator.ui.integer

import java.text.DecimalFormatSymbols

class IntegerFormatter(
    symbols: DecimalFormatSymbols = DecimalFormatSymbols.getInstance()
) {

    private val thousandsSeparator = symbols.groupingSeparator

    fun cleanup(input: String): String {
        if (input.matches("\\D".toRegex())) return ""
        if (input.matches("0+".toRegex())) return "0"

        val sb = StringBuilder()

        for (char in input) {
            if (char.isDigit()) {
                sb.append(char)
                continue
            }
        }

        return sb.toString()
    }

    fun formatForVisual(input: String): String = input
        .reversed()
        .chunked(3)
        .joinToString(separator = thousandsSeparator.toString())
        .reversed()
}
