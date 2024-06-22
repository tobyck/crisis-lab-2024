package com.example.crisislab

import NotificationHandler
import android.content.Context
import android.content.Intent
import android.os.Build
import android.util.Log
import android.widget.Toast
import androidx.annotation.RequiresApi
import okhttp3.Response
import okhttp3.WebSocket
import okhttp3.WebSocketListener

import kotlinx.serialization.*
import kotlinx.serialization.json.Json
import com.example.crisislab.databinding.ActivityMainBinding
import com.example.crisislab.LogViewModel
import org.json.JSONObject
import java.sql.Timestamp
import java.time.*
import java.time.format.DateTimeFormatter

import kotlin.math.round

class WebSocketListener(logViewModel: LogViewModel, socketStatusViewModel: SocketStatusViewModel, context: MainActivity) : WebSocketListener() {
    var logViewModel: LogViewModel = logViewModel;
    var socketStatusViewModel: SocketStatusViewModel = socketStatusViewModel;
    var context: MainActivity = context;

    @RequiresApi(Build.VERSION_CODES.TIRAMISU)
    override fun onOpen(webSocket: WebSocket, response: Response) {
        Log.d("test", "Connected")

        context.notificationHandler = NotificationHandler(context.notificationModule.provideNotificationBuilder(context), context.notificationModule.provideNotificationManager(context), context)
        if(!context.notificationHandler.isServiceRunning) {
            Log.d("ANwd", "service not running")
            val intent = Intent(context, context.notificationHandler::class.java)
            context.startService(intent);
            context.notificationHandler.isServiceRunning = true;
            Log.d("WebSocket", "Notification Service began.")
        }

        socketStatusViewModel.updateStatus("Status: Connected.")
    }

    @RequiresApi(Build.VERSION_CODES.TIRAMISU)
    override fun onMessage(webSocket: WebSocket, text: String) {
        //Log.d("test", "Received : $text")
        val packetList = ArrayList<HashMap<String, String?>>()
        val jObj = JSONObject(text)
        if (jObj.names()[0] == "previous_data") {
            // TODO: Handle previous data
            return;
        }
        for (i in 0 until jObj.length()) {
            val packet = HashMap<String, String?>()
            packet["pressure"] = jObj.optString("pressure")
            packet["height"] = jObj.optString("height")
            packet["timestamp"] = jObj.optString("timestamp")
            //packet["previous_data"]  = jObj.optJSONArray("previous_data")?.toString()

            // Alert
            if (packet["pressure"] == "" && packet["height"] != "") {
                val newLog = packet["height"]?.let { LogItem((round((it.toFloat()*10))/10).toString()+" cm", packet["timestamp"]) }

                if (newLog != null) {
                    logViewModel.addLogItem(newLog)

                    val inst = Instant.ofEpochMilli(newLog.time!!.toLong())
                    val instzdt = inst.atZone(ZoneId.of("Pacific/Auckland"))
                    val formattedinstzdt = DateTimeFormatter.ofPattern("kk:mm - dd/MM/yy - z").format(instzdt)

                    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
                        context.notificationHandler.showNotification("TSUNAMI WARNING", newLog.height, "TSUNAMI", formattedinstzdt);
                    }

                    return;
                }
            }
            packetList.add(packet)
        }
    }

    override fun onClosing(webSocket: WebSocket, code: Int, reason: String) {
        webSocket.close(NORMAL_CLOSURE_STATUS, null)
        output("Closing : $code / $reason")
        socketStatusViewModel.updateStatus("Status: Disconnected.")
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