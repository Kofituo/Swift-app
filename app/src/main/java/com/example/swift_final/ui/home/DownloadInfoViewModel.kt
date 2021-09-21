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
    val setShowDialog = { shouldShow: Boolean, downloadInfo: DownloadInfo? ->
        _downloadInfo = downloadInfo //set before calling compose function
        _dialogShowing.value = shouldShow
    }

    companion object {
        fun newDownloadInfo(
            url: String,
            useAuth: Boolean,
            username: String = "",
            password: String? = null
        ) =
            if (useAuth) DownloadInfo(url, Authentication(username, password))
            else DownloadInfo(url, null)
    }

    /*data class DownloadInfo(val url: String, val authorisation: Authorisation? = null) {
        companion object {
            fun new(
                url: String,
                useAuth: Boolean,
                username: String = "",
                password: String? = null
            ) =
                if (useAuth) DownloadInfo(url, Authorisation(username, password))
                else DownloadInfo(url)
        }
    }

    data class Authorisation(val username: String, val password: String?)*/
}