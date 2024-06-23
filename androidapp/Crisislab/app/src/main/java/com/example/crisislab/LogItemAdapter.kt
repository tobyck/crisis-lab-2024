package com.example.crisislab

import android.os.Build
import android.view.LayoutInflater
import android.view.ViewGroup
import androidx.annotation.RequiresApi
import androidx.recyclerview.widget.RecyclerView
import com.example.crisislab.databinding.LogItemCellBinding

class LogItemAdapter(
    private val logItems: List<LogItem>
): RecyclerView.Adapter<LogItemViewHolder>() {
    override fun onCreateViewHolder(parent: ViewGroup, viewType: Int): LogItemViewHolder {
        val from = LayoutInflater.from(parent.context)
        val binding = LogItemCellBinding.inflate(from, parent, false)
        return LogItemViewHolder(parent.context, binding)
    }

    override fun getItemCount(): Int = logItems.size

	// it's telling the um... fuck off... it's telling the logitemviewholder, it's telling the logitemviewholder *laughter* laughter in square brackets but then surround them in the square brackets
	// ok so it's telling this to bind the log item to - it's saying whenever the log items update, you wanna update it in the view holder... i think. don't quote me on that
    @RequiresApi(Build.VERSION_CODES.O)
    override fun onBindViewHolder(holder: LogItemViewHolder, position: Int) {
        holder.bindLogItem(logItems[position])
    }
}
