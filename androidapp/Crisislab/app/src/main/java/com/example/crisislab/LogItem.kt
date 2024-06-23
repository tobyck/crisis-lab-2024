package com.example.crisislab

import java.util.UUID

class LogItem(
    var height: String,
    var time: String?,
    var id: UUID = UUID.randomUUID()
)
