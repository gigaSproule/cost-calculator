package com.benjaminsproule.server.dao

import com.benjaminsproule.server.model.MaterialStock
import com.benjaminsproule.server.model.MaterialStockWithId

class MaterialStockDao : Dao<MaterialStock, MaterialStockWithId> {
    override fun save(materialStock: MaterialStock): MaterialStockWithId {
        TODO("Not yet implemented")
    }

    override fun update(materialStock: MaterialStockWithId): MaterialStockWithId {
        TODO("Not yet implemented")
    }

    override fun findAll(): List<MaterialStockWithId> {
        TODO("Not yet implemented")
    }

    override fun findById(id: String): MaterialStockWithId? {
        TODO("Not yet implemented")
    }

    override fun delete(id: String) {
        TODO("Not yet implemented")
    }

    fun findByMaterialId(materialId: String): MaterialStockWithId? {
        TODO("Not yet implemented")
    }
}
