package com.benjaminsproule.costcalculator.store

import android.util.Log
import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.*
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.catch
import kotlinx.coroutines.flow.map
import java.io.IOException
import com.benjaminsproule.costcalculator.calculator.Config as CalculatorConfig

data class Config(
    override val markupPercentage: Float = ConfigDefaults.MARKUP_PERCENTAGE,
    override val hourlyRate: Float = ConfigDefaults.MINIMUM_WAGE,
    override val taxRate: Float = ConfigDefaults.BASIC_RATE,
    override val vat: Float = ConfigDefaults.VAT,
    val currency: String = ConfigDefaults.CURRENCY,
) : CalculatorConfig()

private object ConfigDefaults {
    val MARKUP_PERCENTAGE = 0.0f
    val MINIMUM_WAGE = 10.42f
    val BASIC_RATE = 20.0f
    val VAT = 20.0f
    val CURRENCY = "£"
}

class ConfigRepository(private val dataStore: DataStore<Preferences>) {
    private object PreferencesKeys {
        val MARKUP_PERCENTAGE = floatPreferencesKey("markupPercentage")
        val HOURLY_RATE = floatPreferencesKey("hourlyRate")
        val TAX_RATE = floatPreferencesKey("taxRate")
        val VAT = floatPreferencesKey("vat")
        val CURRENCY = stringPreferencesKey("currency")
    }

    val configPreferencesFlow: Flow<Config> = dataStore.data
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
                markupPercentage = preferences[PreferencesKeys.MARKUP_PERCENTAGE] ?: ConfigDefaults.MARKUP_PERCENTAGE,
                hourlyRate = preferences[PreferencesKeys.HOURLY_RATE] ?: ConfigDefaults.MINIMUM_WAGE,
                taxRate = preferences[PreferencesKeys.TAX_RATE] ?: ConfigDefaults.BASIC_RATE,
                vat = preferences[PreferencesKeys.VAT] ?: ConfigDefaults.VAT,
                currency = preferences[PreferencesKeys.CURRENCY] ?: ConfigDefaults.CURRENCY,
            )
        }

    suspend fun storeConfig(config: Config) {
        dataStore.edit { preferences ->
            preferences[PreferencesKeys.MARKUP_PERCENTAGE] = config.markupPercentage
            preferences[PreferencesKeys.HOURLY_RATE] = config.hourlyRate
            preferences[PreferencesKeys.TAX_RATE] = config.taxRate
            preferences[PreferencesKeys.VAT] = config.vat
            preferences[PreferencesKeys.CURRENCY] = config.currency
        }
    }
}
