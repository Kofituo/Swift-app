package com.example.swift_final

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.viewModels
import androidx.compose.material.ScaffoldState
import androidx.compose.material.rememberScaffoldState
import androidx.compose.runtime.rememberCoroutineScope
import androidx.navigation.compose.rememberNavController
import com.example.swift_final.ui.home.DialogViewModel
import com.example.swift_final.ui.home.FrontPage
import com.example.swift_final.ui.theme.Swift_finalTheme
import com.example.swift_final.util.copiedUrl
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.launch

class MainActivity : ComponentActivity() {

    private var scaffoldState: ScaffoldState? = null
    private var scope: CoroutineScope? = null
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            Swift_finalTheme {
                val navController = rememberNavController()
                FrontPage(
                    navController = navController,
                    rememberCoroutineScope().also { scope = it },
                    rememberScaffoldState().also { scaffoldState = it })
            }
        }
    }

    private val dialogViewModel by viewModels<DialogViewModel>()

    override fun onResume() {
        super.onResume()
        //check if enter new address dialog is opened and if the user didn't modify it's contents
        if (dialogViewModel.dialogIsShowing && !dialogViewModel.isModified) {
            // set it with url from clipboard
            copiedUrl?.apply { dialogViewModel.setUrl(this) }
        }
    }

    override fun onBackPressed() {
        scaffoldState?.drawerState?.let {
            if (it.isOpen) {
                scope?.launch { it.close() }
                return // don't do default back press
            }
        }
        super.onBackPressed()
    }
}