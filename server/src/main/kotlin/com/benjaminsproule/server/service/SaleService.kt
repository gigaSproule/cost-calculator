package com.benjaminsproule.server.service

import com.benjaminsproule.server.dao.SaleDao
import com.benjaminsproule.server.model.Sale
import com.benjaminsproule.server.model.SaleItem
import com.benjaminsproule.server.model.SaleStatus
import com.benjaminsproule.server.model.SaleWithId

class SaleService(private val dao: SaleDao) : Service<Sale, SaleWithId> {
    override fun create(sale: Sale): SaleWithId {
        return dao.save(sale)
    }

    override fun update(sale: SaleWithId): SaleWithId {
        return dao.update(sale)
    }

    override fun findAll(): List<SaleWithId> {
        return dao.findAll()
    }

    override fun findById(id: String): SaleWithId? {
        return dao.findById(id)
    }

    override fun delete(id: String) {
        dao.delete(id)
    }

    fun addSaleItem(id: String, saleItem: SaleItem) {
        findById(id)?.let {
            update(SaleWithId(it.id, it.saleItems + saleItem, it.status))
        }
    }

    fun completeSale(id: String) {
        findById(id)?.let {
            update(SaleWithId(it.id, it.saleItems, SaleStatus.Completed))
        }
    }

    fun cancelSale(id: String) {
        findById(id)?.let {
            update(SaleWithId(it.id, it.saleItems, SaleStatus.Cancelled))
        }
    }
}
