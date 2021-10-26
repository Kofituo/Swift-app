package com.example.swift_final.ui.home

import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import com.example.swift_final.lib.Authentication
import com.example.swift_final.lib.DownloadInfo

class DownloadInfoViewModel : ViewModel() {
    private val _dialogShowing = MutableLiveData(false)
    val dialogShowing: LiveData<Boolean> get() = _dialogShowing
    private var _downloadInfo: DownloadInfo? = null
    val downloadInfo get() = _downloadInfo //since it can't be null when download information is opened
    val setShowDialog = { downloadInfo: DownloadInfo? ->
        _downloadInfo = downloadInfo //set before calling compose function
        _dialogShowing.value = downloadInfo != null
    }

    companion object {
        fun newDownloadInfo(
            url: String,
            useAuth: Boolean,
            username: String = "",
            password: String? = null
        ) = DownloadInfo(url, if (useAuth) Authentication(username, password) else null)
    }
}