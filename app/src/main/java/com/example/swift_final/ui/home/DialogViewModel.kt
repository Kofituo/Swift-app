package com.example.swift_final.ui.home

import android.util.Log
import androidx.compose.runtime.Composable
import androidx.compose.ui.text.input.TextFieldValue
import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewmodel.compose.viewModel
import com.example.swift_final.util.isUrl

class DialogViewModel : ViewModel() {

    private val _dialogShowing = MutableLiveData(false)
    val dialogShowing: LiveData<Boolean> get() = _dialogShowing
    val setShowDialog = { shouldShow: Boolean ->
        _dialogShowing.value = shouldShow
        //dialog is dismissed so reset
        if (!shouldShow) {
            setIsError(false)
            autoModifyInSection = true
            _initialText.value = null
            Log.e("trueeeeeeee", "$autoModifyInSection ${initialText.value}")
        }
    }

    private var _initialText: MutableLiveData<TextFieldValue?> = MutableLiveData()
    var autoModifyInSection = true
    val initialText: LiveData<TextFieldValue?> get() = _initialText
    fun setInitialText(textFieldValue: TextFieldValue) {
        _initialText.value = textFieldValue
    }

    private val _isError = MutableLiveData(false)
    val isError:LiveData<Boolean> get() = _isError
    fun setIsError(isError: Boolean) {
        _isError.value = isError
    }
}