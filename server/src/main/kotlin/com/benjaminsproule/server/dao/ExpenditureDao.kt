package com.benjaminsproule.server.dao

import com.benjaminsproule.server.model.Expenditure
import com.benjaminsproule.server.model.ExpenditureWithId

class ExpenditureDao : Dao<Expenditure, ExpenditureWithId> {
    override fun save(expenditure: Expenditure): ExpenditureWithId {
        return TODO("Provide the return value")
    }

    override fun update(expenditure: ExpenditureWithId): ExpenditureWithId {
        return TODO("Provide the return value")
    }

    override fun findAll(): List<ExpenditureWithId> {
        return TODO("Provide the return value")
    }

    override fun findById(id: String): ExpenditureWithId? {
        return TODO("Provide the return value")
    }

    override fun delete(id: String) {}
}
