package com.example.swift_final.ui.unfinished_downloads

import android.os.Parcelable
import android.util.Log
import androidx.annotation.MainThread
import androidx.compose.foundation.ExperimentalFoundationApi
import androidx.compose.foundation.Image
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.Icon
import androidx.compose.material.LinearProgressIndicator
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ExpandLess
import androidx.compose.material.icons.filled.ExpandMore
import androidx.compose.runtime.*
import androidx.compose.runtime.livedata.observeAsState
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalLifecycleOwner
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.constraintlayout.compose.ConstraintLayout
import androidx.constraintlayout.compose.Dimension
import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavController
import com.example.swift_final.ApplicationLoader
import com.example.swift_final.R
import com.example.swift_final.Screens
import com.example.swift_final.lib.FileCategory
import com.example.swift_final.ui.home.DownloadsTopBar
import com.example.swift_final.util.*
import com.example.swift_final.util.DisplayUtils.ScreenPixels.Companion.widthInDp
import kotlinx.parcelize.Parcelize
import kotlinx.serialization.ExperimentalSerializationApi
import kotlinx.serialization.Serializable
import kotlinx.serialization.decodeFromString
import kotlinx.serialization.json.Json
import java.io.File

@Composable
fun UnfinishedPage(
    navController: NavController,
    // optional arguments to which this function can be called with
    filename: String?,
    category: String?,
    fileSize: Long
) {
    Column {
        DownloadsTopBar(title = R.string.unfinished, navController = navController)
        //retrieve the list from shared preferences if any
        val downloadList = rememberSaveable { downloadsList() }
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
            Log.e("list", "$downloadList")
            val unfinishedDownloadVieModel = viewModel<UnfinishedDownloadVieModel>()
            unfinishedDownloadVieModel.setSortMap(downloadList)
            DisposableEffect(key1 = LocalLifecycleOwner.current) {
                onDispose {
                    unfinishedDownloadVieModel.clear()
                }
            }
            UnfinishedList(viewModel = unfinishedDownloadVieModel)
        }
    }
}

@OptIn(ExperimentalFoundationApi::class)
@Composable
fun UnfinishedList(viewModel: UnfinishedDownloadVieModel) {
    val list by viewModel.downloadListLiveData.observeAsState()
    if (list == null) // means we're resigning the list so do nothing
        return
    LazyColumn(modifier = Modifier.fillMaxSize()) {
        val downloadList = requireNotNull(list)
        downloadList.forEachEntry { fileCategory, mutableList ->
            stickyHeader {
                Header(
                    fileCategory = fileCategory,
                    onCategoryClicked = { category, isOpened ->
                        // TODO animate between states
                        if (isOpened) viewModel.addCategory(category)
                        else viewModel.removeCategoryValues(fileCategory = fileCategory)
                    }
                )
            }
            items(mutableList) { item ->
                CategoryListItem(item = item)
            }
        }
    }
}

fun categoryFolder(fileCategory: FileCategory) =
    File(ApplicationLoader.applicationContext.filesDir, fileCategory.name)

