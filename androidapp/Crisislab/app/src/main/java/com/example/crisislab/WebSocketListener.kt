package com.example.crisislab

import android.util.Log
import android.widget.Toast
import okhttp3.Response
import okhttp3.WebSocket
import okhttp3.WebSocketListener

import kotlinx.serialization.*
import kotlinx.serialization.json.Json
import com.example.crisislab.databinding.ActivityMainBinding
import com.example.crisislab.LogViewModel
import org.json.JSONObject
import java.sql.Timestamp

//@Serializable
//data class DataPacket(
//    val pressure: Int,
//    val height: Int,
//    val waveform: String,
//    val timestamp: String
//)

class WebSocketListener : WebSocketListener() {
    override fun onOpen(webSocket: WebSocket, response: Response) {
        Log.d("test", "Connected")
    }

    override fun onMessage(webSocket: WebSocket, text: String) {
        //Log.d("test", "Received : $text")
        val packetList = ArrayList<HashMap<String, String?>>()
        val jObj = JSONObject(text)
        if (jObj.names()[0] == "previous_data") {
            // TODO: Handle previous data
            return;
        }
        //val jsonArry = jObj.getJSONArray(text)
        //Log.d("ugaw", jObj.toString());
        for (i in 0 until jObj.length()) {
            val packet = HashMap<String, String?>()
            //Log.d("auw", jObj.toString());
            //val obj = jObj.getJSONObject(i)
            //Log.d("awdku", jObj.toString());
            packet["pressure"] = jObj.optString("pressure")
            packet["height"] = jObj.optString("height")
            packet["timestamp"] = jObj.optString("timestamp")
            //packet["previous_data"]  = jObj.optJSONArray("previous_data")?.toString()
            if (packet["pressure"] == "" && packet["height"] != "") {
                val newLog = packet["height"]?.let { LogItem(it, packet["timestamp"]) }
                if (newLog != null) {
                    LogViewModel.addLogItem(newLog)
                    return;
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
        Log.e("WebSocket", "Error : " + t.message)
    }

    fun output(text: String?) {
        Log.d("PieSocket", text!!)
    }

    companion object {
        private const val NORMAL_CLOSURE_STATUS = 1000
    }
}