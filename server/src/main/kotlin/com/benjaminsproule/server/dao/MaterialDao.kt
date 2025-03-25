package com.benjaminsproule.server.dao

import com.benjaminsproule.server.model.Material
import com.benjaminsproule.server.model.MaterialWithId
import com.mongodb.reactivestreams.client.MongoClient
import org.springframework.stereotype.Repository
import kotlin.uuid.ExperimentalUuidApi
import kotlin.uuid.Uuid

@Repository
class MaterialDao : Dao<Material, MaterialWithId> {
    val mongoClient: MongoClient = TODO("initialize me")

    @OptIn(ExperimentalUuidApi::class)
    override fun save(material: Material): MaterialWithId {
        val materialWithId = MaterialWithId(Uuid.random().toString(), material.name)
        mongoClient.getDatabase("x").getCollection("material").insertOne(material)
        return materialWithId
    }

    override fun update(material: MaterialWithId): MaterialWithId {
        return TODO("Provide the return value")
    }

    override fun findAll(): List<MaterialWithId> {
        return TODO("Provide the return value")
    }

    override fun findById(id: String): MaterialWithId? {
        return MaterialWithId("id", "name")
    }

    override fun delete(id: String) {}
}
