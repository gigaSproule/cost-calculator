package com.benjaminsproule.server.dao

import com.benjaminsproule.server.model.ProductStock
import com.benjaminsproule.server.model.ProductStockWithId

class ProductStockDao : Dao<ProductStock, ProductStockWithId> {
    override fun save(productStock: ProductStock): ProductStockWithId {
        TODO("Not yet implemented")
    }

    override fun update(productStock: ProductStockWithId): ProductStockWithId {
        TODO("Not yet implemented")
    }

    override fun findAll(): List<ProductStockWithId> {
        TODO("Not yet implemented")
    }

    override fun findById(id: String): ProductStockWithId? {
        TODO("Not yet implemented")
    }

    override fun delete(id: String) {
        TODO("Not yet implemented")
    }

    fun findByProductId(productId: String): ProductStockWithId? {
        TODO("Not yet implemented")
    }
}
