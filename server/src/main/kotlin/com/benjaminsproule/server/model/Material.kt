package com.benjaminsproule.server.model

data class Material(val name: String)

data class MaterialWithId(override val id: String, val name: String) : WithId<Material>()
