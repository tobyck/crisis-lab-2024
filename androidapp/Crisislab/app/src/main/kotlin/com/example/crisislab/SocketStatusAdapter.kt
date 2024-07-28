/*
 * Author: Alex Berry
 * Version: 29/07/2024
 * Purpose: Displays the WebSocket's current connectivity status
 */

package com.example.crisislab

import android.os.Build
import android.view.LayoutInflater
import android.view.ViewGroup
import androidx.annotation.RequiresApi
import androidx.recyclerview.widget.RecyclerView
import com.example.crisislab.databinding.StatusCellBinding

// Adapter for displaying socket status in a RecyclerView
class SocketStatusAdapter(
    private val status: String
) : RecyclerView.Adapter<SocketStatusViewHolder>() {
    // Create and return a ViewHolder for the item
    override fun onCreateViewHolder(parent: ViewGroup, viewType: Int): SocketStatusViewHolder {
        val layoutInflater = LayoutInflater.from(parent.context)
        val binding = StatusCellBinding.inflate(layoutInflater, parent, false)
        return SocketStatusViewHolder(binding)
    }

    // Return the number of items in the adapter (fixed to 1 in this case)
    override fun getItemCount(): Int = 1

    // Bind the status data to the ViewHolder
    @RequiresApi(Build.VERSION_CODES.O)
    override fun onBindViewHolder(holder: SocketStatusViewHolder, position: Int) {
        holder.bindStatus(status)
    }
}
