package com.benjaminsproule.costcalculator.store

import kotlinx.serialization.Serializable

@Serializable
data class StoredMaterial(
    // TODO: Make this multiplatform
//    val id: String = UUID.randomUUID().toString(),
    val id: String = "UUID.randomUUID().toString()",
    var name: String = StoredMaterialDefaults.NAME,
    var quantityPerPack: Float = StoredMaterialDefaults.QUANTITY_PER_PACK,
    var pricePerPack: Float = StoredMaterialDefaults.PRICE_PER_PACK,
    var value: Float = StoredMaterialDefaults.VALUE,
)

private object StoredMaterialDefaults {
    val NAME = ""
    val QUANTITY_PER_PACK = 0.0f
    val PRICE_PER_PACK = 0.0f
    val VALUE = 0.0f
}