package com.benjaminsproule.server.model

data class MaterialStock(val material: Material, val quantity: Int)

data class MaterialStockWithId(override val id: String, val material: Material, val quantity: Int) : WithId<MaterialStock>()
