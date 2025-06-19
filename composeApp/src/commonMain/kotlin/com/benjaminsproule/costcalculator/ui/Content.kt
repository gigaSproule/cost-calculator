@file:OptIn(ExperimentalMaterial3Api::class)

package com.benjaminsproule.costcalculator.ui;

import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Menu
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import androidx.navigation.NavController
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.launch
import kotlinx.serialization.Serializable


interface Route {
    val label: String
}

@Serializable
object Etsy : Route {
    override val label: String = "Etsy"
}

@Serializable
object Paypal : Route {
    override val label: String = "Paypal"
}

@Serializable
object Shopify : Route {
    override val label: String = "Shopify"
}

@Serializable
object Stripe : Route {
    override val label: String = "Stripe"
}

@Serializable
object SumUp : Route {
    override val label: String = "SumUp"
}

@Serializable
object Materials : Route {
    override val label: String = "Materials"
}

@Serializable
object Options : Route {
    override val label: String = "Options"
}

@Composable
fun Content() {
    val services = listOf(
        Etsy,
        Paypal,
        Shopify,
        Stripe,
        SumUp
    )
    val selectedItem = remember { mutableStateOf(services[0]) }

    val drawerState = rememberDrawerState(initialValue = DrawerValue.Closed)
    val scope = rememberCoroutineScope()
    val navController = rememberNavController()

    Surface(modifier = Modifier.fillMaxSize(), color = MaterialTheme.colorScheme.background) {
        ModalNavigationDrawer(
            drawerState = drawerState,
            drawerContent = {
                ModalDrawerSheet {
                    NavigationDrawerContent(services, selectedItem, navController, scope, drawerState)
                }
            }
        ) {
            Scaffold(
                topBar = {
                    TopBarContent(selectedItem, scope, drawerState)
                }
            ) { innerPadding ->
                Row(
                    modifier = Modifier.padding(innerPadding),
                ) {
                    NavHost(navController = navController, startDestination = Etsy) {
                        composable<Etsy> { EtsyUi() }
                        composable<Paypal> { PaypalUi() }
                        composable<Shopify> { ShopifyUi() }
                        composable<Stripe> { StripeUi() }
                        composable<SumUp> { SumUpUi() }
                        composable<Materials> { MaterialsUi() }
                        composable<Options> { OptionsUi() }
                    }
                }
            }
        }
    }
}

@Composable
private fun TopBarContent(selectedItem: MutableState<Route>, scope: CoroutineScope, drawerState: DrawerState) {
    CenterAlignedTopAppBar(
        title = {
            Text(selectedItem.value.label)
        },
        navigationIcon = {
            IconButton(onClick = {
                scope.launch { drawerState.open() }
            }) {
                Icon(
                    imageVector = Icons.Filled.Menu,
                    contentDescription = "Localized description"
                )
            }
        },
    )
}

@Composable
private fun NavigationDrawerContent(
    services: List<Route>,
    selectedService: MutableState<Route>,
    navController: NavController,
    scope: CoroutineScope,
    drawerState: DrawerState
) {
    Text("Services", modifier = Modifier.padding(16.dp))
    HorizontalDivider()
    services.forEach { service ->
        NavigationDrawerItem(
            label = { Text(text = service.label) },
            selected = service == selectedService.value,
            onClick = {
                scope.launch { drawerState.close() }
                selectedService.value = service
                navController.navigate(service)
            }
        )
    }
    HorizontalDivider()
    NavigationDrawerItem(
        label = { Text(text = Materials.label) },
        selected = Materials == selectedService.value,
        onClick = {
            scope.launch { drawerState.close() }
            selectedService.value = Materials
            navController.navigate(Materials)
        }
    )
    NavigationDrawerItem(
        label = { Text(text = Options.label) },
        selected = Options == selectedService.value,
        onClick = {
            scope.launch { drawerState.close() }
            selectedService.value = Options
            navController.navigate(Options)
        }
    )
}
