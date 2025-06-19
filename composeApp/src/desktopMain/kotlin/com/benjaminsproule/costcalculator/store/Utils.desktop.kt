package com.benjaminsproule.costcalculator.store

import java.util.UUID

fun getStorePath(): String {
    val osName = System.getProperty("os.name")?.lowercase() ?: ""
    val configDir = when {
        "windows" in osName -> System.getenv("LOCALAPPDATA")
        "linux" in osName -> System.getenv("XDG_CONFIG_HOME")
            ?: "${System.getenv("HOME")}/.config"

        "macos" in osName -> "${System.getenv("HOME")}/Library/Preferences"
        else -> ""
    }
    return "${configDir}/cost-calculator"
}

actual fun getUUID(): String = UUID.randomUUID().toString()
