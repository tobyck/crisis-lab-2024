package com.example.crisislab

import android.os.Build
import androidx.annotation.RequiresApi
import androidx.recyclerview.widget.RecyclerView
import com.example.crisislab.databinding.LogItemCellBinding
import java.time.Instant
import java.time.ZoneId
import java.time.format.DateTimeFormatter

// This renders the log items
class LogItemViewHolder(
    private val binding: LogItemCellBinding
): RecyclerView.ViewHolder(binding.root) {
    @RequiresApi(Build.VERSION_CODES.O)
    fun bindLogItem(logItem: LogItem) {
        val time = Instant.ofEpochMilli(logItem.time!!.toLong())
        val zonedTime = time.atZone(ZoneId.of("Pacific/Auckland"))
        val formattedTime =
            DateTimeFormatter.ofPattern("kk:mm - dd/MM/yy").format(zonedTime)

        binding.height.text = logItem.height
        binding.time.text = formattedTime
    }
}
