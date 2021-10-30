package com.example.swift_final.ui.home

import android.util.Log
import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Menu
import androidx.compose.material.icons.filled.Search
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontStyle
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavController
import com.example.swift_final.R
import com.example.swift_final.Screens
import com.example.swift_final.ui.*
import com.example.swift_final.util.*
import com.example.swift_final.util.DisplayUtils.ScreenPixels.Companion.widthInDp

@Composable
fun FrontPageContent(navController: NavController,openDrawer:() -> Unit, paddingValues: PaddingValues) {
    Column {
        TopAppBar(
            title = {
                Text(
                    "Swift",
                    fontSize = 36.sp,
                    fontFamily = FontFamily.Serif,
                    fontStyle = FontStyle.Italic
                )
            },
            navigationIcon = {
                IconButton(
                    onClick = {
                        openDrawer()
                    }
                ) {
                    Icon(
                        Icons.Filled.Menu,
                        contentDescription = stringResource(id = R.string.menu),
                        Modifier.size(28.dp)
                    )
                }
            },
            actions = {
                IconButton(onClick = { /*TODO*/ }) {
                    Icon(
                        imageVector = Icons.Filled.Search,
                        contentDescription = stringResource(id = R.string.search),
                        Modifier.size(28.dp)
                    )
                }
            }
        )
        Column(
            modifier = Modifier
                .padding(paddingValues)
                .fillMaxSize()
                .verticalScroll(rememberScrollState()),
            horizontalAlignment = Alignment.CenterHorizontally
        ) {
            Spacer(modifier = Modifier.height(20.dp))
            val dialogViewModel: DialogViewModel = viewModel()
            // Create alert dialog, pass the showDialog state to this Composable
            OnAddUrlClicked(dialogViewModel = dialogViewModel)
            val downloadInfoViewModel = viewModel<DownloadInfoViewModel>()
            val showDownloadInfoViewDialog by downloadInfoViewModel.dialogShowing.observeAsState(
                initial = false
            )
            val sendRequest by downloadInfoViewModel.sendRequest.observeAsState(initial = true)

            DownloadInfoDialog(
                downloadInfoViewModel.downloadInfo,
                showDialog = showDownloadInfoViewDialog,
                setShowDialog = downloadInfoViewModel.setShowDialog,
                sendRequest = sendRequest,
                setSendRequest = downloadInfoViewModel.setSendRequest,
                setSuccess = downloadInfoViewModel.setSuccess,
                navController = navController
            )
            OutlinedButton(
                onClick = { dialogViewModel.setShowDialog(true) },
                border = textFieldBorder(width = OutlineWidth.dp),
                shape = frontPageButtonShape,
                elevation = ButtonDefaults.elevation(defaultElevation = 14.dp),
                modifier = Modifier.size(140.dp)
                //colors =ButtonDefaults.outlinedButtonColors(backgroundColor = MaterialTheme.colors.surface.copy(0f))
            ) {
                Column(horizontalAlignment = Alignment.CenterHorizontally) {
                    Image(
                        painter = painterResource(id = R.drawable.ic_add),
                        contentDescription = "Add",
                        modifier = Modifier
                            .padding(15.dp)
                            .size(AddUrlImageSize.dp)
                    )
                    //Spacer(modifier = Modifier.height(16.dp))
                    Text(
                        text = stringResource(id = R.string.add_url),
                        fontSize = 17.sp,
                        fontFamily = FontFamily.Serif, fontWeight = FontWeight(600)
                    )
                }
            }
            VerticalSpacer(space = 30)
            Divider()
            VerticalSpacer(space = 15)
            Row(horizontalArrangement = Arrangement.Center, modifier = Modifier.padding(10.dp)) {
                ExtraButtons(
                    imageId = R.drawable.ic_folders,
                    text = R.string.downloaded,
                    modifier = Modifier.weight(1f)
                ) {
                    Log.e("clicedk", "here")
                }
                HorizontalSpacer(space = 15)
                ExtraButtons(
                    imageId = R.drawable.ic_download_bordered,
                    text = R.string.unfinished,
                    modifier = Modifier.weight(1f)
                ) {
                    /*on click */
                    navController.navigateSingleTop(Screens.UnfinishedDownloads.name)
                }
                HorizontalSpacer(space = 15)
                ExtraButtons(
                    imageId = R.drawable.ic_settings,
                    text = R.string.options,
                    modifier = Modifier.weight(1f)
                ) {

                }
                HorizontalSpacer(space = 15)
                ExtraButtons(
                    imageId = R.drawable.ic_share,
                    text = R.string.share,
                    modifier = Modifier.weight(1f)
                ) {
                }
            }
            VerticalSpacer(space = 20)
            Divider()
            VerticalSpacer(space = 30)

            TextButton(
                onClick = { /*TODO*/ },
                shape = RoundedCornerShape(100f),
                modifier =
                Modifier
                    .width(DisplayUtils.percentageOfScreenSize(50f, true).widthInDp)
            ) {
                Text(text = stringResource(id = R.string.help))
            }
            VerticalSpacer(space = 10)
            //TODO show on going downloads and /or ads at the bottom
        }
    }
}

private inline val frontPageButtonShape get() = RoundedCornerShape(FrontPageButtonRadius.dp)

@Composable
private fun ExtraButtons(
    modifier: Modifier = Modifier,
    imageId: Int,
    text: Int,
    contentDescription: Int = text,
    onClick: () -> Unit
) {
    OutlinedButton(
        onClick = onClick,
        border = textFieldBorder(width = OutlineWidth.dp),
        shape = frontPageButtonShape,
        elevation = ButtonDefaults.elevation(defaultElevation = 7.dp),
        modifier = modifier.size(FrontPageExtraButtonsSize.dp),
        contentPadding = PaddingValues(1.dp)
    ) {
        Column(
            horizontalAlignment = Alignment.CenterHorizontally,
        ) {
            Image(
                painter = painterResource(id = imageId),
                contentDescription = stringResource(id = contentDescription),
                modifier = Modifier.size(FrontPageExtraImageSize.dp)
            )
            VerticalSpacer(space = 8)
            var fontSize by rememberSaveable { mutableStateOf(11f) }
            Text(
                text = stringResource(id = text),
                fontSize = fontSize.sp,
                fontFamily = FontFamily.Serif,
                fontWeight = FontWeight(600),
                maxLines = 1,
                onTextLayout = {
                    //Log.e("overeflow", "${it.size} ${it.didOverflowHeight}")
                    if (it.didOverflowHeight) fontSize *= 0.98f
                },
            )
        }
    }
}