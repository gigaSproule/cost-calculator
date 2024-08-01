package com.benjaminsproule.costcalculator.ui

import androidx.lifecycle.ViewModel
import androidx.lifecycle.ViewModelProvider
import androidx.lifecycle.ViewModelProvider.AndroidViewModelFactory.Companion.APPLICATION_KEY
import androidx.lifecycle.viewModelScope
import androidx.lifecycle.viewmodel.initializer
import androidx.lifecycle.viewmodel.viewModelFactory
import com.benjaminsproule.costcalculator.CostCalculatorApplication
import com.benjaminsproule.costcalculator.store.MaterialsRepository
import com.benjaminsproule.costcalculator.store.StoredMaterial
import kotlinx.coroutines.flow.SharingStarted
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.stateIn
import kotlinx.coroutines.launch

class MaterialsViewModel(private val materialsRepository: MaterialsRepository) : ViewModel() {
    companion object {
        val Factory: ViewModelProvider.Factory = viewModelFactory {
            initializer {
                val application = (this[APPLICATION_KEY] as CostCalculatorApplication)
                MaterialsViewModel(application.materialsRepository)
            }
        }
    }

    val materials: StateFlow<List<StoredMaterial>> =
        materialsRepository.materialsPreferencesFlow.stateIn(
            scope = viewModelScope,
            started = SharingStarted.WhileSubscribed(5000),
            initialValue = emptyList()
        )

    fun storeMaterials(editableMaterials: List<StoredMaterial>) {
        viewModelScope.launch {
            materialsRepository.storeMaterials(editableMaterials)
        }
    }
}
