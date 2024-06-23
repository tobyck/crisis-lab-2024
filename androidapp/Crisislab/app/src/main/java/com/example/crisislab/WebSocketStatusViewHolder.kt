package com.example.crisislab

import android.content.Context
import android.os.Build
import androidx.annotation.RequiresApi
import androidx.recyclerview.widget.RecyclerView
import com.example.crisislab.databinding.StatusCellBinding
import java.time.format.DateTimeFormatter

// This renders the WebSocket status
class WebSocketStatusViewHolder(
    private val context: Context,
    private val binding: StatusCellBinding
): RecyclerView.ViewHolder(binding.root) {
    @RequiresApi(Build.VERSION_CODES.O)
    fun bindStatus(status: String) {
        binding.WebSocketStatus.text = status
    }
}
