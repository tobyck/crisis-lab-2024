package com.example.crisislab

import android.os.Build
import android.view.LayoutInflater
import android.view.ViewGroup
import androidx.annotation.RequiresApi
import androidx.recyclerview.widget.RecyclerView
import com.example.crisislab.databinding.LogItemCellBinding

// Adapter class for displaying LogItem data in a RecyclerView
class LogItemAdapter(
    private val logItems: List<LogItem>
) : RecyclerView.Adapter<LogItemViewHolder>() {

    // Called when RecyclerView needs a new ViewHolder of the given type to represent an item
    override fun onCreateViewHolder(parent: ViewGroup, viewType: Int): LogItemViewHolder {
        val layoutInflater = LayoutInflater.from(parent.context)
        // Inflate the layout for individual log item cells and create a binding object
        val binding = LogItemCellBinding.inflate(layoutInflater, parent, false)
        return LogItemViewHolder(binding)
    }

    override fun getItemCount(): Int = logItems.size

    // Called by RecyclerView to display the data at the specified position
    @RequiresApi(Build.VERSION_CODES.O)
    override fun onBindViewHolder(holder: LogItemViewHolder, position: Int) {
        // Bind the data for the log item at the given position to the ViewHolder
        holder.bindLogItem(logItems[position])
    }
}
