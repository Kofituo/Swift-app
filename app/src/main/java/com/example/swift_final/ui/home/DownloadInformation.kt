package com.example.swift_final.ui.home

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.interaction.collectIsPressedAsState
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ArrowDropDown
import androidx.compose.material.icons.filled.ArrowDropUp
import androidx.compose.runtime.*
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.ui.ExperimentalComposeUiApi
import androidx.compose.ui.Modifier
import androidx.compose.ui.layout.onSizeChanged
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import androidx.compose.ui.window.Dialog
import androidx.constraintlayout.compose.ConstraintLayout
import androidx.constraintlayout.compose.ConstraintSet
import androidx.constraintlayout.compose.Dimension
import com.example.swift_final.R
import com.example.swift_final.lib.DownloadCallback
import com.example.swift_final.lib.DownloadInfo
import com.example.swift_final.lib.FileCategory
import com.example.swift_final.lib.ResponseErrors
import com.example.swift_final.ui.*
import com.example.swift_final.util.DisplayUtils
import com.example.swift_final.util.DisplayUtils.ScreenPixels.Companion.widthInDp
import com.example.swift_final.util.textFieldBorder
import com.example.swift_final.ui.shimmer.shimmer


@OptIn(ExperimentalComposeUiApi::class)
@Composable
fun DownloadInfoDialog(
    downloadInfo: DownloadInfo?,
    showDialog: Boolean,
    setShowDialog: (DownloadInfo?) -> Unit
) {
    if (!showDialog) return
    requireNotNull(downloadInfo)
    val url = rememberSaveable { mutableStateOf(downloadInfo.url) }
    val filename = rememberSaveable { mutableStateOf("") }
    val filesize = rememberSaveable { mutableStateOf("") }
    val category = rememberSaveable { mutableStateOf("") }
    val resumable = rememberSaveable { mutableStateOf("") }
    val showShimmer by rememberSaveable { mutableStateOf(true) }

    Dialog(
        onDismissRequest = { setShowDialog(null) },
        properties = dialogProperties
    ) {
        val dialogShape = RoundedCornerShape(DialogRadius.dp)
        Box(
            modifier =
            Modifier
                .width(DisplayUtils.percentageOfScreenSize(DialogWidth, true).widthInDp)
                .background(color = MaterialTheme.colors.surface, dialogShape)
                .border(textFieldBorder(width = OutlineWidth.dp), dialogShape)

        ) {
            ConstraintLayout(
                modifier = Modifier
                    .wrapContentHeight()
                    .fillMaxWidth()
            ) {
                val (title, divider, shimmer, downloadButton) = createRefs()
                val topGuide = createGuidelineFromTop(0.04f)
                val startGuide = createGuidelineFromStart(0.045f)
                val endGuide = createGuidelineFromEnd(0.045f)

                Text(
                    text = stringResource(id = R.string.download_information),
                    modifier = Modifier.constrainAs(title) {
                        bottom.linkTo(divider.top)
                        end.linkTo(parent.end)
                        start.linkTo(parent.start)
                        top.linkTo(topGuide)
                    },
                )
                Divider(
                    Modifier
                        .constrainAs(divider) {
                            end.linkTo(endGuide)
                            start.linkTo(startGuide)
                            top.linkTo(title.bottom, topMargin)
                        }
                        .fillMaxWidth(0.8f))
                ConstraintLayout(modifier = Modifier
                    .constrainAs(shimmer) {
                        bottom.linkTo(downloadButton.top)
                        end.linkTo(endGuide)
                        start.linkTo(startGuide)
                        top.linkTo(divider.bottom)
                        width = Dimension.fillToConstraints
                    }
                    .shimmer(show = showShimmer)
                ) {
                    val (urlBox, filenameBox, filesizeBox, resumeBox, categoryBox) = createRefs()
                    DownloadInfoTextField(
                        valueHolder = url,
                        label = R.string.url,
                        modifier = Modifier
                            .constrainAs(urlBox) {
                                bottom.linkTo(filenameBox.top)
                                end.linkTo(parent.end)
                                start.linkTo(parent.start)
                                top.linkTo(parent.top, topMargin)
                                width = Dimension.fillToConstraints
                            },
                        enabled = false
                    )
                    DownloadInfoTextField(
                        valueHolder = filename,
                        label = R.string.file_name,
                        modifier = Modifier
                            .constrainAs(filenameBox) {
                                bottom.linkTo(filesizeBox.top)
                                end.linkTo(parent.end)
                                start.linkTo(parent.start)
                                top.linkTo(urlBox.bottom, topMargin)
                                width = Dimension.fillToConstraints
                            }
                    )
                    DownloadInfoTextField(
                        valueHolder = filesize,
                        label = R.string.file_size,
                        modifier = Modifier.constrainAs(filesizeBox) {
                            bottom.linkTo(categoryBox.top)
                            end.linkTo(parent.end)
                            start.linkTo(parent.start)
                            top.linkTo(filenameBox.bottom, topMargin)
                            width = Dimension.fillToConstraints
                        }, enabled = false
                    )

                    Category(
                        valueHolder = category,
                        modifier = Modifier.constrainAs(categoryBox) {
                            bottom.linkTo(resumeBox.top)
                            end.linkTo(parent.end)
                            start.linkTo(parent.start)
                            top.linkTo(filesizeBox.bottom, topMargin)
                            width = Dimension.fillToConstraints
                        })
                    DownloadInfoTextField(
                        valueHolder = resumable,
                        label = R.string.resumable,
                        modifier = Modifier.constrainAs(resumeBox) {
                            bottom.linkTo(parent.bottom)
                            end.linkTo(parent.end)
                            start.linkTo(parent.start)
                            top.linkTo(categoryBox.bottom, topMargin)
                            width = Dimension.fillToConstraints
                        }, enabled = false
                    )
                }
                OutlinedButton(
                    onClick = { /*TODO*/ },
                    modifier =
                    Modifier
                        .constrainAs(downloadButton) {
                            end.linkTo(parent.end)
                            bottom.linkTo(parent.bottom)
                            top.linkTo(shimmer.bottom)
                        }
                        .padding(topMargin), shape = textFieldShape,
                    border = textFieldBorder(width = OutlineWidth.dp),
                    elevation = ButtonDefaults.elevation(
                        defaultElevation = 4.dp,
                        pressedElevation = 1.dp
                    )
                ) {
                    Text(text = stringResource(id = R.string.download))
                }
            }
        }
    }

    // load data

    object : DownloadCallback {
        override fun responseError(error: ResponseErrors) {
        }

        override fun statusError(error_code: Int, reason: String) {

        }
    }
}

