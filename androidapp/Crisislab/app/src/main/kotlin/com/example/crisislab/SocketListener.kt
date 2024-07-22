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
        context.notificationHandler = NotificationHandler(context.notificationModule.provideNotificationManager(context), context)
        if(!context.notificationHandler.isServiceRunning) {
            output("Service not running")

            val intent = Intent(context, context.notificationHandler::class.java)
            context.startService(intent);
            context.notificationHandler.isServiceRunning = true;

            output("Notification Service began.")
        }

        socketStatusViewModel.updateStatus("Status: Connected.")
    }

    @RequiresApi(Build.VERSION_CODES.TIRAMISU)
    override fun onMessage(webSocket: WebSocket, text: String) {
        val data = JSONObject(text)
        val prevAlerts = data.optJSONArray("previous_alerts")

        if(prevAlerts!=null) {
            for(i in 0 until prevAlerts.length()) {
                val prevPacket: JSONObject = prevAlerts.getJSONObject(i)
                val currentPacket = checkPacket(prevPacket)

                if(currentPacket.bool) {
                    logPacket(currentPacket.packet, false)
                }
            }
        }

        val checked = checkPacket(data);

        if (checked.bool) {
            logPacket(checked.packet, true)
        }
    }

    data class packetType(val bool: Boolean, val packet: HashMap<String, String?>)

    @RequiresApi(Build.VERSION_CODES.O)
    fun logPacket(packet: HashMap<String, String?>, notify: Boolean) {
        // Format the log
        val newLog = packet["height"]?.let { LogItem((round((it.toFloat() * 100)) / 100).toString() + " cm", packet["timestamp"]) }

        if (newLog != null) {
            logViewModel.addLogItem(newLog)

            // Format timestamp
            val time = Instant.ofEpochMilli(newLog.time!!.toLong())
            val zonedTime = time.atZone(ZoneId.of("Pacific/Auckland"))
            val formattedTime =
                DateTimeFormatter.ofPattern("kk:mm - dd/MM/yy - z").format(zonedTime)

            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU && notify) {
                context.notificationHandler.showNotification(
                    "TSUNAMI WARNING",
                    newLog.height,
                    "TSUNAMI",
                    formattedTime
                );
            }
            return;
        }
    }

    fun checkPacket(data: JSONObject?): packetType {
        val packet = HashMap<String, String?>()

        packet["pressure"] = data?.optString("pressure")
        packet["height"] = data?.optString("height")
        packet["timestamp"] = data?.optString("timestamp")

        // Alert packets don't contain a pressure value, only a height and timestamp
        if (packet["pressure"] == "" && packet["height"] != "") {
            return packetType(true, packet)
        }
        return packetType(false, packet)
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
        Log.d("WebSocket", text!!)
    }

    companion object {
        private const val NORMAL_CLOSURE_STATUS = 1000
    }
}