package com.benjaminsproule.server.service

import com.benjaminsproule.server.dao.MaterialStockDao
import com.benjaminsproule.server.dao.ProductStockDao
import com.benjaminsproule.server.model.ProductStock
import com.benjaminsproule.server.model.ProductStockWithId

class ProductStockService(private val dao: ProductStockDao, private val materialStockDao: MaterialStockDao) :
    Service<ProductStock, ProductStockWithId> {
    override suspend fun save(productStock: ProductStock): ProductStockWithId {
        return dao.save(productStock)
    }

    override suspend fun update(productStock: ProductStockWithId): ProductStockWithId {
        return dao.update(productStock)
    }

    override suspend fun findAll(): List<ProductStockWithId> {
        return dao.findAll()
    }

    override suspend fun findById(id: String): ProductStockWithId? {
        return dao.findById(id)
    }

    override suspend fun delete(id: String) {
        dao.delete(id)
    }

    suspend fun addStock(productId: String, quantity: Int) {
        dao.findByProductId(productId)?.let {
            it.product.materials.forEach { material ->
                materialStockDao.findByMaterialId(material.id)?.let { materialStock ->
                    materialStockDao.update(materialStock.copy(quantity = materialStock.quantity - 1))
                }
            }
            update(it.copy(quantity = it.quantity + quantity))
        }
    }
}
