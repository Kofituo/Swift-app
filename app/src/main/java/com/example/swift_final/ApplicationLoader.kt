package com.example.swift_final

import android.app.Application
import android.content.Context
import com.example.swift_final.lib.Logger

//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
class ApplicationLoader:Application() {
    companion object {
        private var _applicationContext: Context? = null
        val applicationContext get() = requireNotNull(_applicationContext)
    }
    override fun onCreate() {
        _applicationContext = applicationContext
        super.onCreate()
        System.loadLibrary("downloader_lib")
        Logger.initialiseLogging()
    }
}