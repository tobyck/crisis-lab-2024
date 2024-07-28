/*
 * Author: Alex Berry
 * Version: 29/07/2024
 * Purpose: Display for the WebSocket's status
 */

package com.example.crisislab

import android.os.Build
import androidx.annotation.RequiresApi
import androidx.recyclerview.widget.RecyclerView
import com.example.crisislab.databinding.StatusCellBinding

// ViewHolder for displaying socket status in a RecyclerView
class SocketStatusViewHolder(
    private val binding: StatusCellBinding
) : RecyclerView.ViewHolder(binding.root) {
    
    // Bind the status data to the item view
    @RequiresApi(Build.VERSION_CODES.O)
    fun bindStatus(status: String) {
        binding.WebSocketStatus.text = status
    }
}
