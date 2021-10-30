package com.example.swift_final.util

import android.content.ClipDescription
import android.content.Context
import android.content.Intent
import android.content.SharedPreferences
import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.width
import androidx.compose.material.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import androidx.core.content.getSystemService
import androidx.navigation.NavController
import com.example.swift_final.ApplicationLoader
import com.example.swift_final.ApplicationLoader.Companion.APP_NAME

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

fun NavController.navigateSingleTop(route: String) = navigate(route = route) {
    launchSingleTop = true
}

inline fun <reified T> SharedPreferences.get(key: String, block: T.() -> Unit = {}): T =
    when (T::class) {
        Boolean::class -> (getBoolean(key, false) as T).apply(block)
        String::class -> (getString(key, null) as T).apply(block)
        Int::class -> (getInt(key, -1) as T).apply(block)
        Long::class -> (getLong(key, -1) as T).apply(block)
        Float::class -> (getFloat(key, -1f) as T).apply(block)
        Set::class -> (getStringSet(key, null) as T).apply(block)
        else -> TODO()
    }

@Composable
fun sharedPreferences(key: String) =
    LocalContext.current.getSharedPreferences(key, Context.MODE_PRIVATE)

fun Context.sharedPreferences(key: String) =
    getSharedPreferences(key, Context.MODE_PRIVATE)

object SharedPreferencesConstants {
    /*** Identifier to receive the list of unfinished downloads ***/
    const val UnfinishedDownloads = "$APP_NAME.unfinished download"
    const val HasDownloads = "has downloads"
}

/* Intent builder*/
/**
 * Creates a new [Intent] and applies [block] to it
 */
inline fun <reified T> Context.buildIntent(block: Intent.() -> Unit) =
    Intent(this, T::class.java).apply(block)
