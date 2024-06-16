package com.example.crisislab

import android.util.Log
import android.widget.Toast
import okhttp3.Response
import okhttp3.WebSocket
import okhttp3.WebSocketListener
import org.json.JSONObject
import java.util.ArrayList
import java.util.HashMap
import com.example.crisislab.databinding.ActivityMainBinding
import com.example.crisislab.LogViewModel


class WebSocketListener : WebSocketListener() {
    override fun onOpen(webSocket: WebSocket, response: Response) {
        webSocket.send("Hello World!")
        Log.e("burak","baglandi")
    }

    override fun onMessage(webSocket: WebSocket, text: String) {
        output("Received : $text")
        val packetList = ArrayList<HashMap<String, String?>>()
        val jObj = JSONObject(text)
        val jsonArry = jObj.getJSONArray("users")
        for (i in 0 until jsonArry.length()) {
            val packet = HashMap<String, String?>()
            val obj = jsonArry.getJSONObject(i)
            packet["height"] = obj.getString("height")
            packet["time"] = obj.getString("time")
            packet["trigger_alert"] = obj.getString("trigger_alert")
            if(packet["trigger_alert"] == "true") {
                val newLog = packet["height"]?.let { LogItem(it, packet["time"]) }
                if (newLog != null) {
                    LogViewModel.addLogItem(newLog)
                }
            }
            packetList.add(packet)
        }
    }


    override fun onClosing(webSocket: WebSocket, code: Int, reason: String) {
        webSocket.close(NORMAL_CLOSURE_STATUS, null)
        output("Closing : $code / $reason")
    }


    override fun onFailure(webSocket: WebSocket, t: Throwable, response: Response?) {
        output("Error : " + t.message+"fsda")
    }

    fun output(text: String?) {
        Log.d("PieSocket", text!!)
    }

    companion object {
        private const val NORMAL_CLOSURE_STATUS = 1000
    }
}