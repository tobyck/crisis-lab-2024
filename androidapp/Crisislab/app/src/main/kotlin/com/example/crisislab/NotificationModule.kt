/*
 * Author: Maxwell Robati
 * Version: 29/07/2024
 * Purpose: Container for all notifications
 */

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
        
        val pendingIntent: PendingIntent = PendingIntent.getActivity(context, 0, intent, PendingIntent.FLAG_IMMUTABLE) // Opens the app when triggered

        // Intent for full-screen notifications
        val fullScreenIntent = Intent(context, MainActivity::class.java)
        val pendingFullScreenIntent = PendingIntent.getActivity(context, 0, fullScreenIntent, PendingIntent.FLAG_IMMUTABLE)

        // Build and return the notification
        return NotificationCompat.Builder(context, "Main Channel ID")
            .setSmallIcon(R.drawable.ic_notif)
            .setContentTitle(title)
            .setContentText(message)
            .setStyle(NotificationCompat.BigTextStyle().bigText(bigText))  // Sets the text when the notification is dropped down
            .setPriority(NotificationCompat.PRIORITY_HIGH)  // Allows the notification to be a heads up one
            .setContentIntent(pendingIntent) // Opens the app when the user clicks on the notification
            .setFullScreenIntent(pendingFullScreenIntent, true)
            .setAutoCancel(true)  // Automatically remove the notification when the user taps it
            .setOngoing(true)
            .setVisibility(NotificationCompat.VISIBILITY_PUBLIC) // Makes the notification visible in full on the lock screen
            .build()
    }

    // Function to generate a Notification Manager
    fun provideNotificationManager(context: Context): NotificationManagerCompat {
        val notificationManager = NotificationManagerCompat.from(context)
        // Create a notification channel for devices running a high enough version of Android
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            val channel = NotificationChannel(
                "Main Channel ID",
                "Main Channel",
                NotificationManager.IMPORTANCE_HIGH  // Makes the notification make a sound and be a headsup notification
            )
            notificationManager.createNotificationChannel(channel)
        }
        return notificationManager
    }
}
