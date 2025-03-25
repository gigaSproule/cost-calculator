package com.benjaminsproule.server.controller

import com.benjaminsproule.server.model.Material
import com.benjaminsproule.server.model.MaterialWithId
import com.benjaminsproule.server.service.MaterialService
import org.springframework.web.bind.annotation.*

@RestController
@RequestMapping("/material")
class MaterialController(val materialService: MaterialService) : Controller<Material, MaterialWithId> {

    @PostMapping("/")
    override suspend fun save(material: Material): MaterialWithId {
        return materialService.save(material)
    }

    @PutMapping("/")
    override suspend fun update(materialWithId: MaterialWithId): MaterialWithId {
        return materialService.update(materialWithId)
    }

    @GetMapping("/")
    override suspend fun findAll(): List<MaterialWithId> {
        return materialService.findAll()
    }

    @GetMapping("/{id}")
    override suspend fun findById(@PathVariable id: String): MaterialWithId? {
        return materialService.findById(id)
    }

    @DeleteMapping("/{id}")
    override suspend fun delete(@PathVariable id: String) {
        materialService.delete(id)
    }
}
