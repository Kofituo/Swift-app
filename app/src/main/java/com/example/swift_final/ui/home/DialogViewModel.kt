package com.example.swift_final.ui.home

import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel

class DialogViewModel : ViewModel() {

    private val _dialogShowing = MutableLiveData(false)
    val dialogShowing: LiveData<Boolean> get() = _dialogShowing
    inline val dialogIsShowing get() = dialogShowing.value ?: false
    var isModified = false

    val setShowDialog = { shouldShow: Boolean ->
        _dialogShowing.value = shouldShow
        //dialog is dismissed so reset
        if (!shouldShow) {
            setIsError(false)
            _url.value = null
            isModified = false
        }
    }

    private var _url: MutableLiveData<String?> = MutableLiveData()
    val url get() = _url.value ?: ""
    val urlLiveData: LiveData<String?> get() = _url
    fun setUrl(url: String) {
        this._url.value = url
    }

    private val _isError = MutableLiveData(false)
    val isError: LiveData<Boolean> get() = _isError
    fun setIsError(isError: Boolean) {
        _isError.value = isError
    }
}