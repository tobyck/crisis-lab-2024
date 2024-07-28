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

// Listens for WebSocket events and processes messages accordingly
class WebSocketListener(
    logViewModel: LogViewModel,
    socketStatusViewModel: SocketStatusViewModel,
    context: MainActivity
) : WebSocketListener() {

    // ViewModel to manage logs
    var logViewModel: LogViewModel = logViewModel
    // ViewModel to manage socket status
    var socketStatusViewModel: SocketStatusViewModel = socketStatusViewModel
    // Context of the main activity
    var context: MainActivity = context

    // Called when the WebSocket connection is opened
    @RequiresApi(Build.VERSION_CODES.TIRAMISU)
    override fun onOpen(webSocket: WebSocket, response: Response) {
        // Initialize the notification handler
        context.notificationHandler = NotificationHandler(
            context.notificationModule.provideNotificationManager(context),
            context
        )

        // Start the notification service if it's not running
        if(!context.notificationHandler.isServiceRunning) {
            output("Service not running")
            val intent = Intent(context, context.notificationHandler::class.java)
            context.startService(intent)
            context.notificationHandler.isServiceRunning = true
            output("Notification Service began.")
        }

        socketStatusViewModel.updateStatus("Status: Connected.")
    }

    // Called when a message is received from the WebSocket
    @RequiresApi(Build.VERSION_CODES.TIRAMISU)
    override fun onMessage(webSocket: WebSocket, text: String) {
        val data = JSONObject(text)
        val prevAlerts = data.optJSONArray("previous_alerts")

        // Process previous alerts if any
        if(prevAlerts != null) {
            for(i in 0 until prevAlerts.length()) {
                val prevPacket: JSONObject = prevAlerts.getJSONObject(i)
                val currentPacket = checkPacket(prevPacket)

                if(currentPacket.bool) {
                    logPacket(currentPacket.packet, false)
                }
            }
        }

        // Check and log the current packet
        val checked = checkPacket(data)
        if (checked.bool) {
            logPacket(checked.packet, true)
        }
    }

    // Data class to hold packet information
    data class Packet(val bool: Boolean, val packet: HashMap<String, String?>)

    // Log the packet and show notification if required
    @RequiresApi(Build.VERSION_CODES.O)
    fun logPacket(packet: HashMap<String, String?>, notify: Boolean) {
        // Format the log
        val newLog = packet["height"]?.let {
            LogItem((round((it.toFloat() * 100)) / 100).toString() + " cm", packet["timestamp"])
        }

        // Add log item and show notification if necessary
        if (newLog != null) {
            logViewModel.addLogItem(newLog)

            // Format timestamp
            val time = Instant.ofEpochMilli(newLog.time!!.toLong())
            val zonedTime = time.atZone(ZoneId.of("Pacific/Auckland"))
            val formattedTime = DateTimeFormatter.ofPattern("kk:mm - dd/MM/yy - z").format(zonedTime)

            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU && notify) {
                context.notificationHandler.showNotification(
                    "TSUNAMI WARNING",
                    newLog.height,
                    "TSUNAMI",
                    formattedTime
                )
            }
            return
        }
    }

    // Check the packet data and determine its validity
    fun checkPacket(data: JSONObject?): Packet {
        val jsonData = HashMap<String, String?>()

        // Extract data from JSON
        jsonData["pressure"] = data?.optString("pressure")
        jsonData["height"] = data?.optString("height")
        jsonData["timestamp"] = data?.optString("timestamp")

        // Check if the packet is an alert (no pressure value)
        if (jsonData["pressure"] == "" && jsonData["height"] != "") {
            return Packet(true, jsonData)
        }
        return Packet(false, jsonData)
    }

    // Called when the WebSocket is closing
    override fun onClosing(webSocket: WebSocket, code: Int, reason: String) {
        webSocket.close(NORMAL_CLOSURE_STATUS, null)
        output("Closing : $code / $reason")
        socketStatusViewModel.updateStatus("Status: Disconnected.")
    }

    // Called when there's a failure in the WebSocket connection
    override fun onFailure(webSocket: WebSocket, t: Throwable, response: Response?) {
        Log.e("WebSocket", "Error : " + t.message)
    }

    // Utility function to output logs
    fun output(text: String?) {
        Log.d("WebSocket", text!!)
    }

    companion object {
        // Normal closure status code
        private const val NORMAL_CLOSURE_STATUS = 1000
    }
}
