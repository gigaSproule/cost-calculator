package com.benjaminsproule.server.model

data class Expenditure(val material: MaterialWithId, val cost: Double, val quantity: Int)

data class ExpenditureWithId(
    override val id: String,
    val material: MaterialWithId,
    val cost: Double,
    val quantity: Int
) :
    WithId<Expenditure>()
