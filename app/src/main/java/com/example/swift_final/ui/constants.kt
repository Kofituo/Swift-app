package com.example.swift_final.ui

import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.ui.ExperimentalComposeUiApi
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.compose.ui.window.DialogProperties

const val DialogRadius = 18
const val OkButtonPadding = 2
const val OutlineWidth = 1
const val TextFieldRadius = 10
const val FrontPageButtonRadius = 10
const val FrontPageExtraButtonsSize = 85
const val AddUrlImageSize = 60
const val FrontPageExtraImageSize = 35
inline val textFieldShape get() = RoundedCornerShape(TextFieldRadius.dp)
@OptIn(ExperimentalComposeUiApi::class)
inline val dialogProperties get() = DialogProperties(usePlatformDefaultWidth = false)
const val DialogWidth = 90f
const val DrawerRadius = 30f
const val DrawerButtonPadding = 13
val defaultFontSize = 14.sp