package com.example.crisislab

import android.content.Context
import android.os.Build
import androidx.annotation.RequiresApi
import androidx.recyclerview.widget.RecyclerView
import com.example.crisislab.databinding.LogItemCellBinding
import java.time.format.DateTimeFormatter

// This renders the log items
class LogItemViewHolder(
    private val context: Context,
    private val binding: LogItemCellBinding
): RecyclerView.ViewHolder(binding.root) {
    @RequiresApi(Build.VERSION_CODES.O)
    fun bindLogItem(logItem: LogItem) {
        binding.height.text = logItem.height
        binding.time.text = logItem.time ?: "";
    }
}
