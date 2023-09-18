package com.marcdif.ledagent.handlers;

import com.github.mbelling.ws281x.Color;
import com.github.mbelling.ws281x.LedStripType;
import com.github.mbelling.ws281x.Ws281xLedStrip;
import com.marcdif.ledagent.Main;
import com.marcdif.ledagent.utils.ColorUtil;
import lombok.Getter;

import java.util.Arrays;

public class LightStrip {
    private final Ws281xLedStrip strip;
    private final long[] pixels;
    @Getter private final StripSettings stripSettings;

    public LightStrip(int ledCount) {
        this(ledCount, 18, 800000, 10, 255, 0, false);
    }


    public LightStrip(int ledCount, int ledPin, int ledFreqHz, int ledDMZ, int ledBrightness, int ledChannel, boolean ledInvert) {
        this(new StripSettings(ledCount, ledPin, ledFreqHz, ledDMZ, ledBrightness, ledChannel, ledInvert));
    }

    public LightStrip(StripSettings stripSettings) {
        this.stripSettings = stripSettings;
        this.strip = stripSettings.instantiateHardwareLedStrip();
        this.pixels = new long[strip.getLedsCount()];
    }

    public void render() {
        strip.render();
    }

    public void setAll(Color color) {
        color = ColorUtil.verify(color);

        if (Main.DEBUG)
            Main.logMessage("[LIGHTS] Set all to " + color.getColorBits() + "!");
        strip.setStrip(color);
        Arrays.fill(pixels, color.getColorBits());
    }

    public void setPixel(Color color, int pixel) {
        color = ColorUtil.verify(color);

        if (Main.DEBUG)
            Main.logMessage("[LIGHTS] Set ID:" + pixel + " to " + color.getColorBits() + "!");
        strip.setPixel(pixel, color);
        pixels[pixel] = color.getColorBits();
    }

    public void setPixels(Color color, int... pixel) {
        color = ColorUtil.verify(color);

        if (Main.DEBUG)
            Main.logMessage("[LIGHTS] Set pixels to " + color.getColorBits() + "!");
        for (int i : pixel) {
            setPixel(color, i);
        }
    }

    public void setPixels(long[] arr) {
        if (Main.DEBUG)
            Main.logMessage("[LIGHTS] Set pixels to array \"" + Arrays.toString(arr) + "\"!");
        int bound;
        if (arr.length >= pixels.length) {
            if (arr.length > pixels.length) {
                Main.logMessage("[LIGHTS] [WARNING] Color array is larger than pixel array, color array will be truncated!");
            }
            bound = pixels.length;
        } else {
            bound = arr.length;
        }
        for (int i = 0; i < bound; i++) {
            setPixel(ColorUtil.getColor(arr[i]), i);
        }
    }

    public Color getPixel(int pixel) {
        if (pixel < pixels.length && pixel >= 0) {
            return new Color(pixels[pixel]);
        }
        return null;
    }

    /**
     * Turn off all strip pixels
     */
    public void clear() {
        setAll(Color.BLACK);
    }

    public int getPixelCount() {
        return strip.getLedsCount();
    }

    public void renderAllOneColor(Color color) {
        long[] arr = new long[getPixelCount()];
        Arrays.fill(arr, color.getColorBits());
        setPixels(arr);
    }

    public record StripSettings(int ledCount, int ledPin, int ledFreqHz, int ledDMZ, int ledBrightness, int ledChannel,
                                boolean ledInvert) {
        public Ws281xLedStrip instantiateHardwareLedStrip() {
            return new Ws281xLedStrip(ledCount, ledPin, ledFreqHz, ledDMZ, ledBrightness, ledChannel, ledInvert,
                    LedStripType.WS2811_STRIP_GRB, true);
        }
    }

}
