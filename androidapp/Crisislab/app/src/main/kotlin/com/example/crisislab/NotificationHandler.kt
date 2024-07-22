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
    private val notificationManager: NotificationManagerCompat,
    private val context: Context
) : Service() {
    var isServiceRunning = false;

    override fun onBind(intent: Intent?): IBinder? {
        return null!!
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        return START_NOT_STICKY;
    }

    fun showNotification(title: String, message: String, type: String?, timestamp: String?) {
        if (ActivityCompat.checkSelfPermission(
                context,
                Manifest.permission.POST_NOTIFICATIONS
            ) != PackageManager.PERMISSION_GRANTED
        ) {
            Log.d("NotificationHandler", "Permission denied");
            return
        }
        Log.d("NotificationHandler", "Permission Granted, notifying.")

        if(type === "TSUNAMI") {
            val notif = NotificationModule.build(context, title, "A Tsunami has been detected!", "A $message Tsunami has been detected! $timestamp");
            notificationManager.notify(1, notif);
        }
    }
}