package com.example.swift_final.ui.home

import android.net.ConnectivityManager
import android.net.Network
import android.net.NetworkRequest
import androidx.core.content.getSystemService
import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.example.swift_final.ApplicationLoader
import com.example.swift_final.lib.Authentication
import com.example.swift_final.lib.DownloadInfo
import kotlinx.coroutines.launch

class DownloadInfoViewModel : ViewModel() {
    private val _dialogShowing = MutableLiveData(false)
    val dialogShowing: LiveData<Boolean> get() = _dialogShowing
    private var _downloadInfo: DownloadInfo? = null
    val downloadInfo get() = _downloadInfo //since it can't be null when download information is opened
    val setShowDialog = { downloadInfo: DownloadInfo? ->
        _downloadInfo = downloadInfo //set before calling compose function
        _dialogShowing.value = downloadInfo != null
        if (downloadInfo == null) { //means we're closing the dialog
            _sendRequest.value = true
            success = false
        }
    }
    private val _sendRequest: MutableLiveData<Boolean> = MutableLiveData(true)
    val sendRequest: LiveData<Boolean> get() = _sendRequest

    val setSendRequest = { sendRequest: Boolean ->
        _sendRequest.value = sendRequest
    }

    private var success = false
    val setSuccess = {
        success = true
    }

    private val networkCallback: ConnectivityManager.NetworkCallback

    init {
        networkCallback = object : ConnectivityManager.NetworkCallback() {
            override fun onAvailable(network: Network) {
                if (!success && dialogShowing.value == true) {
                    //retry
                    viewModelScope.launch {
                        _sendRequest.value = true
                    }
                }
            }

            override fun onLost(network: Network) {
            }
        }

        ApplicationLoader.applicationContext.getSystemService<ConnectivityManager>()?.run {
            if (android.os.Build.VERSION.SDK_INT >= android.os.Build.VERSION_CODES.N)
                registerDefaultNetworkCallback(networkCallback)
            else registerNetworkCallback(NetworkRequest.Builder().build(), networkCallback)
        }
    }

    override fun onCleared() {
        ApplicationLoader.applicationContext.getSystemService<ConnectivityManager>()
            ?.unregisterNetworkCallback(networkCallback)
        super.onCleared()
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