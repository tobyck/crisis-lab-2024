/*
 * Author: Alex Berry
 * Version: 29/07/2024
 * Purpose: Handles displaying individual items within the RecyclerView
 */

package com.example.crisislab

import android.os.Build
import androidx.annotation.RequiresApi
import androidx.recyclerview.widget.RecyclerView
import com.example.crisislab.databinding.LogItemCellBinding
import java.time.Instant
import java.time.ZoneId
import java.time.format.DateTimeFormatter

// ViewHolder class for rendering log items in the RecyclerView
class LogItemViewHolder(
    private val binding: LogItemCellBinding
) : RecyclerView.ViewHolder(binding.root) {
    @RequiresApi(Build.VERSION_CODES.O)
    fun bindLogItem(logItem: LogItem) {
        // Convert the unix timestamp to a human-readable format
        val time = Instant.ofEpochMilli(logItem.time!!.toLong())
        val zonedTime = time.atZone(ZoneId.of("Pacific/Auckland"))
        val formattedTime = DateTimeFormatter.ofPattern("kk:mm - dd/MM/yy").format(zonedTime)

        // Bind the height and formatted time to the respective TextViews
        binding.height.text = logItem.height
        binding.time.text = formattedTime
    }
}
