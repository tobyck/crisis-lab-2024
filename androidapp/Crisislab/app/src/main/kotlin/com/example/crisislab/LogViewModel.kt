/*
 * Author: Alex Berry
 * Version: 29/07/2024
 * Purpose:
 */
package com.example.crisislab

import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel

// ViewModel to store and manage log items
class LogViewModel : ViewModel() {
    var logItems = MutableLiveData<MutableList<LogItem>>(ArrayList())

    fun addLogItem(newLog: LogItem) {
        val list = logItems.value
        list!!.add(newLog)
        logItems!!.postValue(list)
    }
}
