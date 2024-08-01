package com.benjaminsproule.costcalculator

import android.app.Application
import android.content.Context
import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.Preferences
import androidx.datastore.preferences.preferencesDataStore
import com.benjaminsproule.costcalculator.store.ConfigRepository
import com.benjaminsproule.costcalculator.store.MaterialsRepository


private val Context.configDataStore: DataStore<Preferences> by preferencesDataStore(name = "config")
private val Context.materialsDataStore: DataStore<Preferences> by preferencesDataStore(name = "materials")

class CostCalculatorApplication : Application() {
    lateinit var configRepository: ConfigRepository
    lateinit var materialsRepository: MaterialsRepository
    override fun onCreate() {
        super.onCreate()
        configRepository = ConfigRepository(configDataStore)
        materialsRepository = MaterialsRepository(materialsDataStore)
    }
}
