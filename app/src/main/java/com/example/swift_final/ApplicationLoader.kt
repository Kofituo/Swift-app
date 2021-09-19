package com.example.swift_final

import android.app.Application
import android.content.Context

//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
class ApplicationLoader:Application() {
    companion object {
        private var _applicationContext: Context? = null
        val applicationContext get() = requireNotNull(_applicationContext)
    }
    override fun onCreate() {
        _applicationContext = applicationContext
        super.onCreate()
    }
}