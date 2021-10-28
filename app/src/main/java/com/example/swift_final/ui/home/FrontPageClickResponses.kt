package com.example.swift_final.ui.home

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
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.input.PasswordVisualTransformation
import androidx.compose.ui.text.input.VisualTransformation
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.compose.ui.window.Dialog
import androidx.constraintlayout.compose.ConstraintLayout
import androidx.constraintlayout.compose.Dimension
import androidx.lifecycle.viewmodel.compose.viewModel
import com.example.swift_final.ApplicationLoader
import com.example.swift_final.R
import com.example.swift_final.ui.*
import com.example.swift_final.util.*
import com.example.swift_final.util.DisplayUtils.ScreenPixels.Companion.widthInDp

@Composable
fun OnAddUrlClicked(dialogViewModel: DialogViewModel) {
    val showDialog by dialogViewModel.dialogShowing.observeAsState(false)
    if (!showDialog) return
    var checkState: MutableState<Boolean>?
    var userName by rememberSaveable { mutableStateOf("") }
    var password: String? by rememberSaveable { mutableStateOf(null) }
    Dialog(onDismissRequest = {
        dialogViewModel.setShowDialog(false)
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
                AddressField(dialogViewModel,Modifier.constrainAs(urlBox) {
                    end.linkTo(parent.end)
                    start.linkTo(parent.start)
                    top.linkTo(title.bottom, topMargin)
                    width = Dimension.percent(0.88f)
                })
                checkState = useAuth(Modifier.constrainAs(useAuth) {
                    bottom.linkTo(usernameBox.top)
                    start.linkTo(startGuide)
                    top.linkTo(urlBox.bottom, topMargin)
                })
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
                        top.linkTo(useAuth.bottom, 4.dp)
                        width = Dimension.percent(0.85f)
                    }
                )
                var showPassword by remember { mutableStateOf(false) }
                OutlinedTextField(
                    value = run {
                        if (!checkBoxEnabled(checkState)) password = null //reset password
                        password ?: ""
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
                val downloadInfoViewModel = viewModel<DownloadInfoViewModel>()
                OutlinedButton(
                    onClick = {
                        //check for invalid url
                        if (dialogViewModel.url.isBlank()) {
                            //set error
                            dialogViewModel.setIsError(true)
                            return@OutlinedButton
                        }
                        downloadInfoViewModel.setShowDialog(
                            DownloadInfoViewModel.newDownloadInfo(
                                dialogViewModel.url,
                                checkState!!.value, userName, password
                            )
                        )
                        dialogViewModel.setShowDialog(false)
                    },
                    shape = textFieldShape,
                    modifier = Modifier
                        .constrainAs(okButton) {
                            bottom.linkTo(parent.bottom)
                            end.linkTo(parent.end)
                            top.linkTo(passwordBox.bottom)
                        }
                        .padding(14.dp),
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
    }, verticalAlignment = Alignment.CenterVertically) {
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
private fun AddressField(dialogViewModel: DialogViewModel, modifier: Modifier) {
    if (!dialogViewModel.dialogIsShowing) return
    val url by dialogViewModel.urlLiveData.observeAsState()
    val isError by dialogViewModel.isError.observeAsState(false)
    OutlinedTextField(
        value = url ?: copiedUrl?.also { dialogViewModel.setUrl(it) } ?: "",
        onValueChange = {
            //means the user is updating the url
            dialogViewModel.setUrl(it)
            dialogViewModel.isModified = true
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
}

private val topMargin = 12.dp