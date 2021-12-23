package com.example.swift_final.util

/**
 * Performs a panicking cast
 * */
@Suppress("UNCHECKED_CAST")
inline fun <T, C> T.cast() = this as C

inline fun <K, V> Map<out K, V>.forEachEntry(action: (K, V) -> Unit) {
    for ((key, value) in this) action(key, value)
}