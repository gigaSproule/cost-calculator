package com.benjaminsproule.server.service

import com.benjaminsproule.server.dao.MaterialStockDao
import com.benjaminsproule.server.model.MaterialStock
import com.benjaminsproule.server.model.MaterialStockWithId

class MaterialStockService(val dao: MaterialStockDao) : Service<MaterialStock, MaterialStockWithId> {
    override fun create(materialStock: MaterialStock): MaterialStockWithId {
        return dao.save(materialStock)
    }

    override fun update(materialStock: MaterialStockWithId): MaterialStockWithId {
        return dao.update(materialStock)
    }

    override fun findAll(): List<MaterialStockWithId> {
        return dao.findAll()
    }

    override fun findById(id: String): MaterialStockWithId? {
        return dao.findById(id)
    }

    override fun delete(id: String) {
        dao.delete(id)
    }
}
