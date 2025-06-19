package com.benjaminsproule.costcalculator.store

import androidx.lifecycle.viewmodel.CreationExtras
import kotlinx.coroutines.flow.Flow

interface ConfigRepository {
    val configPreferencesFlow: Flow<Config>

    suspend fun storeConfig(config: Config)
}

expect fun getConfigRepository(creationExtras: CreationExtras): ConfigRepository
