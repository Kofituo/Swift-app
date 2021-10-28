package com.example.swift_final.ui.home

import android.util.Log
import androidx.compose.foundation.Image
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
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.layout.layoutId
import androidx.compose.ui.layout.onSizeChanged
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.compose.ui.window.Dialog
import androidx.constraintlayout.compose.ConstraintLayout
import androidx.constraintlayout.compose.ConstraintSet
import androidx.constraintlayout.compose.Dimension
import com.example.swift_final.ApplicationLoader
import com.example.swift_final.R
import com.example.swift_final.lib.*
import com.example.swift_final.ui.*
import com.example.swift_final.ui.shimmer.shimmer
import com.example.swift_final.util.DisplayUtils
import com.example.swift_final.util.DisplayUtils.ScreenPixels.Companion.widthInDp
import com.example.swift_final.util.HorizontalSpacer
import com.example.swift_final.util.textFieldBorder
import kotlinx.coroutines.*
import java.util.*
import kotlin.time.ExperimentalTime

@OptIn(ExperimentalComposeUiApi::class, ExperimentalTime::class)
@Composable
fun DownloadInfoDialog(
    downloadInfo: DownloadInfo?,
    showDialog: Boolean,
    setShowDialog: (DownloadInfo?) -> Unit
) {
    if (!showDialog) return
    requireNotNull(downloadInfo)
    val isLandscape = DisplayUtils.isLandscape
    val url = rememberSaveable { mutableStateOf(downloadInfo.url) }
    val filename = rememberSaveable { mutableStateOf("") }
    val filesize = rememberSaveable { mutableStateOf("") }
    val category = rememberSaveable { mutableStateOf("") }
    val resumable = rememberSaveable { mutableStateOf("") }
    var enable by rememberSaveable { mutableStateOf(false) }
    var typeOfFile: TypeOfFile? by rememberSaveable { mutableStateOf(null) }
    val showSimmer = !enable
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
                constraintSet = constraints(isLandscape = isLandscape),
                modifier = Modifier
                    .wrapContentHeight()
                    .fillMaxWidth()
            ) {
                Text(
                    text = stringResource(id = R.string.download_information),
                    modifier = Modifier.layoutId(ConstraintIds.Title),
                )
                Divider(
                    Modifier
                        .layoutId(ConstraintIds.Divider)
                        .fillMaxWidth(0.8f)
                )
                if (isLandscape) {
                    Text(
                        text = "${stringResource(id = R.string.url)}: ",
                        modifier = Modifier.layoutId(ConstraintIds.Url),
                        fontSize = defaultFontSize
                    )
                    Text(
                        text = url.value,
                        modifier = Modifier
                            .layoutId(ConstraintIds.UrlShimmer)
                            .shimmer(show = showSimmer),
                        fontSize = defaultFontSize
                    )
                    Text(
                        text = "${stringResource(id = R.string.file_name)}: ",
                        modifier = Modifier.layoutId(ConstraintIds.FilenameBox),
                        fontSize = defaultFontSize
                    )
                    DownloadInfoTextField(
                        valueHolder = filename,
                        label = 0,
                        modifier = Modifier
                            .layoutId(ConstraintIds.FilenameShimmer)
                            .shimmer(show = showSimmer),
                        enabled = enable
                    )
                    Text(
                        text = "${stringResource(id = R.string.file_size)}: ",
                        modifier = Modifier.layoutId(ConstraintIds.FileSizeBox),
                        fontSize = defaultFontSize
                    )
                    Text(
                        text = filesize.value,
                        modifier = Modifier
                            .layoutId(ConstraintIds.FileSizeShimmer)
                            .shimmer(show = showSimmer),
                        fontSize = defaultFontSize
                    )
                    Text(
                        text = "${stringResource(id = R.string.category)}: ",
                        modifier = Modifier.layoutId(ConstraintIds.CategoryBox),
                        fontSize = defaultFontSize
                    )
                    Category(
                        valueHolder = category,
                        modifier = Modifier
                            .layoutId(ConstraintIds.CategoryShimmer)
                            .shimmer(show = showSimmer), enabled = enable,
                        label = 0
                    )
                    Text(
                        text = "${stringResource(id = R.string.resumable)}: ",
                        modifier = Modifier
                            .layoutId(ConstraintIds.ResumeBox),
                        fontSize = defaultFontSize
                    )
                    Text(
                        text = resumable.value,
                        modifier = Modifier
                            .layoutId(ConstraintIds.ResumableShimmer)
                            .shimmer(show = showSimmer),
                        fontSize = defaultFontSize
                    )
                    Image(
                        painter = painterResource(id = getImageId(typeOfFile)),
                        contentDescription = typeOfFile?.name ?: "",
                        modifier = Modifier
                            .layoutId(ConstraintIds.Icon)
                            .shimmer(show = showSimmer)
                    )

                } else {
                    ConstraintLayout(
                        modifier = Modifier
                            .layoutId(ConstraintIds.ShimmerLayout)
                            .shimmer(show = showSimmer)
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
                                },
                            enabled = enable
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
                            }, enabled = enable
                        )
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
                }
                OutlinedButton(
                    onClick = { /*TODO*/ },
                    modifier =
                    Modifier
                        .layoutId(ConstraintIds.Ok)
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
    var sendRequest by rememberSaveable { mutableStateOf(true) }
    var errorDialogData: ErrorDialogData? by rememberSaveable { mutableStateOf(null) }

    if (sendRequest) {
        LaunchedEffect(true) {
            val callback = object : DownloadCallback {
                var shouldDismiss = true
                override fun responseError(error: ResponseErrors) {
                    val resId = when (error) {
                        ResponseErrors.ConnectionTimeout -> R.string.timeout
                        ResponseErrors.ErrorParsingRequest, ResponseErrors.UnableToDecodeRequest -> R.string.error_parsing_req
                        ResponseErrors.RedirectedManyTimes -> R.string.redirected_many
                        ResponseErrors.UnknownError -> R.string.unknown_error
                    }
                    errorDialogData = ErrorDialogData(
                        body = ApplicationLoader.getString(resId),
                        onDialogDismiss = {
                            errorDialogData = null
                            if (shouldDismiss)
                                setShowDialog(null)
                        },
                        confirmButtonText = ApplicationLoader.getString(R.string.retry),
                        confirmButtonCallback = {
                            errorDialogData = null
                            sendRequest = true
                            shouldDismiss = false
                        },
                        dismissButtonText = ApplicationLoader.getString(R.string.cancel),
                        dismissButtonCallback = {
                            errorDialogData = null
                            setShowDialog(null)
                        })
                }

                override fun statusError(error_code: Int, reason: String) {
                    errorDialogData = ErrorDialogData(
                        title = "${ApplicationLoader.getString(R.string.status_code)} $error_code",
                        body = "${ApplicationLoader.getString(R.string.reason)} $reason",
                        onDialogDismiss = { errorDialogData = null },
                        confirmButtonText = ApplicationLoader.getString(R.string.cancel),
                        confirmButtonCallback = {
                            errorDialogData = null
                            //sendRequest = true no retry
                        }
                    )
                }

                override fun isActive() = this@LaunchedEffect.isActive
            }

            withContext(Dispatchers.IO) {
                val info = Downloader.getRequestInfo(downloadInfo, callback)
                sendRequest = false
                if (info != null) {
                    //success
                    errorDialogData = null
                    filename.value = info.filename()
                    filesize.value =
                        info.fileSize ?: ApplicationLoader.getString(id = R.string.unknown_size)
                    category.value = info.category.name
                    resumable.value =
                        ApplicationLoader.getString(id = if (info.isResumable) R.string.yes else R.string.no)
                    typeOfFile = info.typeOfFile()
                    enable = true
                }
            }
        }
    }
    errorDialogData?.also {
        ErrorDialog(
            title = it.title,
            body = it.body,
            onDialogDismiss = it.onDialogDismiss,
            confirmButtonText = it.confirmButtonText,
            confirmButtonCallback = it.confirmButtonCallback,
            dismissButtonText = it.dismissButtonText,
            dismissButtonCallback = it.dismissButtonCallback
        )
    }
}

