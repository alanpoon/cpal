package rust.cpal;

import java.lang.UnsupportedOperationException;
import android.os.Build;
import android.util.Log;
import org.freedesktop.gstreamer.GStreamer;
public class MainActivity extends android.app.NativeActivity {

    static {

        String[] supported_abis;

        try {
            supported_abis = (String[]) Build.class.getField("SUPPORTED_ABIS").get(null);
        } catch (Exception e) {
            // Assume that this is an older phone; use backwards-compatible targets.
            supported_abis = new String[]{Build.CPU_ABI, Build.CPU_ABI2};
        }

        boolean matched_an_abi = false;

        System.loadLibrary("gstreamer_android");
        GStreamer.init(context);

        if (!matched_an_abi) {
            throw new UnsupportedOperationException("Could not find a native abi target to load");
        }

    }
}