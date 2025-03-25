package com.benjaminsproule.server.service

import com.benjaminsproule.server.dao.ExpenditureDao
import com.benjaminsproule.server.dao.MaterialStockDao
import com.benjaminsproule.server.model.Expenditure
import com.benjaminsproule.server.model.ExpenditureWithId
import com.benjaminsproule.server.model.MaterialStockWithId

class ExpenditureService(private val dao: ExpenditureDao, private val materialStockDao: MaterialStockDao) :
    Service<Expenditure, ExpenditureWithId> {
    override suspend fun save(expenditure: Expenditure): ExpenditureWithId {
        materialStockDao.findByMaterialId(expenditure.material.id)?.let { materialStock ->
            materialStockDao.update(
                MaterialStockWithId(
                    materialStock.id,
                    materialStock.material,
                    materialStock.quantity + expenditure.quantity
                )
            )
        }
        return dao.save(expenditure)
    }

    override suspend fun update(expenditure: ExpenditureWithId): ExpenditureWithId {
        return dao.update(expenditure)
    }

    override suspend fun findAll(): List<ExpenditureWithId> {
        return dao.findAll()
    }

    override suspend fun findById(id: String): ExpenditureWithId? {
        return dao.findById(id)
    }

    override suspend fun delete(id: String) {
        dao.delete(id)
    }
}
