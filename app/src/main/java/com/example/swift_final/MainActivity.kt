package com.example.swift_final

import android.os.Bundle
import android.util.Log
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.viewModels
import com.example.swift_final.lib.DownloadInfo
import com.example.swift_final.ui.home.DialogViewModel
import com.example.swift_final.ui.home.FrontPage
import com.example.swift_final.ui.home.setAddUrlTextField
import com.example.swift_final.ui.theme.Swift_finalTheme

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            Swift_finalTheme {
                FrontPage()
            }
        }
        DownloadInfo("here",null)
    }

    private val dialogViewModel by viewModels<DialogViewModel>()

    override fun onResume() {
        super.onResume()
        if (dialogViewModel.dialogShowing.value!!) {
            Log.e("value", "${dialogViewModel.initialText.value}  ${setAddUrlTextField()}")
            val text = setAddUrlTextField()
            //Happens on configuration change
            if (!dialogViewModel.autoModifyInSection || text.annotatedString.isBlank()) return
            dialogViewModel.setInitialText(text)
        }
    }
}