package com.example.swift_final.ui.home

import androidx.compose.foundation.layout.size
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Menu
import androidx.compose.runtime.Composable
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontStyle
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.example.swift_final.R
import kotlinx.coroutines.launch

@Composable
fun FrontPage() {
    val scaffoldState = rememberScaffoldState()
    val scope = rememberCoroutineScope()
    Scaffold(
        scaffoldState = scaffoldState,
        drawerContent = { Text("Drawer content") },
        drawerShape = Drawer.drawerShape(),
        topBar = {
            TopAppBar(
                title = {
                    Text(
                        "Swift",
                        fontSize = 36.sp,
                        fontFamily = FontFamily.Serif,
                        fontStyle = FontStyle.Italic
                    )
                },
                navigationIcon = {
                    IconButton(
                        onClick = {
                            scope.launch { scaffoldState.drawerState.open() }
                        }
                    ) {
                        Icon(
                            Icons.Filled.Menu,
                            contentDescription = stringResource(id = R.string.menu),
                            Modifier.size(28.dp)
                        )
                    }
                }
            )
        },
        content = { innerPadding ->
           FrontPageContent(innerPadding)
        },
        //backgroundColor = Color.Gray
    )
}
