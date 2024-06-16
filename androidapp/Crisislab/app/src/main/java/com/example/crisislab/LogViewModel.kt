package com.example.crisislab

import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import java.time.LocalTime
import java.util.UUID

class LogViewModel: ViewModel()
{
    var logItems = MutableLiveData<MutableList<LogItem>?>()

    init {
        logItems.value = mutableListOf()
    }

    fun addLogItem(newLog: LogItem){
        val list = logItems.value
        list!!.add(newLog)
        logItems.postValue(list)
    }

    fun updateLogItem(id: UUID, height: String, time: String){
        val list = logItems.value
        val log = list!!.find { it.id == id }!!
        log.height = height
        log.time = time
        logItems.postValue(list)
    }

    companion object {
        var logItems = MutableLiveData<MutableList<LogItem>?>()

        fun addLogItem(newLog: LogItem) {
            val list = logItems.value
            list!!.add(newLog)
            logItems.postValue(list)
        }
    }

}