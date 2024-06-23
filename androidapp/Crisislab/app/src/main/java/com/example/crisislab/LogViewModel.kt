package com.example.crisislab

import android.util.Log
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import java.time.LocalTime
import java.util.UUID

// This stores the log items
class LogViewModel: ViewModel() {
    var logItems = MutableLiveData<MutableList<LogItem>>(ArrayList());

    fun addLogItem(newLog: LogItem) {
        val list = logItems.value
        list!!.add(newLog)
        logItems!!.postValue(list)
    }
}
