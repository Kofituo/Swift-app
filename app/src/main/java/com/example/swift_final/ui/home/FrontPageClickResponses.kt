package com.example.swift_final.ui.home

import android.content.ClipDescription
import android.util.Log
import android.view.Gravity
import android.widget.Toast
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Info
import androidx.compose.material.icons.filled.Visibility
import androidx.compose.material.icons.filled.VisibilityOff
import androidx.compose.runtime.*
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.ui.Alignment
import androidx.compose.ui.ExperimentalComposeUiApi
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.Rect
import androidx.compose.ui.geometry.Size
import androidx.compose.ui.geometry.toRect
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.Outline
import androidx.compose.ui.graphics.Shape
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.input.PasswordVisualTransformation
import androidx.compose.ui.text.input.TextFieldValue
import androidx.compose.ui.text.input.VisualTransformation
import androidx.compose.ui.unit.Density
import androidx.compose.ui.unit.LayoutDirection
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.compose.ui.window.Dialog
import androidx.compose.ui.window.DialogProperties
import androidx.constraintlayout.compose.ConstraintLayout
import androidx.constraintlayout.compose.Dimension
import androidx.core.content.getSystemService
import androidx.core.view.GravityCompat
import androidx.lifecycle.viewmodel.compose.viewModel
import com.example.swift_final.ApplicationLoader
import com.example.swift_final.R
import com.example.swift_final.ui.*
import com.example.swift_final.util.*
import com.example.swift_final.util.DisplayUtils.ScreenPixels.Companion.widthInDp

@Composable
fun OnAddUrlClicked(showDialog: Boolean, setShowDialog: (Boolean) -> Unit) {
    if (!showDialog) return
    var checkState: MutableState<Boolean>?
    var userName by rememberSaveable { mutableStateOf("") }
    var password by rememberSaveable { mutableStateOf("") }
    Dialog(onDismissRequest = {
        setShowDialog(false)
        //Log.e("dismmissed", "hyoyu $checkState")
    }, properties = dialogProperties) {
        val roundedCornerShape = RoundedCornerShape(DialogRadius.dp)
        Column(
            modifier =
            Modifier
                .width(DisplayUtils.percentageOfScreenSize(DialogWidth, true).widthInDp)
                .wrapContentHeight()
                .border(textFieldBorder(width = OutlineWidth.dp), roundedCornerShape)
                .background(color = MaterialTheme.colors.surface, roundedCornerShape)
            //.padding(18.dp, 18.dp, 18.dp, 10.dp),
        ) {
            ConstraintLayout(modifier = Modifier.fillMaxWidth()) {
                val topGuideline = createGuidelineFromTop(0.035f)
                val startGuide = createGuidelineFromStart(0.07f)
                val (title, urlBox, useAuth, usernameBox, passwordBox, okButton) = createRefs()
                Text(
                    text = stringResource(id = R.string.add_url_title),
                    modifier = Modifier.constrainAs(title) {
                        end.linkTo(parent.end)
                        start.linkTo(parent.start)
                        top.linkTo(topGuideline)
                    })
                //VerticalSpacer(space = 15)
                AddressField(Modifier.constrainAs(urlBox) {
                    end.linkTo(parent.end)
                    start.linkTo(parent.start)
                    top.linkTo(title.bottom, topMargin)
                    width = Dimension.percent(0.88f)
                })
                //VerticalSpacer(space = 18)
                checkState = useAuth(Modifier.constrainAs(useAuth) {
                    bottom.linkTo(usernameBox.top)
                    start.linkTo(startGuide)
                    top.linkTo(urlBox.bottom, topMargin)
                })
                //VerticalSpacer(space = 16)
                OutlinedTextField(
                    value = run {
                        if (!checkBoxEnabled(checkState)) userName = ""
                        userName
                    },
                    onValueChange = { userName = it },
                    label = { Text(text = stringResource(id = R.string.username)) },
                    shape = textFieldShape,
                    enabled = checkBoxEnabled(checkState),
                    modifier = Modifier.constrainAs(usernameBox) {
                        end.linkTo(parent.end)
                        start.linkTo(parent.start)
                        top.linkTo(useAuth.bottom,4.dp)
                        width = Dimension.percent(0.85f)
                    }
                )
                //VerticalSpacer(space = 16)
                var showPassword by remember { mutableStateOf(false) }
                OutlinedTextField(
                    value = run {
                        if (!checkBoxEnabled(checkState)) password = ""
                        password
                    },
                    onValueChange = { password = it },
                    label = { Text(text = stringResource(id = R.string.password)) },
                    shape = textFieldShape,
                    enabled = checkBoxEnabled(checkState),
                    visualTransformation = if (showPassword) VisualTransformation.None else PasswordVisualTransformation(),
                    trailingIcon = {
                        //TODO animate between the 2 icons
                        IconButton(onClick = { showPassword = !showPassword }) {
                            Icon(
                                imageVector = if (showPassword) Icons.Filled.Visibility else Icons.Filled.VisibilityOff,
                                contentDescription = stringResource(id = R.string.toggle_password)
                            )
                        }
                    },
                    modifier = Modifier.constrainAs(passwordBox) {
                        width = Dimension.percent(0.85f)
                        end.linkTo(parent.end)
                        start.linkTo(parent.start)
                        top.linkTo(usernameBox.bottom, topMargin)
                        bottom.linkTo(okButton.top)
                    },
                )
                //VerticalSpacer(space = 15)
                val dialogViewModel = viewModel<DialogViewModel>()
                val downloadInfoViewModel = viewModel<DownloadInfoViewModel>()
                OutlinedButton(
                    onClick = {
                        //check for invalid url
                        //Jusg
                        if (dialogViewModel.initialText.value?.text.isNullOrBlank()) {
                            //set error
                            dialogViewModel.setIsError(true)
                            return@OutlinedButton
                        }
                        downloadInfoViewModel.setShowDialog(
                            true,
                            DownloadInfoViewModel.DownloadInfo.new(
                                dialogViewModel.initialText.value?.text!!,
                                checkState!!.value, userName, password
                            )
                        )
                        setShowDialog(false)
                    },
                    shape = textFieldShape,
                    modifier = Modifier.constrainAs(okButton) {
                        bottom.linkTo(parent.bottom)
                        end.linkTo(parent.end)
                        top.linkTo(passwordBox.bottom)
                    }.padding(14.dp),
                    border = textFieldBorder(width = OutlineWidth.dp),
                    elevation = ButtonDefaults.elevation(
                        defaultElevation = 4.dp,
                        pressedElevation = 1.dp
                    )
                ) {
                    Text(text = stringResource(id = android.R.string.ok))
                }
            }
        }
    }
}

