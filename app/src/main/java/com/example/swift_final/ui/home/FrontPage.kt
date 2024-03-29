package com.example.swift_final.ui.home

import androidx.compose.material.Scaffold
import androidx.compose.material.ScaffoldState
import androidx.compose.runtime.Composable
import androidx.navigation.NavHostController
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.navArgument
import com.example.swift_final.Screens
import com.example.swift_final.ui.unfinished_downloads.UnfinishedPage
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.launch

@Composable
fun FrontPage(
    navController: NavHostController,
    scope: CoroutineScope,
    scaffoldState: ScaffoldState
) {
    Scaffold(
        scaffoldState = scaffoldState,
        drawerContent = {
            Drawer.DrawerContent(
                navController
            ) { scope.launch { scaffoldState.drawerState.close() } }
        },
        drawerShape = Drawer.drawerShape(),
        topBar = {},
        content = { innerPadding ->
            NavHost(navController = navController, startDestination = Screens.FrontPage.name) {
                val openDrawer = {
                    scope.launch { scaffoldState.drawerState.open() }
                    Unit // really??
                }
                composable(Screens.FrontPage.name) {
                    FrontPageContent(
                        navController,
                        openDrawer = openDrawer,
                        innerPadding
                    )
                }
                composable("${Screens.UnfinishedDownloads.name}?filename={filename}&category={category}&filesize={filesize}",
                    arguments = listOf(
                        navArgument("filename") { nullable = true },
                        navArgument("category") { nullable = true },
                        navArgument("filesize") { defaultValue = 0L }
                    )) {
                    UnfinishedPage(
                        navController = navController,
                        it.arguments?.getString("filename"),
                        it.arguments?.getString("category"),
                        it.arguments?.getLong("filesize") ?: 0
                    )
                }
            }
        },
    )
}
