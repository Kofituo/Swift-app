package com.example.swift_final.ui.unfinished_downloads

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.sp
import androidx.constraintlayout.compose.ConstraintLayout
import androidx.navigation.NavController
import com.example.swift_final.R
import com.example.swift_final.lib.FileCategory
import com.example.swift_final.ui.home.DownloadsTopBar
import com.example.swift_final.util.SharedPreferencesConstants
import com.example.swift_final.util.get
import com.example.swift_final.util.sharedPreferences
import kotlinx.serialization.ExperimentalSerializationApi
import kotlinx.serialization.Serializable
import kotlinx.serialization.decodeFromString
import kotlinx.serialization.json.Json

@Composable
fun UnfinishedPage(navController: NavController) {
    Column {
        DownloadsTopBar(title = R.string.unfinished, navController = navController)
        //retrieve the list from shared preferences if any
        val downloadList = downloadsList()
        if (downloadList.values.all { it.isNullOrEmpty() }) {
            // no downloads yet or all downloads completed
            // check if no download at all
            val hasPreviousDownloads =
                sharedPreferences(key = SharedPreferencesConstants.UnfinishedDownloads)
                    .get<Boolean>(SharedPreferencesConstants.UnfinishedDownloads)
            // create layout
            val imageId: Int
            val text: String
            if (hasPreviousDownloads) {
                imageId = R.drawable.ic_ok
                text = stringResource(id = R.string.no_uncompleted_downloads)
            } else {
                imageId = R.drawable.ic_sad
                text = stringResource(id = R.string.no_downloads)
            }
            ConstraintLayout(modifier = Modifier.fillMaxSize()) {
                val (image, message) = createRefs()
                Image(
                    painter = painterResource(id = imageId),
                    contentDescription = text,
                    modifier = Modifier.constrainAs(image) {
                        linkTo(parent.top, parent.bottom, bias = 0.37f)
                        start.linkTo(parent.start)
                        end.linkTo(parent.end)
                    })
                Text(text = text, fontSize = 18.sp, modifier = Modifier.constrainAs(message) {
                    end.linkTo(parent.end)
                    start.linkTo(parent.start)
                    top.linkTo(image.bottom)
                    linkTo(top = image.bottom, bottom = parent.bottom, bias = 0.03f)
                })
            }
        } else {
            //there's some downloading
            TODO()
        }
    }
}

@OptIn(ExperimentalSerializationApi::class)
@Composable
private fun downloadsList(): Map<FileCategory, MutableList<UnfinishedDownloadData>?> {
    val sharedPreferences =
        sharedPreferences(key = SharedPreferencesConstants.UnfinishedDownloads)

    return remember {
        FileCategory.values().associate { value ->
            value to sharedPreferences.get<String?>(value.name)?.let {
                //it should be a list of `UnfinishedDownloadData`
                Json.decodeFromString<MutableList<UnfinishedDownloadData>>(it)
            }
        }
    }
}

@Serializable
private data class UnfinishedDownloadData(
    val filename: String,
    /**
     * Total download size if any (not the amount downloaded)
     * */
    val fileSize: Int?,
    val fileCategory: FileCategory,
    var ongoing: Boolean,
    val timeStamp: Long
)