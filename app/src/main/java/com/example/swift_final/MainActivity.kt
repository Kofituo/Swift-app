package com.example.swift_final

import android.os.Bundle
import android.util.Log
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.viewModels
import com.example.swift_final.ui.home.DialogViewModel
import com.example.swift_final.ui.home.FrontPage
import com.example.swift_final.ui.theme.Swift_finalTheme
import com.example.swift_final.util.copiedUrl

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            Swift_finalTheme {
                FrontPage()
            }
        }
    }

    private val dialogViewModel by viewModels<DialogViewModel>()

    override fun onResume() {
        super.onResume()
        //check if enter new address dialog is opened and if the user didn't modify it's contents
        Log.e("res","show ${dialogViewModel.dialogIsShowing}  mod ${dialogViewModel.isModified}")
        if (dialogViewModel.dialogIsShowing && !dialogViewModel.isModified) {
            // set it with url from clipboard
            copiedUrl?.apply { dialogViewModel.setUrl(this) }
        }
    }
}