data class ErrorDialogData(
    var title: String = ApplicationLoader.getString(id = R.string.error),
    var body: String,
    var onDialogDismiss: () -> Unit,
    var confirmButtonText: String,
    var confirmButtonCallback: () -> Unit,
    var dismissButtonText: String? = null,
    var dismissButtonCallback: (() -> Unit)? = null,
)

private fun getImageId(typeOfFile: TypeOfFile?) =
    when (typeOfFile) {
        TypeOfFile.Excel -> R.drawable.ic_excel
        TypeOfFile.Html -> R.drawable.ic_html
        TypeOfFile.Jpg -> R.drawable.ic_jpg
        TypeOfFile.Mkv -> R.drawable.ic_mkv
        TypeOfFile.MpFour -> R.drawable.ic_mp4
        TypeOfFile.Pdf -> R.drawable.ic_pdf
        TypeOfFile.Png -> R.drawable.ic_png
        TypeOfFile.PowerPoint -> R.drawable.ic_powerpoint
        TypeOfFile.Word -> R.drawable.ic_word
        TypeOfFile.MpThree -> R.drawable.ic_mp3
        TypeOfFile.Gif -> R.drawable.ic_gif
        TypeOfFile.Zip -> R.drawable.ic_zip
        TypeOfFile.Iso -> R.drawable.ic_iso
        TypeOfFile.ThreeGp -> R.drawable.ic_3gp
        TypeOfFile.Flv -> R.drawable.ic_flv
        TypeOfFile.Application -> R.drawable.ic_application
        TypeOfFile.Audio -> R.drawable.ic_audio
        TypeOfFile.Video -> R.drawable.ic_video_player
        TypeOfFile.Image -> R.drawable.ic_photo
        /*TypeOfFile.Document ->
        TypeOfFile.Compressed ->
        null, TypeOfFile.Other -> */
        else -> R.drawable.ic_other
    }

