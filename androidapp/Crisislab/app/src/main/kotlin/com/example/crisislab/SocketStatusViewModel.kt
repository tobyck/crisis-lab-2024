package com.example.crisislab

import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel

// ViewModel for managing and providing the WebSocket status
class SocketStatusViewModel: ViewModel() {
    // LiveData for holding the current status of the WebSocket connection
    var status: MutableLiveData<String> = MutableLiveData()

    // Update the WebSocket status and notify observers
    fun updateStatus(newStatus: String) {
        status.postValue(newStatus)
    }
}
