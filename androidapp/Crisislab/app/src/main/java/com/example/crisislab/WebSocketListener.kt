package com.example.crisislab

interface WebSocketListener {
    fun onConnected()
    fun onMessage(message: String)
    fun onDisconnected()
}
