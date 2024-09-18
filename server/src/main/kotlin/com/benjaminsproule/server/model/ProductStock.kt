package com.benjaminsproule.server.model

data class ProductStock(val product: Product, val quantity: Int)

data class ProductStockWithId(override val id: String, val product: Product, val quantity: Int) : WithId<ProductStock>()
