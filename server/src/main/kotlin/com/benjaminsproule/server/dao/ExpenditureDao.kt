package com.benjaminsproule.server.dao

import com.benjaminsproule.server.model.Expenditure
import com.benjaminsproule.server.model.ExpenditureWithId

class ExpenditureDao : Dao<Expenditure, ExpenditureWithId> {
    override fun save(expenditure: Expenditure): ExpenditureWithId {}

    override fun update(expenditure: ExpenditureWithId): ExpenditureWithId {
    }

    override fun findAll(): List<ExpenditureWithId> {}

    override fun findById(id: String): ExpenditureWithId? {}

    override fun delete(id: String) {}
}
