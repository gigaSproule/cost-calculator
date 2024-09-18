package com.benjaminsproule.server.model

data class Product(val name: String, val materials: List<MaterialWithId>)

data class ProductWithId(override val id: String, val name: String, val materials: List<MaterialWithId>) :
    WithId<Product>()