@Composable
fun CategoryListItem(item: UnfinishedDownloadData) {
    // use current file size (if it exists) as progress value
    val progressInBytes: MutableState<Long>? =
        if (item.fileSize == 0L) null
        else rememberSaveable {
            mutableStateOf(
                runCatching {
                    File(categoryFolder(item.fileCategory), item.filename).length()
                }.getOrElse { 0 }
            )
        }
    Log.e("const", "layout")
    ConstraintLayout(modifier = Modifier.fillMaxSize()) {
        val (filename, progressBar, icon, speed, progressText) = createRefs()
        Text(text = item.filename,
            maxLines = 1,
            modifier = Modifier.constrainAs(filename) {
                top.linkTo(parent.top)
                linkTo(start = parent.start, end = icon.start, bias = 0f)
                bottom.linkTo(progressBar.top)
                width = Dimension.percent(0.8f)
            }
        )
        //TODO add animation
        // https://developer.android.com/reference/kotlin/androidx/compose/material/package-summary?hl=pt#LinearProgressIndicator(kotlin.Float,androidx.compose.ui.Modifier,androidx.compose.ui.graphics.Color,androidx.compose.ui.graphics.Color)
        val progressModifier = Modifier.constrainAs(progressBar) {
            top.linkTo(filename.bottom)
            linkTo(start = parent.start, end = icon.start, bias = 0f)
            bottom.linkTo(parent.bottom)
            width = Dimension.percent(0.8f)
        }
        val progressBytes = progressInBytes?.value?.toDouble()
        progressBytes?.let {
            LinearProgressIndicator(
                modifier = progressModifier,
                progress = ((it / item.fileSize) * 100).toFloat()
            )
        } ?: LinearProgressIndicator(modifier = progressModifier)
        // TODO add error button
    }
}

val FileCategory.drawableId
    get() = when (this) {
        FileCategory.Video -> R.drawable.ic_video
        FileCategory.Document -> R.drawable.ic_document
        FileCategory.Image -> R.drawable.ic_image
        FileCategory.Compressed -> R.drawable.ic_zip_c
        FileCategory.Audio -> R.drawable.ic_audio
        FileCategory.Application -> R.drawable.ic_app
        FileCategory.Other -> R.drawable.ic_other
    }

@Composable
fun Header(fileCategory: FileCategory, onCategoryClicked: (FileCategory, Boolean) -> Unit) {
    var isOpened by rememberSaveable { mutableStateOf(true) }
    ConstraintLayout(
        modifier =
        Modifier
            .fillMaxSize()
            .clickable {
                isOpened = !isOpened
                // no need to pass category here... it just feels right to
                onCategoryClicked(fileCategory, isOpened)
            }
            .padding(top = 4.dp)
    ) {
        val (image, title, icon) = createRefs()
        val startGuide = createGuidelineFromStart(0.02f)
        val endGuide = createGuidelineFromStart(0.95f)
        val imageBarrier = createEndBarrier(image)
        Image(
            painter = painterResource(id = fileCategory.drawableId),
            contentDescription = fileCategory.name,
            modifier = Modifier
                .constrainAs(image) {
                    start.linkTo(startGuide)
                    top.linkTo(parent.top)
                    bottom.linkTo(parent.bottom)
                }
                .padding(8.dp)
                .size(DisplayUtils.percentageOfScreenSize(8f).widthInDp)
        )

        Text(
            text = fileCategory.name,
            fontSize = 18.sp,
            modifier = Modifier
                .constrainAs(title) {
                    top.linkTo(parent.top)
                    bottom.linkTo(parent.bottom)
                    linkTo(start = image.end, end = icon.start, bias = 0f)
                    start.linkTo(imageBarrier)
                }
                .padding(14.dp)
        )
        Icon(
            imageVector = if (isOpened) Icons.Filled.ExpandLess else Icons.Filled.ExpandMore,
            contentDescription = fileCategory.name,
            modifier = Modifier.constrainAs(icon) {
                top.linkTo(parent.top)
                bottom.linkTo(parent.bottom)
                end.linkTo(endGuide)
            }
        )
    }
}

fun sortMap(downloadMap: Map<FileCategory, List<UnfinishedDownloadData>>):
        MutableMap<FileCategory, MutableList<UnfinishedDownloadData>> {
    // show the most recent download on top,
    // hence that category items would be on top
    // the next category would be the other one which is next to most recent
    // so basically:
    var time = 0L
    val categoryList = downloadMap.keys.toMutableList() //FileCategory.values().size
    downloadMap.values.flatten().forEach {
        val isMoreRecent = it.timeStamp > time
        time = it.timeStamp
        if (isMoreRecent) {
            categoryList.remove(it.fileCategory)
            categoryList.add(it.fileCategory)
        }
    }
    // reverse the list
    return categoryList.asReversed().associateWith { downloadMap.getValue(it) }.cast()
}


