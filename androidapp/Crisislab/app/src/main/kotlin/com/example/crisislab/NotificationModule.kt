package com.example.crisislab

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.PendingIntent
import android.content.Context
import android.content.Intent
import android.os.Build
import androidx.core.app.NotificationCompat
import androidx.core.app.NotificationManagerCompat

object NotificationModule {

    // Function to build a notification
    fun build(context: Context, title: String, message: String, bigText: String?): Notification {
        // Intent to open the app when the user taps the notification
        val intent = Intent(context, MainActivity::class.java).apply {
            flags = Intent.FLAG_ACTIVITY_NEW_TASK or Intent.FLAG_ACTIVITY_CLEAR_TASK
        }
        val pendingIntent: PendingIntent = PendingIntent.getActivity(context, 0, intent, PendingIntent.FLAG_IMMUTABLE)

        // Intent for full-screen notifications
        val fullScreenIntent = Intent(context, MainActivity::class.java)
        val pendingFullScreenIntent = PendingIntent.getActivity(context, 0, fullScreenIntent, PendingIntent.FLAG_IMMUTABLE)

        // Build and return the notification
        return NotificationCompat.Builder(context, "Main Channel ID")
            .setPriority(NotificationCompat.PRIORITY_DEFAULT)
            .setSmallIcon(R.drawable.ic_notif)
            .setContentTitle(title)
            .setContentText(message)
            .setStyle(NotificationCompat.BigTextStyle().bigText(bigText))  // Set the style for the notification with big text
            .setPriority(NotificationCompat.PRIORITY_DEFAULT)  // Set the priority of the notification
            .setContentIntent(pendingIntent)
            .setFullScreenIntent(pendingFullScreenIntent, true)
            .setAutoCancel(true)  // Automatically remove the notification when the user taps it
            .setOngoing(true)
            .setVisibility(NotificationCompat.VISIBILITY_PUBLIC)
            .build()
    }

    // Function to provide the NotificationManagerCompat
    fun provideNotificationManager(context: Context): NotificationManagerCompat {
        val notificationManager = NotificationManagerCompat.from(context)
        // Create a notification channel for devices running Android O or higher
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            val channel = NotificationChannel(
                "Main Channel ID",  // Channel ID
                "Main Channel",  // Channel name
                NotificationManager.IMPORTANCE_HIGH  // Channel importance
            )
            notificationManager.createNotificationChannel(channel)
        }
        return notificationManager
    }
}
