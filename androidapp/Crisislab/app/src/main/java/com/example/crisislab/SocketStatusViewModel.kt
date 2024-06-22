package com.example.crisislab

import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel

class SocketStatusViewModel: ViewModel() {
    var status: MutableLiveData<String> = MutableLiveData();

    fun updateStatus(newStatus: String) {
        status.postValue(newStatus);
    }
}