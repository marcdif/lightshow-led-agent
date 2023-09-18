package com.marcdif.ledagent.lights;

import com.marcdif.ledagent.Main;
import com.marcdif.ledagent.handlers.LEDStage;
import com.marcdif.ledagent.handlers.LightStrip;
import com.marcdif.ledagent.show.ShowManager;
import lombok.Getter;

public class LightManager {
    @Getter private final LightThread lightThread;
    private final LightStrip lightStrip;
    @Getter private final LEDStage ledStage;
    @Getter private final ShowManager showManager;

    public LightManager() {
        ledStage = new LEDStage(Main.getConfigUtil().getPixelLocations());

        lightStrip = new LightStrip(ledStage.getLedCount());
        lightStrip.clear();

        ledStage.setLightStrip(lightStrip);

        lightThread = new LightThread(LightState.STARTUP, lightStrip);
        lightThread.start();

        showManager = new ShowManager();
    }

    public LightState getLightState() {
        return lightThread.getLightState();
    }
}