@Composable
fun DialogButton(textResource: Int, onClick: () -> Unit) {

}

private fun checkBoxEnabled(checkState: MutableState<Boolean>?) = checkState?.value ?: false

@Composable
private fun useAuth(modifier: Modifier): MutableState<Boolean> {
    val checkState = rememberSaveable { mutableStateOf(false) }
    val onCheckChange = { isChecked: Boolean -> checkState.value = isChecked }
    val interaction = remember { MutableInteractionSource() }
    Row(modifier.clickable(interaction, indication = null) {
        onCheckChange(!checkState.value)
    },verticalAlignment = Alignment.CenterVertically) {
        Checkbox(
            checked = checkState.value,
            onCheckedChange = onCheckChange,
            interactionSource = interaction
        )
        HorizontalSpacer(space = 8)
        Text(text = stringResource(id = R.string.use_authorisation), fontSize = 14.sp)
    }
    return checkState
}

@Composable
private fun AddressField(modifier: Modifier) {
    //Get url from clipboard
    val dialogViewModel = viewModel<DialogViewModel>()
    val initialValueState by dialogViewModel.initialText.observeAsState()
    val isError by dialogViewModel.isError.observeAsState(false)
    val value =
        initialValueState
            ?.let {
                if (it.annotatedString.isBlank() && dialogViewModel.autoModifyInSection) null //means user changed
                else it
            } ?: setAddUrlTextField().also {
            dialogViewModel.setInitialText(it)
            //Log.e("called alsoe", "alsose ${dialogViewModel.autoModifyInSection}")
        }
    //Log.e("initttttttt", " |${initialValueState?.annotatedString}| ${setAddUrlTextField()} $value")
    OutlinedTextField(
        value = value,
        onValueChange = {
            dialogViewModel.setInitialText(it)
            //means the user is updating the website
            dialogViewModel.autoModifyInSection = false
            dialogViewModel.setIsError(false)
        },
        label = {
            Text(text = stringResource(id = R.string.address))
        },
        shape = textFieldShape,
        singleLine = true,
        modifier = modifier,
        isError = isError,
        trailingIcon =
        if (isError) {
            { //show icon if is error
                IconButton(onClick = {
                    //show toast for now
                    Toast.makeText(
                        ApplicationLoader.applicationContext,
                        R.string.invalid_url_toast,
                        Toast.LENGTH_SHORT
                    ).show()
                }) {
                    Icon(
                        imageVector = Icons.Filled.Info,
                        contentDescription = stringResource(id = R.string.invalid_url)
                    )
                }
            }
        } else null
    )
    Log.e("com[ppin", "texxxxxxxxxxxxxffffffff ${dialogViewModel.autoModifyInSection}")
}

private fun geCopiedText(): String? {
    val clipboardManager =
        ApplicationLoader.applicationContext.getSystemService<android.content.ClipboardManager>()
    return clipboardManager?.run {
        val containsText =
            primaryClipDescription?.hasMimeType(ClipDescription.MIMETYPE_TEXT_PLAIN)
                ?: false
        Log.e(
            "contains text",
            "dsdcasc $containsText $primaryClip ${
                primaryClip?.description?.hasMimeType(ClipDescription.MIMETYPE_TEXT_PLAIN)
            }"
        )
        if (containsText) primaryClip?.getItemAt(0)?.text?.toString() else null
    }
}

fun setAddUrlTextField() =
    TextFieldValue(geCopiedText()?.let { if (it.isUrl) it else null } ?: "")

private val topMargin = 12.dp