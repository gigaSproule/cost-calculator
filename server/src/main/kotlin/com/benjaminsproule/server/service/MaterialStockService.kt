package com.benjaminsproule.server.service

import com.benjaminsproule.server.dao.MaterialStockDao
import com.benjaminsproule.server.model.MaterialStock
import com.benjaminsproule.server.model.MaterialStockWithId

class MaterialStockService(val dao: MaterialStockDao) : Service<MaterialStock, MaterialStockWithId> {
    override suspend fun save(materialStock: MaterialStock): MaterialStockWithId {
        return dao.save(materialStock)
    }

    override suspend fun update(materialStock: MaterialStockWithId): MaterialStockWithId {
        return dao.update(materialStock)
    }

    override suspend fun findAll(): List<MaterialStockWithId> {
        return dao.findAll()
    }

    override suspend fun findById(id: String): MaterialStockWithId? {
        return dao.findById(id)
    }

    override suspend fun delete(id: String) {
        dao.delete(id)
    }
}
