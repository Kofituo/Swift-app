package com.example.swift_final.ui.home

import android.util.Log
import androidx.compose.ui.geometry.CornerRadius
import androidx.compose.ui.geometry.Rect
import androidx.compose.ui.geometry.RoundRect
import androidx.compose.ui.geometry.Size
import androidx.compose.ui.graphics.Outline
import androidx.compose.ui.graphics.Shape
import androidx.compose.ui.unit.Density
import androidx.compose.ui.unit.LayoutDirection
import androidx.compose.ui.unit.dp
import com.example.swift_final.util.DisplayUtils
import com.example.swift_final.util.DisplayUtils.isTablet

object Drawer {
    fun drawerShape() =
        object : Shape {
            override fun createOutline(
                size: Size,
                layoutDirection: LayoutDirection,
                density: Density
            ): Outline {
                val pixels =  DisplayUtils.percentageOfScreenSize(if (isTablet) 45f else 78f)
                val boundaries = Rect(0f, 0f, pixels.width, size.height)
                val radius = 24.dp.value
                //Log.e("pixles $radius", "$size $pixels ${density.fontScale}")
                val isLtr = layoutDirection == LayoutDirection.Ltr
                val radiusObject = CornerRadius(radius, radius)
                return Outline.Rounded(
                    RoundRect(
                        boundaries,
                        topLeft = if (isLtr) CornerRadius.Zero else radiusObject,
                        topRight = if (isLtr) radiusObject else CornerRadius.Zero,
                        bottomLeft = if (isLtr) CornerRadius.Zero else radiusObject,
                        bottomRight = if (isLtr) radiusObject else CornerRadius.Zero
                    )
                )
            }
        }
}