@Composable
private fun ErrorDialog(
    title: String = stringResource(id = R.string.error),
    body: String,
    onDialogDismiss: () -> Unit,
    confirmButtonText: String,
    confirmButtonCallback: () -> Unit,
    dismissButtonText: String? = null,
    dismissButtonCallback: (() -> Unit)? = null,
) {
    AlertDialog(
        onDismissRequest = onDialogDismiss,
        title = {
            Row {
                Icon(
                    painter = painterResource(id = R.drawable.ic_baseline_error_outline_24),
                    contentDescription = stringResource(id = R.string.error_icon),
                    tint = Color.Unspecified
                )
                HorizontalSpacer(space = 10)
                Text(text = title)
            }
        },
        confirmButton = {
            TextButton(onClick = confirmButtonCallback) {
                Text(text = confirmButtonText)
            }
        },
        dismissButton = dismissButtonText?.let {
            {
                requireNotNull(dismissButtonCallback)
                TextButton(onClick = dismissButtonCallback) {
                    Text(text = it)
                }
            }
        },
        text = {
            Text(text = body)
        }
    )
}

//TODO
private fun constraints(isLandscape: Boolean) = ConstraintSet {
    val title = createRefFor(ConstraintIds.Title)
    val divider = createRefFor(ConstraintIds.Divider)
    val shimmerLayout = if (!isLandscape) createRefFor(ConstraintIds.ShimmerLayout) else null
    val downloadButton = createRefFor(ConstraintIds.DownloadButton)
    //Guidelines
    val topGuide = createGuidelineFromTop(0.04f)
    val startGuide = createGuidelineFromStart(if (isLandscape) 0.035f else 0.045f)
    val endGuide = createGuidelineFromEnd(0.045f)
    //
    val okButton = createRefFor(ConstraintIds.Ok)

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
        if (isLandscape)
            width = Dimension.percent(0.8f)
    }
    shimmerLayout?.also {
        constrain(it) {
            bottom.linkTo(downloadButton.top)
            end.linkTo(endGuide)
            start.linkTo(startGuide)
            top.linkTo(divider.bottom)
            width = Dimension.fillToConstraints
        }
        constrain(okButton) {
            end.linkTo(parent.end)
            bottom.linkTo(parent.bottom)
            top.linkTo(it.bottom)
        }
    }
    if (isLandscape) {
        val urlShimmer = createRefFor(ConstraintIds.UrlShimmer)
        val iconGuide = createGuidelineFromEnd(0.2f)
        val filenameShimmer = createRefFor(ConstraintIds.FilenameShimmer)
        val url = createRefFor(ConstraintIds.Url)
        val filenameBox = createRefFor(ConstraintIds.FilenameBox)
        val filesizeBox = createRefFor(ConstraintIds.FileSizeBox)
        val filesizeShimmer = createRefFor(ConstraintIds.FileSizeShimmer)
        val categoryBox = createRefFor(ConstraintIds.CategoryBox)
        val resumeBox = createRefFor(ConstraintIds.ResumeBox)
        val resumableShimmer = createRefFor(ConstraintIds.ResumableShimmer)
        val categoryShimmer = createRefFor(ConstraintIds.CategoryShimmer)
        val iconLayout = createRefFor(ConstraintIds.Icon)
        val namesEnd = createEndBarrier(filenameBox, margin = 1.dp)
        val topMargin = 10.dp
        constrain(url) {
            bottom.linkTo(urlShimmer.bottom)
            end.linkTo(namesEnd)
            start.linkTo(startGuide)
            top.linkTo(urlShimmer.top)
            width = Dimension.fillToConstraints
        }
        constrain(urlShimmer) {
            bottom.linkTo(filenameShimmer.top)
            end.linkTo(iconGuide)
            start.linkTo(namesEnd)
            top.linkTo(divider.bottom, topMargin)
            width = Dimension.fillToConstraints
        }

        constrain(filenameBox) {
            bottom.linkTo(filenameShimmer.bottom)
            end.linkTo(namesEnd)
            start.linkTo(startGuide)
            top.linkTo(filenameShimmer.top)
            width = Dimension.fillToConstraints
        }
        constrain(filenameShimmer) {
            bottom.linkTo(filesizeShimmer.top)
            end.linkTo(iconGuide)
            start.linkTo(namesEnd)
            top.linkTo(urlShimmer.bottom, topMargin)
            width = Dimension.fillToConstraints
        }
        constrain(filesizeBox) {
            bottom.linkTo(filesizeShimmer.bottom)
            end.linkTo(namesEnd)
            start.linkTo(startGuide)
            width = Dimension.fillToConstraints
        }
        constrain(filesizeShimmer) {
            bottom.linkTo(categoryShimmer.top, topMargin)
            end.linkTo(iconGuide)
            start.linkTo(namesEnd)
            top.linkTo(filenameShimmer.bottom, topMargin)
            width = Dimension.fillToConstraints
        }
        constrain(categoryBox) {
            bottom.linkTo(categoryShimmer.bottom)
            end.linkTo(namesEnd)
            start.linkTo(startGuide)
            top.linkTo(categoryShimmer.top)
            width = Dimension.fillToConstraints
        }
        constrain(categoryShimmer) {
            end.linkTo(iconGuide)
            bottom.linkTo(resumableShimmer.top)
            start.linkTo(namesEnd)
            top.linkTo(filesizeShimmer.bottom)
            width = Dimension.fillToConstraints
        }
        constrain(resumeBox) {
            bottom.linkTo(resumableShimmer.bottom)
            end.linkTo(namesEnd)
            start.linkTo(startGuide)
            top.linkTo(resumableShimmer.top)
            //width = Dimension.fillToConstraints
        }
        constrain(resumableShimmer) {
            bottom.linkTo(okButton.top)
            end.linkTo(iconGuide)
            start.linkTo(namesEnd)
            top.linkTo(categoryShimmer.bottom, topMargin)
            width = Dimension.fillToConstraints
        }

        constrain(okButton) {
            bottom.linkTo(parent.bottom)
            end.linkTo(endGuide)
            start.linkTo(iconGuide)
            top.linkTo(resumableShimmer.bottom)
        }
        constrain(iconLayout) {
            bottom.linkTo(resumableShimmer.bottom)
            end.linkTo(parent.end)
            start.linkTo(iconGuide)
            top.linkTo(urlShimmer.top)
            width = Dimension.percent(0.18f)
            height = width
        }
    }
}

