package io.rousan.globalcounter.activities;


import android.content.ComponentName;
import android.content.Context;
import android.content.Intent;
import android.content.ServiceConnection;
import android.graphics.Color;
import android.os.Bundle;
import android.os.Handler;
import android.os.IBinder;
import android.os.Message;
import android.os.Messenger;
import android.os.RemoteException;
import android.view.Menu;
import android.view.MenuItem;
import android.view.View;
import android.widget.TextView;

import com.google.android.material.snackbar.Snackbar;

import androidx.appcompat.app.AppCompatActivity;
import io.rousan.globalcounter.R;
import io.rousan.globalcounter.bridge.MessageData;
import io.rousan.globalcounter.message.What;
import io.rousan.globalcounter.services.WorkerService;
import timber.log.Timber;

public class MainActivity extends AppCompatActivity {
    public static final int WHAT_SET_REPLY_MESSENGER = Integer.MAX_VALUE;

    private boolean isBoundWithService;
    private Messenger sendMessenger;
    private Messenger receiveMessenger;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        startService(new Intent(this, WorkerService.class));
        bindService(new Intent(this, WorkerService.class), serviceConnection, Context.BIND_AUTO_CREATE);
    }

    public void onBridgeMessage(int what, MessageData data) {
        Timber.i("Got a message: what: %s", What.toString(what));

        switch (what) {
            case What.COUNTER_VALUE: {
                final String counter_value = data.getString("value");
                runOnUiThread(new Runnable() {
                    @Override
                    public void run() {
                        TextView tv = (TextView) findViewById(R.id.tv_counter_val);
                        tv.setText(counter_value);
                    }
                });
                return;
            }
            case What.ERROR: {
                final String msg = data.getString("msg");
                runOnUiThread(new Runnable() {
                    @Override
                    public void run() {
                        showSnackbar(msg, false);
                    }
                });
                return;
            }
            case What.SNACK_BAR_MSG: {
                final String msg = data.getString("msg");
                runOnUiThread(new Runnable() {
                    @Override
                    public void run() {
                        showSnackbar(msg, true);
                    }
                });
                return;
            }
        }
    }

    public void onIncreaseButtonClick(View v) {
        sendMessage(What.INCREASE_COUNTER, MessageData.empty());
    }

    public void onDecreaseButtonClick(View v) {
        sendMessage(What.DECREASE_COUNTER, MessageData.empty());
    }

    public void sendMessage(int what, MessageData data) {
        if (isBoundWithService) {
            Message msg = Message.obtain(null, what);
            msg.setData(data.getData());

            try {
                sendMessenger.send(msg);
            } catch (RemoteException exp) {
                Timber.i(exp);
            }
        }
    }

    public void showSnackbar(String msg, boolean isSuccess) {
        if (isSuccess) {
            Snackbar.make(findViewById(android.R.id.content), msg, Snackbar.LENGTH_SHORT)
                    .show();
        } else {
            final Snackbar snackbar = Snackbar.make(findViewById(android.R.id.content), msg, Snackbar.LENGTH_INDEFINITE);
            snackbar.setAction("Close", new View.OnClickListener() {
                @Override
                public void onClick(View v) {
                    snackbar.dismiss();
                }
            });
            snackbar.show();
        }
    }

    private ServiceConnection serviceConnection = new ServiceConnection() {
        @Override
        public void onServiceConnected(ComponentName name, IBinder service) {
            Timber.i("Connected with service");

            sendMessenger = new Messenger(service);
            receiveMessenger = new Messenger(new IncomingHandler(MainActivity.this));
            isBoundWithService = true;

            try {
                Message msg = Message.obtain(null, WHAT_SET_REPLY_MESSENGER);
                msg.replyTo = receiveMessenger;
                sendMessenger.send(msg);
            } catch (RemoteException exp) {
                Timber.e(exp);
            }

            sendMessage(What.INITIATE, MessageData.empty());
        }

        @Override
        public void onServiceDisconnected(ComponentName name) {
            Timber.i("Disconnected with service");

            sendMessenger = null;
            receiveMessenger = null;
            isBoundWithService = false;
        }
    };

    public static class IncomingHandler extends Handler {
        private MainActivity mainActivity;

        IncomingHandler(MainActivity mainActivity) {
            this.mainActivity = mainActivity;
        }

        @Override
        public void handleMessage(Message msg) {
            mainActivity.onBridgeMessage(msg.what, new MessageData(msg.getData()));
        }
    }

    @Override
    public boolean onCreateOptionsMenu(Menu menu) {
        getMenuInflater().inflate(R.menu.menu_main, menu);
        return true;
    }

    @Override
    public boolean onOptionsItemSelected(MenuItem item) {
        int id = item.getItemId();

        if (id == R.id.action_about) {
            return true;
        }

        return super.onOptionsItemSelected(item);
    }

    @Override
    protected void onDestroy() {
        super.onDestroy();
        unbindService(serviceConnection);
    }
}
