package com.example.crisislab

import android.os.Bundle
import android.util.Log
import android.widget.TextView
import androidx.activity.ComponentActivity
import androidx.lifecycle.ViewModelProvider
import androidx.recyclerview.widget.LinearLayoutManager
import com.example.crisislab.databinding.ActivityMainBinding
import org.json.JSONException
import org.json.JSONObject

private const val TAG = "MainActivity"
private const val INITIAL_STATUS = false
class MainActivity : ComponentActivity(), WebSocketListener {
    private lateinit var relayStatus: TextView
    private lateinit var sensorStatus: TextView
    private lateinit var alertsStatus: TextView
    private lateinit var logViewModel: LogViewModel
    private lateinit var binding: ActivityMainBinding

    private val webSocketClient = WebSocketClient("ws://170.64.254.27:8080")


    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
        relayStatus = findViewById(R.id.tvRelayStatus)
        sensorStatus = findViewById(R.id.tvSensorStatus)
        alertsStatus = findViewById(R.id.tvAlertsStatus)
        initialStatusUpdate(INITIAL_STATUS)
        webSocketClient.connect(this@MainActivity)
        binding = ActivityMainBinding.inflate(layoutInflater)
        setContentView(binding.root)
        logViewModel = ViewModelProvider(this).get(LogViewModel::class.java)
        setRecylerView()
    }

    override fun onConnected() {

    }

    override fun onMessage(message: String) {
        try {

            val packetList = ArrayList<HashMap<String, String?>>()
            val jObj = JSONObject(message)
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
                        logViewModel.addLogItem(newLog)
                    }
                }
                packetList.add(packet)
            }
        } catch (ex: JSONException) {
            Log.e("JsonParser Example", "unexpected JSON exception", ex)
        }



    }

    override fun onDisconnected() {
        // Handle disconnection
    }


    private fun setRecylerView() {
        logViewModel.logItems.observe(this){
            binding.logListRecyclerView.apply {
                layoutManager = LinearLayoutManager(applicationContext)
                adapter = it?.let { it1 -> LogItemAdapter(it1) }
            }
        }
    }

    private fun initialStatusUpdate(INITIAL_STATUS: Boolean) {
        if (!INITIAL_STATUS){
            relayStatus.text = "offline"
            sensorStatus.text = "offline"
            alertsStatus.text = "offline"
        } else {
            relayStatus.text = "online"
            sensorStatus.text = "online"
            alertsStatus.text = "online"
        }
    }

    private fun statusUpdate(newRelayStatus: Boolean, newSensorStatus: Boolean, newAlertsStatus: Boolean){
        if (newRelayStatus){
            relayStatus.text = "online"
        } else{
            relayStatus.text = "offline"
        }
        if (newSensorStatus){
            sensorStatus.text = "online"
        } else{
            sensorStatus.text = "offline"
        }
        if (newAlertsStatus){
            alertsStatus.text = "online"
        } else{
            alertsStatus.text = "offline"
        }
    }


}
