package com.example.crisislab

import NotificationHandler
import android.Manifest
import android.app.Activity
import android.os.Build
import android.os.Bundle
import android.util.Log
import androidx.activity.ComponentActivity
import androidx.annotation.RequiresApi
import androidx.core.app.ActivityCompat
import androidx.lifecycle.ViewModelProvider
import androidx.recyclerview.widget.LinearLayoutManager
import com.example.crisislab.databinding.ActivityMainBinding
import okhttp3.OkHttpClient
import okhttp3.Request
import okhttp3.WebSocket

class MainActivity() : ComponentActivity() {
    lateinit var logViewModel: LogViewModel
    lateinit var socketStatusViewModel: SocketStatusViewModel;
    private lateinit var binding:ActivityMainBinding
    val notificationModule: NotificationModule = NotificationModule;
    lateinit var notificationHandler: NotificationHandler;

    @RequiresApi(Build.VERSION_CODES.O)
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
        binding = ActivityMainBinding.inflate(layoutInflater)
        setContentView(binding.root)
        val client = OkHttpClient()
        logViewModel = ViewModelProvider(this).get(LogViewModel::class.java)
        socketStatusViewModel = ViewModelProvider(this).get(SocketStatusViewModel::class.java);
        socketStatusViewModel.updateStatus("Status: Disconnected.")

		// If user high enough API version then ask for permission for notifications
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
            Log.d("Permission", "Asking for permissions")
            (ActivityCompat::requestPermissions)(this as Activity, arrayOf(Manifest.permission.POST_NOTIFICATIONS, Manifest.permission.USE_FULL_SCREEN_INTENT), 0)
        }

        binding.connect.setOnClickListener {
            connectToSocket(client)
        }

        connectToSocket(client)
        setRecylerView()
    }

    private fun connectToSocket(client: OkHttpClient) {
        val connections = client.connectionPool.connectionCount()

        if(connections > 0) {
            Log.e("Socket", "Already connected to WebSocket.")
            return;
        }

        Log.d("Socket", "Connecting");

        val request: Request = Request
            .Builder()
            .url("http://10.165.228.97:8081")
            .build()
        val listener = SocketListener(logViewModel, socketStatusViewModel, this);
        val ws: WebSocket = client.newWebSocket(request, listener)
    }

    private fun setRecylerView() {
        logViewModel.logItems.observe(this) {
            binding.logListRecyclerView.apply {
                layoutManager = LinearLayoutManager(applicationContext)
                adapter = it?.let { it1 -> LogItemAdapter(it1) }
            }
        }

        socketStatusViewModel.status.observe(this) {
            binding.WebSocketStatusRecyclerView.apply {
                layoutManager = LinearLayoutManager(applicationContext)
                adapter = it?.let { it1 -> SocketStatusAdapter(it1) }
            }
        }
    }
} 
