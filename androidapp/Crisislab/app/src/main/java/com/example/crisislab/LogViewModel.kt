package com.example.crisislab

import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel

// This stores the log items
class LogViewModel: ViewModel() {
    var logItems = MutableLiveData<MutableList<LogItem>>(ArrayList());

    fun addLogItem(newLog: LogItem) {
        val list = logItems.value
        list!!.add(newLog)
        logItems!!.postValue(list)
    }
}
