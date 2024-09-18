package com.benjaminsproule.server.service

import com.benjaminsproule.server.dao.MaterialDao
import com.benjaminsproule.server.model.Material
import com.benjaminsproule.server.model.MaterialWithId

class MaterialService(val dao: MaterialDao) : Service<Material, MaterialWithId> {
    override fun create(material: Material): MaterialWithId {
        return dao.save(material)
    }

    override fun update(material: MaterialWithId): MaterialWithId {
        return dao.update(material)
    }

    override fun findAll(): List<MaterialWithId> {
        return dao.findAll()
    }

    override fun findById(id: String): MaterialWithId? {
        return dao.findById(id)
    }

    override fun delete(id: String) {
        dao.delete(id)
    }
}
