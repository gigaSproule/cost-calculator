package com.benjaminsproule.server.service

import com.benjaminsproule.server.model.WithId

interface Service<T, I : WithId<T>> {
    fun create(t: T): I

    fun update(i: I): I

    fun findAll(): List<I>

    fun findById(id: String): I?

    fun delete(id: String)
}
