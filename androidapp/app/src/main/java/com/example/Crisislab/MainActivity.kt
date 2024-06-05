package com.example.Crisislab

import android.os.Bundle
import android.widget.TextView
import androidx.activity.ComponentActivity
import androidx.lifecycle.ViewModelProvider
import androidx.recyclerview.widget.LinearLayoutManager
import com.example.tippy2.R
import com.example.tippy2.databinding.ActivityMainBinding


private const val TAG = "MainActivity"
private const val INITIAL_STATUS = false
class MainActivity : ComponentActivity(), WebSocketListener {
    private lateinit var relayStatus: TextView
    private lateinit var sensorStatus: TextView
    private lateinit var alertsStatus: TextView
    private lateinit var logViewModel: LogViewModel
    private lateinit var binding: ActivityMainBinding

    private val webSocketClient = WebSocketClient("ws://serverip:8080/ws")


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
        // Handle received message
    }

    override fun onDisconnected() {
        // Handle disconnection
    }


    private fun setRecylerView() {
        val mainActivity = this
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
