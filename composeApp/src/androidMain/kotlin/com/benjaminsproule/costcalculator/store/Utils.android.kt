package com.benjaminsproule.costcalculator.store

import java.util.UUID

actual fun getUUID(): String = java.util.UUID.randomUUID().toString()