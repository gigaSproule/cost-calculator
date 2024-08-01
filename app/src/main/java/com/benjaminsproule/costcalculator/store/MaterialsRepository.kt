package com.benjaminsproule.costcalculator.store

import android.util.Log
import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.Preferences
import androidx.datastore.preferences.core.edit
import androidx.datastore.preferences.core.emptyPreferences
import androidx.datastore.preferences.core.stringPreferencesKey
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.catch
import kotlinx.coroutines.flow.map
import kotlinx.serialization.Serializable
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import java.io.IOException
import java.util.*

@Serializable
data class StoredMaterial(
    val id: String = UUID.randomUUID().toString(),
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

class MaterialsRepository(private val dataStore: DataStore<Preferences>) {
    private object PreferencesKeys {
        val MATERIALS = stringPreferencesKey("materials")
    }

    var materialsPreferencesFlow: Flow<List<StoredMaterial>> = dataStore.data
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

    suspend fun storeMaterials(materials: List<StoredMaterial>) {
        dataStore.edit { preferences ->
            preferences[PreferencesKeys.MATERIALS] = Json.encodeToString(materials.map {
                it.value = it.pricePerPack / it.quantityPerPack
                it
            })
        }
    }

}
