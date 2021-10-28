package com.example.swift_final.util

import android.content.res.Configuration
import android.util.Log
import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.runtime.Composable
import androidx.compose.runtime.currentComposer
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.unit.dp
import com.example.swift_final.ApplicationLoader
import kotlin.math.sqrt


object DisplayUtils {
    private var _isTablet: Boolean? = null

    //checks whether the diagonal is >= 7 inches
    val isTablet
        get() = _isTablet ?: run {
            val yInches = pixelsHeight / metrics.ydpi
            val xInches = pixelsWidth / metrics.xdpi
            val diagonalInches = sqrt((xInches * xInches + yInches * yInches).toDouble())
            Log.e("is rtablet", " here $diagonalInches ${metrics.ydpi} h ${metrics.xdpi}")
            val result = diagonalInches >= 7
            _isTablet = result
            result
        }

    private val metrics get() = ApplicationLoader.applicationContext.resources.displayMetrics

    private val pixelsWidth get() = metrics.widthPixels
    private val pixelsHeight get() = metrics.heightPixels

    fun percentageOfScreenSize(
        percent: Float,
        orientationDependent: Boolean = false
    ): ScreenPixels {
        val value = percent / 100
        val independentWidth by lazy { ScreenPixels.screenSize(pixelsHeight, pixelsWidth) }
        val width: Float =
            if (orientationDependent) pixelsWidth.toFloat() else independentWidth.width
        val height = if (orientationDependent) pixelsHeight.toFloat() else independentWidth.height
        return ScreenPixels(value * width, value * height)
    }

    data class ScreenPixels(val width: Float, val height: Float) {
        companion object {
            fun screenSize(first: Int, second: Int) = ScreenPixels(
                if (first > second) second.toFloat() else first.toFloat(),
                if (first > second) first.toFloat() else second.toFloat()
            )

            val ScreenPixels.widthInDp get() = (width / metrics.density).dp
            val ScreenPixels.heightInDp get() = (height / metrics.density).dp
        }
    }

    val isLandscape
        @Composable get() =
            LocalContext.current.resources.configuration.orientation == Configuration.ORIENTATION_LANDSCAPE

    /*@Composable
    fun <T> useOnUiMode(light: T, dark: T) = if (isSystemInDarkTheme()) dark else light*/
}