package com.example.crisislab

import android.app.NotificationChannel
import android.app.NotificationManager
import android.content.Context
import android.os.Build
import androidx.core.app.NotificationCompat
import androidx.core.app.NotificationManagerCompat
import androidx.core.graphics.drawable.IconCompat


object NotificationModule {

    fun provideNotificationBuilder(context: Context): NotificationCompat.Builder {
        return NotificationCompat.Builder(context, "Main Channel ID")
            .setContentTitle("Title")
            .setContentText("Text")
            .setPriority(NotificationCompat.PRIORITY_DEFAULT)
            .setSmallIcon(R.drawable.ic_notif)
    }

    fun build(context: Context, title: String, message: String): NotificationCompat.Builder {
        return NotificationCompat.Builder(context, "Main Channel ID")
            .setSmallIcon(R.drawable.ic_notif)
            .setContentTitle(title)
            .setContentText(message)
            .setPriority(NotificationCompat.PRIORITY_DEFAULT)
            .setAutoCancel(true)
            .setOngoing(true)
            .setVisibility(NotificationCompat.VISIBILITY_PUBLIC)
    }

    fun provideNotificationManager(context: Context): NotificationManagerCompat {
        val notificationManager = NotificationManagerCompat.from(context)
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            val channel = NotificationChannel(
                "Main Channel ID",
                "Main Channel",
                NotificationManager.IMPORTANCE_DEFAULT
            )
            notificationManager.createNotificationChannel(channel)
        }
        return notificationManager;
    }
}