package com.benjaminsproule.costcalculator.ui.time

import java.text.DecimalFormatSymbols

class TimeFormatter(
    symbols: DecimalFormatSymbols = DecimalFormatSymbols.getInstance()
) {

    private val timeSeparator = ':'

    fun cleanup(input: String): String {
//        if (input.matches("\\D".toRegex())) return ""
//        if (input.matches("0+".toRegex())) return "0"

        val sb = StringBuilder()

        var hasTimeSep = false

        for (char in input) {
            if (char.isDigit()) {
                sb.append(char)
                continue
            }
            if (char == timeSeparator && !hasTimeSep && sb.isNotEmpty()) {
                sb.append(char)
                hasTimeSep = true
            }
        }

        return sb.toString()
    }

    fun formatForVisual(input: String): String {
        val split = input.split(timeSeparator)

        val intPart = split[0]

        val fractionPart = split.getOrNull(1)

        return if (fractionPart == null) intPart else intPart + timeSeparator + fractionPart
    }
}