private object ConstraintIds {
    const val Title = "title"
    const val Divider = "divider"
    const val ShimmerLayout = "shimmer"
    const val DownloadButton = "download"
    const val Url = "url"
    const val UrlShimmer = "url shimmer"
    const val FilenameBox = "filenameBox"
    const val FilenameShimmer = "FilenameShimmer"
    const val FileSizeBox = "filesizeBox"
    const val FileSizeShimmer = "FileSizeShimmer"
    const val CategoryBox = "category"
    const val CategoryShimmer = "CategoryShimmer"
    const val ResumeBox = "ResumeBox"
    const val ResumableShimmer = "ResumableShimmer"
    const val Ok = "ok"
    const val Icon = "icon"
}

@Composable
private fun DownloadInfoTextField(
    valueHolder: MutableState<String>,
    label: Int,
    modifier: Modifier,
    enabled: Boolean = true,
    readOnly: Boolean = false,
    trailingIcon: @Composable (() -> Unit)? = null,
    interactionSource: MutableInteractionSource = remember { MutableInteractionSource() },
) {
    OutlinedTextField(
        value = valueHolder.value,
        onValueChange = { valueHolder.value = it },
        modifier = modifier,
        label =
        if (label != 0) {
            { Text(text = stringResource(id = label)) }
        } else null,
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
private fun Category(
    valueHolder: MutableState<String>,
    modifier: Modifier,
    enabled: Boolean,
    label: Int = R.string.category
) {
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
            label = label,
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
            interactionSource = interactionSource,
            enabled = enabled
        )
    }
}

//TODO support more locales
private val categories = FileCategory.values().map { it.name }

private val topMargin = 14.dp