object UnfinishedPage {
    fun getIntentString(filename: String, category: String, fileSize: Long) =
        "${Screens.UnfinishedDownloads}?filename=$filename&category=$category&filesize=$fileSize"
}

@OptIn(ExperimentalSerializationApi::class)
private fun downloadsList(): MutableMap<FileCategory, MutableList<UnfinishedDownloadData>?> {
    val sharedPreferences =
        ApplicationLoader.applicationContext
            .sharedPreferences(key = SharedPreferencesConstants.UnfinishedDownloads)
    return FileCategory.values().associate { value ->
        value to sharedPreferences.get<String?>(value.name)?.let {
            //it should be a list of `UnfinishedDownloadData`
            Json.decodeFromString<MutableList<UnfinishedDownloadData>>(it)
        }
    }.cast()
}

@Serializable
@Parcelize
data class UnfinishedDownloadData(
    val filename: String,
    /**
     * Total download size if any (not the amount downloaded)
     * */
    val fileSize: Long,
    val fileCategory: FileCategory,
    val resumable: Boolean,
    var ongoing: Boolean,
    /**
     * Represents the most time when time when this download
     * resumed / started
     * */
    val timeStamp: Long
) : Parcelable
/******, Comparable<UnfinishedDownloadData> {

 */
/**
 * Compares this object with the specified object for order.
 * Returns zero if this object is equal to
 * the specified other object, a negative number if it's less than other,
 * or a positive number if it's greater than other.
 * *
override fun compareTo(other: UnfinishedDownloadData): Int {
var out = filename.compareTo(other.filename)
if (out == 0)
out =
}************/

class UnfinishedDownloadVieModel : ViewModel() {

    private var originalList: MutableMap<FileCategory, MutableList<UnfinishedDownloadData>>? = null
    private inline val mOriginalList get() = requireNotNull(originalList)

    fun setSortMap(downloadMap: MutableMap<FileCategory, MutableList<UnfinishedDownloadData>?>) {
        if (originalList != null) return
        Log.e("setting", "list")
        originalList = sortMap(downloadMap.filterNot { it.value.isNullOrEmpty() }
            .cast()).also { _downloadList.value = it.toMutableMap() }
    }

    private var _downloadList: MutableLiveData<MutableMap<FileCategory, List<UnfinishedDownloadData>>?> =
        MutableLiveData(null)

    val downloadListLiveData: LiveData<MutableMap<FileCategory, MutableList<UnfinishedDownloadData>>> // shouldn't be null
        get() = _downloadList.cast()

    fun clear() {
        Log.e("clearing", "now")
        originalList = null
        _downloadList.value = null
    }

    // called when the user deletes or download finish
    @MainThread
    fun updateList() {
        originalList =
            sortMap(downloadsList().filterNot { it.value.isNullOrEmpty() }.cast())
                .also { _downloadList.value = it.toMutableMap() }
    }
    //val add TODO

    /**
     * Throws an error if the category isn't in the map
     * */
    fun removeCategoryValues(fileCategory: FileCategory) {
        val temp = _downloadList.value ?: return
        _downloadList.value = null
        temp.let {
            require(fileCategory in it)
            it[fileCategory] = mutableListOf()
            _downloadList.value = it
        }
    }

    /**
     * Throws an error if the category isn't in the original map
     * map or this map already has a list in that category
     * */
    fun addCategory(fileCategory: FileCategory) {
        val temp = _downloadList.value ?: return
        _downloadList.value = null
        temp.let {
            val originalValue = mOriginalList[fileCategory]
            requireNotNull(originalValue)
            val previousValue = it[fileCategory]
             require(previousValue != null && previousValue.isEmpty())
            it[fileCategory] = originalValue
            _downloadList.value = it
        }
        onCleared()
    }
}