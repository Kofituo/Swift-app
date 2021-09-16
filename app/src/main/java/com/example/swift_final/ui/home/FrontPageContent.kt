package com.example.swift_final.ui.home

import android.util.Log
import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.example.swift_final.R
import com.example.swift_final.util.DisplayUtils
import com.example.swift_final.util.DisplayUtils.ScreenPixels.Companion.widthInDp
import com.example.swift_final.util.HorizontalSpacer
import androidx.lifecycle.viewmodel.compose.viewModel
import com.example.swift_final.ui.*
import com.example.swift_final.util.VerticalSpacer
import com.example.swift_final.util.textFieldBorder

@Composable
fun FrontPageContent(paddingValues: PaddingValues) {
    //Log.e("padding Values", "$paddingValues")
    Column(
        modifier = Modifier
            .padding(paddingValues)
            .fillMaxSize()
            .verticalScroll(rememberScrollState()),
        horizontalAlignment = Alignment.CenterHorizontally
    ) {
        Spacer(modifier = Modifier.height(20.dp))
        val dialogViewModel: DialogViewModel = viewModel()
        val showDialog by dialogViewModel.dialogShowing.observeAsState(false)
        // Create alert dialog, pass the showDialog state to this Composable
        OnAddUrlClicked(showDialog = showDialog, setShowDialog = dialogViewModel.setShowDialog)
        val downloadInfoViewModel = viewModel<DownloadInfoViewModel>()
        val showDownloadInfoViewDialog by downloadInfoViewModel.dialogShowing.observeAsState(initial = false)
        //Log.e("infffffffffff","showwwwwww $showDownloadInfoViewDialog")
        DownloadInfoDialog(
            downloadInfoViewModel.downloadInfo,
            showDialog = showDownloadInfoViewDialog,
            setShowDialog = downloadInfoViewModel.setShowDialog
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