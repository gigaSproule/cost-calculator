package com.benjaminsproule.costcalculator.store

import android.util.Log
import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.Preferences
import androidx.datastore.preferences.core.edit
import androidx.datastore.preferences.core.emptyPreferences
import androidx.datastore.preferences.core.stringPreferencesKey
import androidx.lifecycle.ViewModelProvider.AndroidViewModelFactory.Companion.APPLICATION_KEY
import androidx.lifecycle.viewmodel.CreationExtras
import com.benjaminsproule.costcalculator.CostCalculatorApplication
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.catch
import kotlinx.coroutines.flow.map
import kotlinx.serialization.json.Json
import java.io.IOException

class AndroidMaterialsRepository(private val dataStore: DataStore<Preferences>) :
    MaterialsRepository {
    private object PreferencesKeys {
        val MATERIALS = stringPreferencesKey("materials")
    }

    override val materialsPreferencesFlow: Flow<List<StoredMaterial>> = dataStore.data
        .catch { exception ->
            // dataStore.data throws an IOException when an error is encountered when reading data
            if (exception is IOException) {
                Log.e("ConfigRepository", "Error reading preferences.", exception)
                emit(emptyPreferences())
            } else {
                throw exception
            }
        }.map { preferences ->
            preferences[PreferencesKeys.MATERIALS]
                ?.let { Json.decodeFromString<List<StoredMaterial>>(it) }
                .orEmpty()
        }

    override suspend fun storeMaterials(materials: List<StoredMaterial>) {
        dataStore.edit { preferences ->
            preferences[PreferencesKeys.MATERIALS] = Json.encodeToString(materials.map {
                it.value = it.pricePerPack / it.quantityPerPack
                it
            })
        }
    }

}

actual fun getMaterialsRepository(creationExtras: CreationExtras): MaterialsRepository {
    val application = (creationExtras[APPLICATION_KEY] as CostCalculatorApplication)
    return application.materialsRepository
}