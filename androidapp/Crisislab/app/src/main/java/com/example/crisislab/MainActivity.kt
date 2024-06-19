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

class MainActivity : ComponentActivity() {
    lateinit var logViewModel: LogViewModel
    private lateinit var binding:ActivityMainBinding

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
        binding = ActivityMainBinding.inflate(layoutInflater)
        setContentView(binding.root)
        val client: OkHttpClient =  OkHttpClient()
        logViewModel = ViewModelProvider(this).get(LogViewModel::class.java)

        binding.connect.setOnClickListener {
            Log.d("PieSocket","Connecting");

            val request: Request = Request
                .Builder()
                .url("ws://dashboard.alex-berry.net:8080")
                .build()
            val listener = WebSocketListener(logViewModel);
            val ws: WebSocket = client.newWebSocket(request, listener)
        }
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
}