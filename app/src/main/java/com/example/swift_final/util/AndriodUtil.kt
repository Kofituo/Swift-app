package com.example.swift_final.util

import android.content.ClipDescription
import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.width
import androidx.compose.material.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import androidx.core.content.getSystemService
import com.example.swift_final.ApplicationLoader

/**
 * Adds vertical [space] in [dp]
 * */
@Composable
fun VerticalSpacer(space: Int) = Spacer(modifier = Modifier.height(space.dp))


/**
 * Adds horizontal [space] in [dp]
 * */
@Composable
fun HorizontalSpacer(space: Int) = Spacer(modifier = Modifier.width(space.dp))

@Composable
fun textFieldBorder(width: Dp) = BorderStroke(width = width, MaterialTheme.colors.onSurface)

val copiedText: String?
    get() {
        val clipboardManager =
            ApplicationLoader.applicationContext.getSystemService<android.content.ClipboardManager>()
        return clipboardManager?.run {
            val containsText =
                primaryClipDescription?.hasMimeType(ClipDescription.MIMETYPE_TEXT_PLAIN)
                    ?: false
            if (containsText) primaryClip?.getItemAt(0)?.text?.toString() else null
        }
    }

inline val copiedUrl get() = copiedText?.let { if (it.isUrl) it else null }