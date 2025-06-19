package com.benjaminsproule.costcalculator.store

import androidx.lifecycle.viewmodel.CreationExtras
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.flow

class DesktopMaterialsRepository : MaterialsRepository {
    override val materialsPreferencesFlow: Flow<List<StoredMaterial>> = flow { listOf(StoredMaterial()) }

    override suspend fun storeMaterials(materials: List<StoredMaterial>) {
        TODO("Not yet implemented")
    }
}

private val materialsRepository = DesktopMaterialsRepository()

actual fun getMaterialsRepository(creationExtras: CreationExtras): MaterialsRepository = materialsRepository