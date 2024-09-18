package com.benjaminsproule.server.dao

import com.benjaminsproule.server.model.Material
import com.benjaminsproule.server.model.MaterialWithId

class MaterialDao : Dao<Material, MaterialWithId> {
    override fun save(material: Material): MaterialWithId {}

    override fun update(material: MaterialWithId): MaterialWithId {
    }

    override fun findAll(): List<MaterialWithId> {}

    override fun findById(id: String): MaterialWithId? {}

    override fun delete(id: String) {}
}
