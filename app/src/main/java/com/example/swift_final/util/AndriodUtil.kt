package com.example.swift_final.util

import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.width
import androidx.compose.material.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp

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