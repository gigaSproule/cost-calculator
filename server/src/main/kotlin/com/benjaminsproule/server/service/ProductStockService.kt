package com.benjaminsproule.server.service

import com.benjaminsproule.server.dao.MaterialStockDao
import com.benjaminsproule.server.dao.ProductStockDao
import com.benjaminsproule.server.model.ProductStock
import com.benjaminsproule.server.model.ProductStockWithId

class ProductStockService(private val dao: ProductStockDao, private val materialStockDao: MaterialStockDao) :
    Service<ProductStock, ProductStockWithId> {
    override fun create(productStock: ProductStock): ProductStockWithId {
        return dao.save(productStock)
    }

    override fun update(productStock: ProductStockWithId): ProductStockWithId {
        return dao.update(productStock)
    }

    override fun findAll(): List<ProductStockWithId> {
        return dao.findAll()
    }

    override fun findById(id: String): ProductStockWithId? {
        return dao.findById(id)
    }

    override fun delete(id: String) {
        dao.delete(id)
    }

    fun addStock(productId: String, quantity: Int) {
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
