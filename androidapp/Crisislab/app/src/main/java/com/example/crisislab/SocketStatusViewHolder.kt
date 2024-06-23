package com.example.crisislab

import android.os.Build
import androidx.annotation.RequiresApi
import androidx.recyclerview.widget.RecyclerView
import com.example.crisislab.databinding.StatusCellBinding

// This renders the WebSocket status
class SocketStatusViewHolder(
    private val binding: StatusCellBinding
): RecyclerView.ViewHolder(binding.root) {
    @RequiresApi(Build.VERSION_CODES.O)
    fun bindStatus(status: String) {
        binding.WebSocketStatus.text = status
    }
}
Build