package com.benjaminsproule.server.service

import com.benjaminsproule.server.dao.MaterialDao
import com.benjaminsproule.server.model.Material
import com.benjaminsproule.server.model.MaterialWithId

@org.springframework.stereotype.Service
class MaterialService(val dao: MaterialDao) : Service<Material, MaterialWithId> {
    override suspend fun save(material: Material): MaterialWithId {
        return dao.save(material)
    }

    override suspend fun update(material: MaterialWithId): MaterialWithId {
        return dao.update(material)
    }

    override suspend fun findAll(): List<MaterialWithId> {
        return dao.findAll()
    }

    override suspend fun findById(id: String): MaterialWithId? {
        return dao.findById(id)
    }

    override suspend fun delete(id: String) {
        dao.delete(id)
    }
}
