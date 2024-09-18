package com.benjaminsproule.server.dao

import com.benjaminsproule.server.model.Sale
import com.benjaminsproule.server.model.SaleWithId

class SaleDao : Dao<Sale, SaleWithId> {
    override fun save(sale: Sale): SaleWithId {}

    override fun update(sale: SaleWithId): SaleWithId {
    }

    override fun findAll(): List<SaleWithId> {}

    override fun findById(id: String): SaleWithId? {}

    override fun delete(id: String) {}
}
