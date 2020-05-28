package io.rousan.globalcounter.services;


import android.app.Service;
import android.content.Intent;
import android.os.Handler;
import android.os.IBinder;
import android.os.Message;
import android.os.Messenger;
import android.os.RemoteException;

import io.rousan.globalcounter.activities.MainActivity;
import io.rousan.globalcounter.bridge.Bridge;
import io.rousan.globalcounter.bridge.MessageData;
import timber.log.Timber;

public class WorkerService extends Service {
    private Bridge bridge;
    private Messenger receiveMessenger;
    private Messenger sendMessenger;

    public WorkerService() {
        this.bridge = new Bridge(this, new Bridge.OnMessageListener() {
            @Override
            public void onMessage(int what, MessageData data) {
                if (sendMessenger != null) {
                    Message message = Message.obtain(null, what);
                    message.setData(data.getData());

                    try {
                        sendMessenger.send(message);
                    } catch (RemoteException e) {
                        e.printStackTrace();
                    }
                }
            }
        });
    }

    @Override
    public void onCreate() {
        super.onCreate();
        Timber.i("Worker Service created");
        this.bridge.start();
    }

    @Override
    public int onStartCommand(Intent intent, int flags, int startId) {
        Timber.i("Worker Service onStartCommand");
        return START_NOT_STICKY;
    }

    @Override
    public IBinder onBind(Intent intent) {
        Timber.i("Worker Service onBind");
        receiveMessenger = new Messenger(new IncomingHandler(this));
        return receiveMessenger.getBinder();
    }

    @Override
    public boolean onUnbind(Intent intent) {
        Timber.i("Worker Service onUnbind");
        sendMessenger = null;
        return true;
    }

    @Override
    public void onRebind(Intent intent) {
        super.onRebind(intent);
        Timber.i("Worker Service onRebind");
    }

    @Override
    public void onDestroy() {
        super.onDestroy();
        Timber.i("Worker Service destroyed");
        this.bridge.shutdown();
    }

    public static class IncomingHandler extends Handler {
        private WorkerService workerService;

        IncomingHandler(WorkerService workerService) {
            this.workerService = workerService;
        }

        @Override
        public void handleMessage(Message msg) {
            if (msg.what == MainActivity.WHAT_SET_REPLY_MESSENGER) {
                workerService.sendMessenger = msg.replyTo;
                return;
            }

            workerService.bridge.sendMessage(msg.what, new MessageData(msg.getData()));
        }
    }
}

