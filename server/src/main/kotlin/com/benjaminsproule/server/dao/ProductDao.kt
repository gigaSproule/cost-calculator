package com.benjaminsproule.server.dao

import com.benjaminsproule.server.model.Product
import com.benjaminsproule.server.model.ProductWithId

class ProductDao : Dao<Product, ProductWithId> {
    override fun save(product: Product): ProductWithId {}

    override fun update(product: ProductWithId): ProductWithId {
    }

    override fun findAll(): List<ProductWithId> {}

    override fun findById(id: String): ProductWithId? {}

    override fun delete(id: String) {}
}
