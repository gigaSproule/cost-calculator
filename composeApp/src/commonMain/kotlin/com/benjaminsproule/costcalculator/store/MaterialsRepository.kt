package com.benjaminsproule.costcalculator.store

import androidx.lifecycle.viewmodel.CreationExtras
import kotlinx.coroutines.flow.Flow

interface MaterialsRepository {
    val materialsPreferencesFlow: Flow<List<StoredMaterial>>

    suspend fun storeMaterials(materials: List<StoredMaterial>)
}

expect fun getMaterialsRepository(creationExtras: CreationExtras): MaterialsRepository
