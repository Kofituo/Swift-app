package com.example.swift_final.ui.home

import androidx.compose.foundation.layout.*
import androidx.compose.material.*
import androidx.compose.runtime.Composable
import androidx.compose.runtime.CompositionLocalProvider
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.RectangleShape
import androidx.compose.ui.graphics.Shape
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp

@Composable
private fun AppBar(
    backgroundColor: Color,
    contentColor: Color,
    elevation: Dp,
    contentPadding: PaddingValues,
    shape: Shape,
    modifier: Modifier = Modifier,
    content: @Composable RowScope.() -> Unit
) {
    Surface(
        color = backgroundColor,
        contentColor = contentColor,
        elevation = elevation,
        shape = shape,
        modifier = modifier
    ) {
        Row(
            Modifier
                .fillMaxWidth()
                .padding(contentPadding)
                .height(100.dp),
            horizontalArrangement = Arrangement.End,
            verticalAlignment = Alignment.Top,
            content = content
        )
    }
}

@Composable
fun TopAppBar(
    title: @Composable () -> Unit,
    modifier: Modifier = Modifier,
    navigationIcon: @Composable (() -> Unit)? = null,
    actions: @Composable RowScope.() -> Unit = {},
    backgroundColor: Color = MaterialTheme.colors.primarySurface,
    contentColor: Color = contentColorFor(backgroundColor),
    elevation: Dp = AppBarDefaults.TopAppBarElevation
) {
    AppBar(
        backgroundColor,
        contentColor,
        elevation,
        AppBarDefaults.ContentPadding,
        RectangleShape,
        modifier
    ) {
        if (navigationIcon == null) {
            Spacer(TitleInsetWithoutIcon)
        } else {
            Row(
                TitleIconModifier,
                verticalAlignment = Alignment.CenterVertically
            ) {
                CompositionLocalProvider(
                    LocalContentAlpha provides ContentAlpha.high,
                    content = navigationIcon
                )
            }
        }

        Row(
            Modifier
                .fillMaxHeight()
                .weight(1f),
            verticalAlignment = Alignment.CenterVertically
        ) {
            ProvideTextStyle(value = MaterialTheme.typography.body1) {
                CompositionLocalProvider(
                    LocalContentAlpha provides ContentAlpha.high,
                    content = title
                )
            }
        }

        CompositionLocalProvider(LocalContentAlpha provides ContentAlpha.medium) {
            Row(
                Modifier.fillMaxHeight(),
                horizontalArrangement = Arrangement.Start,
                verticalAlignment = Alignment.Top,
                content = actions
            )
        }
    }
}

// TODO: this should probably be part of the touch target of the start and end icons, clarify this
private val AppBarHorizontalPadding = 4.dp

// Start inset for the title when there is no navigation icon provided
private val TitleInsetWithoutIcon = Modifier.width(16.dp - AppBarHorizontalPadding)

// Start inset for the title when there is a navigation icon provided
private val TitleIconModifier = Modifier
    .fillMaxHeight()
    .width(72.dp - AppBarHorizontalPadding)
