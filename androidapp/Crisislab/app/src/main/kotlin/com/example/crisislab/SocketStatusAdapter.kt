package com.example.crisislab

import android.os.Build
import android.view.LayoutInflater
import android.view.ViewGroup
import androidx.annotation.RequiresApi
import androidx.recyclerview.widget.RecyclerView
import com.example.crisislab.databinding.StatusCellBinding

class SocketStatusAdapter(
    private val status: String
): RecyclerView.Adapter<SocketStatusViewHolder>() {
    override fun onCreateViewHolder(parent: ViewGroup, viewType: Int): SocketStatusViewHolder {
        val from = LayoutInflater.from(parent.context)
        val binding = StatusCellBinding.inflate(from, parent, false)
        return SocketStatusViewHolder(binding)
    }

    override fun getItemCount(): Int {
        return 1
    }

    @RequiresApi(Build.VERSION_CODES.O)
    override fun onBindViewHolder(holder: SocketStatusViewHolder, position: Int) {
        holder.bindStatus(status)
    }
}