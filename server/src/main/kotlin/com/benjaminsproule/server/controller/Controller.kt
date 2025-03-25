package com.benjaminsproule.server.controller

import com.benjaminsproule.server.model.WithId

interface Controller<T, I : WithId<T>> {
    suspend fun save(t: T): I

    suspend fun update(i: I): I

    suspend fun findAll(): List<I>

    suspend fun findById(id: String): I?

    suspend fun delete(id: String)
}
