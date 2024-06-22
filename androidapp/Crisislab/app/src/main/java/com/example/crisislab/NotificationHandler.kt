import android.Manifest
import android.app.Activity
import android.app.Notification
import android.app.PendingIntent
import android.app.Service
import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.content.pm.PackageManager
import android.os.Build
import android.os.IBinder
import android.util.Log
import androidx.annotation.RequiresApi
import androidx.core.app.ActivityCompat
import androidx.core.app.NotificationCompat
import androidx.core.app.NotificationManagerCompat

@RequiresApi(Build.VERSION_CODES.TIRAMISU)
class NotificationHandler (
    private val notificationBuilder: NotificationCompat.Builder,
    private val notificationManager: NotificationManagerCompat,

    private val context: Context
) : Service() {
    var isServiceRunning = false;
    lateinit var fullScreenIntent: Intent;
    lateinit var fullScreenPendingIntent: PendingIntent;


    override fun onBind(intent: Intent?): IBinder? {
        return null!!
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        return Service.START_NOT_STICKY;
    }

    fun showNotification(title: String, message: String, type: String?, timestamp: String?) {
        if (ActivityCompat.checkSelfPermission(
                context,
                Manifest.permission.POST_NOTIFICATIONS
            ) != PackageManager.PERMISSION_GRANTED
        ) {
            Log.d("NotificationHandler", "Permission denied");
            // TODO: Consider calling
            //    ActivityCompat#requestPermissions
            // here to request the missing permissions, and then overriding
            //   public void onRequestPermissionsResult(int requestCode, String[] permissions,
            //                                          int[] grantResults)
            // to handle the case where the user grants the permission. See the documentation
            // for ActivityCompat#requestPermissions for more details.
            return
        }
        Log.d("NotificationHandler", "Permission Granted, notifying.")

        if(type === "TSUNAMI") {
            notificationBuilder.setContentTitle(title)
            //notificationBuilder.setContentText(message)
            notificationBuilder.setContentText("A Tsunami has been detected!")
            notificationBuilder.setStyle(NotificationCompat.BigTextStyle().bigText("A $message Tsunami has been detected! $timestamp"))
            fullScreenIntent = Intent(context, NotificationHandler::class.java)
            fullScreenPendingIntent = PendingIntent.getActivity(context, 0,
                fullScreenIntent, PendingIntent.FLAG_MUTABLE)
            notificationBuilder.setFullScreenIntent(fullScreenPendingIntent, true);
            notificationManager.notify(1, notificationBuilder.build());
        }
    }
}