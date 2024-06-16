package com.example.crisislab

import android.os.Bundle
import android.util.Log
import android.widget.TextView
import androidx.activity.ComponentActivity
import androidx.lifecycle.ViewModelProvider
import androidx.recyclerview.widget.LinearLayoutManager
import com.example.crisislab.databinding.ActivityMainBinding
import okhttp3.OkHttpClient
import okhttp3.Request
import okhttp3.WebSocket
import org.json.JSONObject
import java.util.ArrayList
import java.util.HashMap


private const val TAG = "MainActivity"
private const val INITIAL_STATUS = false
class MainActivity : ComponentActivity() {
    private lateinit var relayStatus: TextView
    private lateinit var sensorStatus: TextView
    private lateinit var alertsStatus: TextView
    private lateinit var logViewModel: LogViewModel
    private lateinit var binding:ActivityMainBinding


    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
        relayStatus = findViewById(R.id.tvRelayStatus)
        sensorStatus = findViewById(R.id.tvSensorStatus)
        alertsStatus = findViewById(R.id.tvAlertsStatus)
        initialStatusUpdate(INITIAL_STATUS)
        binding = ActivityMainBinding.inflate(layoutInflater)
        setContentView(binding.root)
        val client: OkHttpClient =  OkHttpClient()

        binding.connect.setOnClickListener {
            Log.d("PieSocket","Connecting");

            val request: Request = Request
                .Builder()
                .url("ws://170.64.254.27:8443")
                .build()
            val listener = WebSocketListener()
            val ws: WebSocket = client.newWebSocket(request, listener)
        }
        logViewModel = ViewModelProvider(this).get(LogViewModel::class.java)
        setRecylerView()
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
