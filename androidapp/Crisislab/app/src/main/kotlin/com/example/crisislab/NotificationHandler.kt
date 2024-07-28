import android.Manifest
import android.app.Service
import android.content.Context
import android.content.Intent
import android.content.pm.PackageManager
import android.os.Build
import android.os.IBinder
import android.util.Log
import androidx.annotation.RequiresApi
import androidx.core.app.ActivityCompat
import androidx.core.app.NotificationManagerCompat
import com.example.crisislab.NotificationModule

@RequiresApi(Build.VERSION_CODES.TIRAMISU)
class NotificationHandler (
    // Notification manager for displaying notifications
    private val notificationManager: NotificationManagerCompat,
    // Needs a comment
    private val context: Context
) : Service() {
    var isServiceRunning = false

    // Binding the service to a client (not used)
    override fun onBind(intent: Intent?): IBinder? {
        return null!!
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        return START_NOT_STICKY // Service won't restart if the system stops it
    }

    fun showNotification(title: String, message: String, type: String?, timestamp: String?) {
        // Check if the app has permission to post notifications
        if (ActivityCompat.checkSelfPermission(
                context,
                Manifest.permission.POST_NOTIFICATIONS
            ) != PackageManager.PERMISSION_GRANTED
        ) {
            Log.d("NotificationHandler", "Permission denied")
            return
        }
        Log.d("NotificationHandler", "Permission Granted, notifying.")

        // Create and display a notification if the type is "TSUNAMI"
        if (type == "TSUNAMI") {
            val notif = NotificationModule.build(context, title, "A Tsunami has been detected!", "A $message Tsunami has been detected! $timestamp")
            notificationManager.notify(1, notif)
        }
    }
}
