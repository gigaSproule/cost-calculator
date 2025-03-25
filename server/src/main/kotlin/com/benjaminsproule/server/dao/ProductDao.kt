package com.benjaminsproule.server.dao

import com.benjaminsproule.server.model.Product
import com.benjaminsproule.server.model.ProductWithId

class ProductDao : Dao<Product, ProductWithId> {
    override fun save(product: Product): ProductWithId {
        return TODO("Provide the return value")
    }

    override fun update(product: ProductWithId): ProductWithId {
        return TODO("Provide the return value")
    }

    override fun findAll(): List<ProductWithId> {
        return TODO("Provide the return value")
    }

    override fun findById(id: String): ProductWithId? {
        return TODO("Provide the return value")
    }

    override fun delete(id: String) {}
}
