package com.benjaminsproule.costcalculator.store

import androidx.lifecycle.viewmodel.CreationExtras
import com.akuleshov7.ktoml.Toml
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.flow
import kotlinx.coroutines.flow.flowOn
import kotlinx.coroutines.flow.map
import kotlinx.serialization.decodeFromString
import kotlinx.serialization.encodeToString
import java.nio.file.Files
import java.nio.file.Path
import java.nio.file.StandardOpenOption
import kotlin.io.path.createFile
import kotlin.io.path.createParentDirectories
import kotlin.io.path.exists

class DesktopConfigRepository : ConfigRepository {
    val configFile: Path = Path.of(getStorePath(), "calculator_config.toml")

    override val configPreferencesFlow: Flow<Config> =
        flow {
            if (!configFile.exists()) {
                configFile.createParentDirectories()
                configFile.createFile()
            }
            emit(Files.readString(configFile))
        }
            .flowOn(Dispatchers.IO)
            .map { fileContents -> Toml.decodeFromString(fileContents) }

    override suspend fun storeConfig(config: Config) {
        Files.writeString(configFile, Toml.encodeToString(config), StandardOpenOption.TRUNCATE_EXISTING)
    }

}

private val configRepository = DesktopConfigRepository()

actual fun getConfigRepository(creationExtras: CreationExtras): ConfigRepository = configRepository