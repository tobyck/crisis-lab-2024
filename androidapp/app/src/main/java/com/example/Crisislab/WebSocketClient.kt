package com.example.Crisislab

import io.ktor.client.HttpClient
import io.ktor.client.plugins.websocket.*
import kotlinx.coroutines.GlobalScope
import kotlinx.coroutines.launch
import io.ktor.websocket.*

class WebSocketClient(private val url: String) {

    private val client = HttpClient {
        install(WebSockets)
    }

    fun connect(listener: WebSocketListener) {
        GlobalScope.launch {
            client.wss(url) {
                listener.onConnected()
                try {
                    for (frame in incoming) {
                        if (frame is Frame.Text) {
                            listener.onMessage(frame.readText())
                        }
                    }
                } catch (e: Exception) {
                    listener.onDisconnected()
                }
            }
        }
    }

    fun disconnect() {
        client.close()
    }
}
