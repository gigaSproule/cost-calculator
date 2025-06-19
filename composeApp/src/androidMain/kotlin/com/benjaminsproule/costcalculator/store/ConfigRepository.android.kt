package com.benjaminsproule.costcalculator.store

import android.util.Log
import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.*
import androidx.lifecycle.ViewModelProvider.AndroidViewModelFactory.Companion.APPLICATION_KEY
import androidx.lifecycle.viewmodel.CreationExtras
import com.benjaminsproule.costcalculator.CostCalculatorApplication
import com.benjaminsproule.costcalculator.ui.ConfigViewModel
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.catch
import kotlinx.coroutines.flow.map
import java.io.IOException

class AndroidConfigRepository(private val dataStore: DataStore<Preferences>) : ConfigRepository {
    private object PreferencesKeys {
        val MARKUP_PERCENTAGE = floatPreferencesKey("markupPercentage")
        val HOURLY_RATE = floatPreferencesKey("hourlyRate")
        val TAX_RATE = floatPreferencesKey("taxRate")
        val VAT = floatPreferencesKey("vat")
        val CURRENCY = stringPreferencesKey("currency")
    }

    override val configPreferencesFlow: Flow<Config> = dataStore.data
        .catch { exception ->
            // dataStore.data throws an IOException when an error is encountered when reading data
            if (exception is IOException) {
                Log.e("ConfigRepository", "Error reading preferences.", exception)
                emit(emptyPreferences())
            } else {
                throw exception
            }
        }.map { preferences ->
            Config(
                markupPercentage = preferences[PreferencesKeys.MARKUP_PERCENTAGE]
                    ?: ConfigDefaults.MARKUP_PERCENTAGE,
                hourlyRate = preferences[PreferencesKeys.HOURLY_RATE]
                    ?: ConfigDefaults.MINIMUM_WAGE,
                taxRate = preferences[PreferencesKeys.TAX_RATE] ?: ConfigDefaults.BASIC_RATE,
                vat = preferences[PreferencesKeys.VAT] ?: ConfigDefaults.VAT,
                currency = preferences[PreferencesKeys.CURRENCY] ?: ConfigDefaults.CURRENCY,
            )
        }

    override suspend fun storeConfig(config: Config) {
        dataStore.edit { preferences ->
            preferences[PreferencesKeys.MARKUP_PERCENTAGE] = config.markupPercentage
            preferences[PreferencesKeys.HOURLY_RATE] = config.hourlyRate
            preferences[PreferencesKeys.TAX_RATE] = config.taxRate
            preferences[PreferencesKeys.VAT] = config.vat
            preferences[PreferencesKeys.CURRENCY] = config.currency
        }
    }
}

actual fun getConfigRepository(creationExtras: CreationExtras): ConfigRepository {
    val application = (creationExtras[APPLICATION_KEY] as CostCalculatorApplication)
    return application.configRepository
}