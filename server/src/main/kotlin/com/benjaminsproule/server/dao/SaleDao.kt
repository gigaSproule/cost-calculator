package com.benjaminsproule.server.dao

import com.benjaminsproule.server.model.Sale
import com.benjaminsproule.server.model.SaleWithId

class SaleDao : Dao<Sale, SaleWithId> {
    override fun save(sale: Sale): SaleWithId {
        return TODO("Provide the return value")
    }

    override fun update(sale: SaleWithId): SaleWithId {
        return TODO("Provide the return value")
    }

    override fun findAll(): List<SaleWithId> {
        return TODO("Provide the return value")
    }

    override fun findById(id: String): SaleWithId? {
        return TODO("Provide the return value")
    }

    override fun delete(id: String) {}
}
