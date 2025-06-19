package com.benjaminsproule.costcalculator.store

import androidx.lifecycle.viewmodel.CreationExtras
import com.akuleshov7.ktoml.Toml
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.catch
import kotlinx.coroutines.flow.flow
import kotlinx.coroutines.flow.flowOn
import kotlinx.coroutines.flow.map
import kotlinx.serialization.Serializable
import kotlinx.serialization.decodeFromString
import kotlinx.serialization.encodeToString
import java.nio.file.Files
import java.nio.file.Path
import java.nio.file.StandardOpenOption
import kotlin.io.path.createFile
import kotlin.io.path.createParentDirectories
import kotlin.io.path.exists

@Serializable
data class StoredMaterials(val storedMaterials: List<StoredMaterial>) {
}

class DesktopMaterialsRepository : MaterialsRepository {
    val storedMaterialsFile: Path = Path.of(getStorePath(), "materials.toml")

    override val materialsPreferencesFlow: Flow<List<StoredMaterial>> = flow {
        if (!storedMaterialsFile.exists()) {
            storedMaterialsFile.createParentDirectories()
            storedMaterialsFile.createFile()
        }
        emit(Files.readString(storedMaterialsFile))
    }
        .flowOn(Dispatchers.IO)
        .map { fileContents -> Toml.decodeFromString<StoredMaterials>(fileContents) }
        .catch { cause -> StoredMaterials(emptyList()) }
        .map { storedMaterials -> storedMaterials.storedMaterials }

    override suspend fun storeMaterials(materials: List<StoredMaterial>) {
        Files.writeString(
            storedMaterialsFile,
            Toml.encodeToString(StoredMaterials(materials)),
            StandardOpenOption.TRUNCATE_EXISTING
        )
    }
}

private val materialsRepository = DesktopMaterialsRepository()

actual fun getMaterialsRepository(creationExtras: CreationExtras): MaterialsRepository = materialsRepository