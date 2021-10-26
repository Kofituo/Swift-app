package com.example.swift_final

import android.app.Application
import android.content.Context
import com.example.swift_final.lib.Logger

//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
class ApplicationLoader:Application() {
    companion object {
        private var mApplicationContext: Context? = null
        val applicationContext get() = requireNotNull(mApplicationContext)
    }
    override fun onCreate() {
        mApplicationContext = applicationContext
        System.loadLibrary("downloader_lib")
        super.onCreate()
        Logger.initialiseLogging()
    }
}