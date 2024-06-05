package com.example.Crisislab

import java.time.LocalTime
import java.util.UUID

class LogItem(
    var height: String,
    var time: LocalTime,
    var id: UUID = UUID.randomUUID()
) {

}