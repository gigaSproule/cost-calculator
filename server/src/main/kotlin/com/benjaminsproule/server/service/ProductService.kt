package com.benjaminsproule.server.service

import com.benjaminsproule.server.dao.ProductDao
import com.benjaminsproule.server.model.Product
import com.benjaminsproule.server.model.ProductWithId

class ProductService(val dao: ProductDao) : Service<Product, ProductWithId> {
    override suspend fun save(product: Product): ProductWithId {
        return dao.save(product)
    }

    override suspend fun update(product: ProductWithId): ProductWithId {
        return dao.update(product)
    }

    override suspend fun findAll(): List<ProductWithId> {
        return dao.findAll()
    }

    override suspend fun findById(id: String): ProductWithId? {
        return dao.findById(id)
    }

    override suspend fun delete(id: String) {
        dao.delete(id)
    }
}
