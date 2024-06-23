package com.example.crisislab

import NotificationHandler
import android.content.Intent
import android.os.Build
import android.util.Log
import androidx.annotation.RequiresApi
import okhttp3.Response
import okhttp3.WebSocket
import okhttp3.WebSocketListener
import org.json.JSONObject
import java.time.*
import java.time.format.DateTimeFormatter
import kotlin.math.round

class SocketListener(logViewModel: LogViewModel, socketStatusViewModel: SocketStatusViewModel, context: MainActivity) : WebSocketListener() {
    var logViewModel: LogViewModel = logViewModel;
    var socketStatusViewModel: SocketStatusViewModel = socketStatusViewModel;
    var context: MainActivity = context;

    @RequiresApi(Build.VERSION_CODES.TIRAMISU)
    override fun onOpen(webSocket: WebSocket, response: Response) {
        Log.d("test", "Connected")

        // JANKY JANK JANK
        context.notificationHandler = NotificationHandler(context.notificationModule.provideNotificationManager(context), context)
        if(!context.notificationHandler.isServiceRunning) {
            Log.d("WebSocket", "Service not running")
            val intent = Intent(context, context.notificationHandler::class.java)
            context.startService(intent);
            context.notificationHandler.isServiceRunning = true;
            Log.d("WebSocket", "Notification Service began.")
        }

        socketStatusViewModel.updateStatus("Status: Connected.")
    }

    @RequiresApi(Build.VERSION_CODES.TIRAMISU)
    override fun onMessage(webSocket: WebSocket, text: String) {
        val jObj = JSONObject(text)

        for (i in 0 until jObj.length()) {
            val packet = HashMap<String, String?>()
            packet["pressure"] = jObj.optString("pressure")
            packet["height"] = jObj.optString("height")
            packet["timestamp"] = jObj.optString("timestamp")

			// Alert packets don't contain a pressure value, only a height and timestamp
            if (packet["pressure"] == "" && packet["height"] != "") {
				// Make a LogItem with height rounded to 2dp
                val newLog = packet["height"]?.let { LogItem((round((it.toFloat() * 100)) / 100).toString() + " cm", packet["timestamp"]) }

                if (newLog != null) {
                    logViewModel.addLogItem(newLog)

                    // Format timestamp
                    val time = Instant.ofEpochMilli(newLog.time!!.toLong())
                    val zonedTime = time.atZone(ZoneId.of("Pacific/Auckland"))
                    val formattedTime = DateTimeFormatter.ofPattern("kk:mm - dd/MM/yy - z").format(zonedTime)

                    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
                        context.notificationHandler.showNotification("TSUNAMI WARNING", newLog.height, "TSUNAMI", formattedTime);
                    }

                    return;
                }
            }
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