@Composable
private fun ProcessRequest() {

}

//TODO
private fun constraints(isLandscape: Boolean) = ConstraintSet {
    val title = createRefFor(ConstraintIds.Title)
    val divider = createRefFor(ConstraintIds.Divider)
    val shimmer = createRefFor(ConstraintIds.ShimmerLayout)
    val downloadButton = createRefFor(ConstraintIds.DownloadButton)
    //Guidelines
    val iconGuideline = if (isLandscape) createGuidelineFromEnd(0.2f) else null
    val topGuide = createGuidelineFromTop(0.04f)
    val startGuide = createGuidelineFromStart(0.045f)
    val endGuide = createGuidelineFromEnd(0.045f)

    constrain(title) {
        bottom.linkTo(divider.top)
        end.linkTo(parent.end)
        start.linkTo(parent.start)
        top.linkTo(topGuide)
    }

    constrain(divider) {
        end.linkTo(endGuide)
        start.linkTo(startGuide)
        top.linkTo(title.bottom, topMargin)
    }
}

private object ConstraintIds {
    const val Title = "title"
    const val Divider = "divider"
    const val ShimmerLayout = "shimmer"
    const val DownloadButton = "download"
}

@Composable
private fun DownloadInfoTextField(
    valueHolder: MutableState<String>,
    label: Int,
    modifier: Modifier,
    enabled: Boolean = true,
    readOnly: Boolean = false,
    trailingIcon: @Composable (() -> Unit)? = null,
    interactionSource: MutableInteractionSource = remember { MutableInteractionSource() }
) {
    OutlinedTextField(
        value = valueHolder.value,
        onValueChange = { valueHolder.value = it },
        modifier = modifier,
        label = { Text(text = stringResource(id = label)) },
        shape = textFieldShape,
        singleLine = true,
        enabled = enabled,
        readOnly = readOnly,
        trailingIcon = trailingIcon,
        interactionSource = interactionSource
    )
}

@Composable
//TODO: Support more locales if app blows
private fun Category(valueHolder: MutableState<String>, modifier: Modifier) {
    var isExpanded by remember { mutableStateOf(false) }

    Column(modifier = modifier) {
        val interactionSource = remember { MutableInteractionSource() }
        if (interactionSource.collectIsPressedAsState().value) isExpanded = true
        var menuWidth by remember { mutableStateOf(0) }
        DropdownMenu(
            expanded = isExpanded,
            onDismissRequest = { isExpanded = false },
            modifier = Modifier.width(LocalDensity.current.run { menuWidth.toDp() }),
        ) {
            categories.forEach { category ->
                DropdownMenuItem(onClick = {
                    assert(isExpanded) //remove
                    valueHolder.value = category
                    isExpanded = false
                }) {
                    Text(text = category)
                }
            }
        }
        DownloadInfoTextField(
            valueHolder = valueHolder,
            modifier = Modifier
                .fillMaxWidth()
                .onSizeChanged { menuWidth = it.width },
            label = R.string.category,
            readOnly = true,
            trailingIcon = {
                //TODO animate between the 2 icons
                IconButton(onClick = { isExpanded = !isExpanded }) {
                    Icon(
                        imageVector = if (isExpanded) Icons.Filled.ArrowDropUp else Icons.Filled.ArrowDropDown,
                        contentDescription = stringResource(id = R.string.categories)
                    )
                }
            },
            interactionSource = interactionSource
        )
    }
}

//TODO support more locales
private val categories = FileCategory.values().map { it.name }

private val topMargin = 14.dp