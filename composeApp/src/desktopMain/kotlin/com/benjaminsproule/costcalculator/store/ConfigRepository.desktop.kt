package com.benjaminsproule.costcalculator.store

import androidx.lifecycle.viewmodel.CreationExtras
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.flow

class DesktopConfigRepository : ConfigRepository {
    override val configPreferencesFlow: Flow<Config> = flow { Config() }

    override suspend fun storeConfig(config: Config) {
        TODO("Not yet implemented")
    }
}

private val configRepository = DesktopConfigRepository()

actual fun getConfigRepository(creationExtras: CreationExtras): ConfigRepository = configRepository