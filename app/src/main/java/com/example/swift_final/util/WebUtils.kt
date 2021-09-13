package com.example.swift_final.util

import android.util.Patterns

inline val CharSequence.isUrl
    get() = Patterns.WEB_URL.matcher(this).matches()