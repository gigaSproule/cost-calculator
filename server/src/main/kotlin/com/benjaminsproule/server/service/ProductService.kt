package com.benjaminsproule.server.service

import com.benjaminsproule.server.dao.ProductDao
import com.benjaminsproule.server.model.Product
import com.benjaminsproule.server.model.ProductWithId

class ProductService(val dao: ProductDao) : Service<Product, ProductWithId> {
    override fun create(product: Product): ProductWithId {
        return dao.save(product)
    }

    override fun update(product: ProductWithId): ProductWithId {
        return dao.update(product)
    }

    override fun findAll(): List<ProductWithId> {
        return dao.findAll()
    }

    override fun findById(id: String): ProductWithId? {
        return dao.findById(id)
    }

    override fun delete(id: String) {
        dao.delete(id)
    }
}
