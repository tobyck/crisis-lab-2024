/*
 * Author: Alex Berry
 * Version: 29/07/2024
 * Purpose: Holds and updates the WebSocket's status
 */

package com.example.crisislab

import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel

// ViewModel for managing and providing the WebSocket status
class SocketStatusViewModel: ViewModel() {
    // Holds the current status of the WebSocket connection
    var status: MutableLiveData<String> = MutableLiveData()

    // Update the WebSocket status and notify observers
    fun updateStatus(newStatus: String) {
        status.postValue(newStatus)
    }
}
