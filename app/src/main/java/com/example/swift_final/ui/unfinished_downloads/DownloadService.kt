package com.example.swift_final.ui.unfinished_downloads

import android.app.Service
import android.content.Intent
import android.os.*
import android.util.Log
import com.example.swift_final.ApplicationLoader
import com.example.swift_final.ui.home.DownloadIntentData

class DownloadService : Service() {

    companion object {
        const val DownloadInfo = "${ApplicationLoader.APP_NAME}/downloadInfo"
    }

    private var serviceLooper: Looper? = null
    private var serviceHandler: ServiceHandler? = null

    private inner class ServiceHandler(looper: Looper) : Handler(looper) {
        override fun handleMessage(msg: Message) {

        }
    }

    inner class DownloadBinder : Binder() {
        val service get() = this@DownloadService
    }

    private val downloadBinder = DownloadBinder()

    override fun onBind(intent: Intent): IBinder {
        return downloadBinder
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        Log.e("service", "starting")
        serviceHandler?.obtainMessage()?.also {
            it.arg1 = startId
            intent?.getParcelableExtra<DownloadIntentData>(DownloadInfo)?.let { data ->
                it.obj = data
            }
            serviceHandler?.sendMessage(it)
        }
        return START_REDELIVER_INTENT
    }

    override fun onCreate() {
        HandlerThread("DownloadThread", Process.THREAD_PRIORITY_BACKGROUND).apply {
            start()
            serviceHandler = ServiceHandler(looper)
            serviceLooper = looper
        }
    }

    override fun onDestroy() {
        Log.e("service", "stopped")
    }
}