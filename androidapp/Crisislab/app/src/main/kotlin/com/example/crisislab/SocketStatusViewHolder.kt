package com.example.crisislab

import android.os.Build
import androidx.annotation.RequiresApi
import androidx.recyclerview.widget.RecyclerView
import com.example.crisislab.databinding.StatusCellBinding

// ViewHolder for displaying socket status in a RecyclerView
class SocketStatusViewHolder(
    // Binding object for the item layout
    private val binding: StatusCellBinding
) : RecyclerView.ViewHolder(binding.root) {

    // Bind the status data to the item view
    @RequiresApi(Build.VERSION_CODES.O)
    fun bindStatus(status: String) {
        binding.WebSocketStatus.text = status
    }
}
