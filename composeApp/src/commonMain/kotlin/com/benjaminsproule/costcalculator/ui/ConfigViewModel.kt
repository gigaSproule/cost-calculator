package com.benjaminsproule.costcalculator.ui

import androidx.lifecycle.ViewModel
import androidx.lifecycle.ViewModelProvider
import androidx.lifecycle.viewModelScope
import androidx.lifecycle.viewmodel.initializer
import androidx.lifecycle.viewmodel.viewModelFactory
import com.benjaminsproule.costcalculator.store.Config
import com.benjaminsproule.costcalculator.store.ConfigRepository
import com.benjaminsproule.costcalculator.store.getConfigRepository
import kotlinx.coroutines.flow.SharingStarted
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.stateIn
import kotlinx.coroutines.launch

class ConfigViewModel(private val configRepository: ConfigRepository) : ViewModel() {
    companion object {
        val Factory: ViewModelProvider.Factory = viewModelFactory {
            initializer {
                ConfigViewModel(getConfigRepository(this))
            }
        }
    }

    val config: StateFlow<Config> =
        configRepository.configPreferencesFlow.stateIn(
            scope = viewModelScope,
            started = SharingStarted.WhileSubscribed(5000),
            initialValue = Config()
        )

    fun storeConfig(config: Config) {
        viewModelScope.launch {
            configRepository.storeConfig(config)
        }
    }

    fun resetToDefaults() {
        storeConfig(Config())
    }
}
