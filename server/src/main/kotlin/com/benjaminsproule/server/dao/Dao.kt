package com.benjaminsproule.server.dao

import com.benjaminsproule.server.model.WithId

interface Dao<T, I: WithId<T>> {
    fun save(t: T): I

    fun update(i: I): I

    fun findAll(): List<I>

    fun findById(id: String): I?

    fun delete(id: String)
}
