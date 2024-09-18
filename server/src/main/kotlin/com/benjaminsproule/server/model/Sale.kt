package com.benjaminsproule.server.model

enum class SaleStatus {
    Created,
    Completed,
    Cancelled
}

data class SaleItem(val product: Product, val price: Double)

data class Sale(val saleItems: List<SaleItem>, val status: SaleStatus)

data class SaleWithId(override val id: String, val saleItems: List<SaleItem>, val status: SaleStatus) : WithId<Sale>()
