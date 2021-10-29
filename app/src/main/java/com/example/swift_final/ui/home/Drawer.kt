package com.example.swift_final.ui.home

import android.util.Log
import androidx.compose.foundation.*
import androidx.compose.foundation.layout.*
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.CornerRadius
import androidx.compose.ui.geometry.Rect
import androidx.compose.ui.geometry.RoundRect
import androidx.compose.ui.geometry.Size
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.Outline
import androidx.compose.ui.graphics.Shape
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.Density
import androidx.compose.ui.unit.LayoutDirection
import androidx.compose.ui.unit.dp
import androidx.constraintlayout.compose.ConstraintLayout
import androidx.navigation.NavController
import com.example.swift_final.R
import com.example.swift_final.Screens
import com.example.swift_final.ui.DrawerButtonPadding
import com.example.swift_final.ui.DrawerRadius
import com.example.swift_final.util.DisplayUtils
import com.example.swift_final.util.DisplayUtils.ScreenPixels.Companion.heightInDp
import com.example.swift_final.util.DisplayUtils.isTablet
import com.example.swift_final.util.HorizontalSpacer
import com.example.swift_final.util.VerticalSpacer
import com.example.swift_final.util.navigateSingleTop

object Drawer {
    fun drawerShape() =
        object : Shape {
            override fun createOutline(
                size: Size,
                layoutDirection: LayoutDirection,
                density: Density
            ): Outline {
                val pixels = DisplayUtils.percentageOfScreenSize(if (isTablet) 45f else 78f)
                val boundaries = Rect(0f, 0f, pixels.width, size.height)
                val isLtr = layoutDirection == LayoutDirection.Ltr
                val radiusObject = CornerRadius(DrawerRadius, DrawerRadius)
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

    @Composable
    //Very dynamic drawer content
    fun DrawerContent(navController: NavController, closeDrawer: () -> Unit) {
        Column(
            modifier = Modifier
                .verticalScroll(rememberScrollState())
        ) {
            //header
            DrawerHeader()
            Log.e("called","drawer")
            //If update show update
            Text(text = "let's gpp", modifier = Modifier
                .clickable {
                    Log.e("here", "notq")
                }
                .fillMaxWidth())
            VerticalSpacer(space = 10)
            DrawerButtons(
                iconResource = R.drawable.ic_folder,
                labelResource = R.string.all_downloads
            ) {

            }
            DrawerButtons(
                iconResource = R.drawable.ic_folders,
                labelResource = R.string.downloaded
            ) {
            }
            DrawerButtons(
                iconResource = R.drawable.ic_download_bordered,
                labelResource = R.string.unfinished
            ) {
                navController.navigateSingleTop(Screens.UnfinishedDownloads.name)
                closeDrawer()
            }

            DrawerButtons(
                iconResource = android.R.drawable.ic_menu_search,
                labelResource = R.string.find
            ) {
            }

            DrawerButtons(iconResource = R.drawable.ic_settings, labelResource = R.string.options) {

            }
            DrawerButtons(
                iconResource = android.R.drawable.ic_dialog_info,
                labelResource = R.string.help
            ) {

            }

            DrawerButtons(
                iconResource = R.drawable.ic_share,
                labelResource = R.string.tell_friend
            ) {

            }
        }
    }

    @Composable
    private fun DrawerButtons(iconResource: Int, labelResource: Int, onClick: () -> Unit) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .clickable(onClick = onClick)
                .padding(DrawerButtonPadding.dp)
        ) {
            val label = stringResource(id = labelResource)
            Image(
                painter = painterResource(id = iconResource),
                contentDescription = label,
                modifier = Modifier.size(30.dp)
            )
            HorizontalSpacer(space = 20)
            Text(text = label)
        }
    }

    @Composable
    private fun DrawerHeader() {
        ConstraintLayout(
            modifier = Modifier
                .height(DisplayUtils.percentageOfScreenSize(20f).heightInDp)
                .background(
                    Color.LightGray
                )
        ) {
            //TODO
        }
    }
}