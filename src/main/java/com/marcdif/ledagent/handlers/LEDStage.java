package com.marcdif.ledagent.handlers;

import com.github.mbelling.ws281x.Color;
import lombok.Getter;

import java.util.Arrays;

@Getter
public class LEDStage {
    private final int corner1, corner2, corner3, corner4, ledCount;

    private LightStrip lightStrip = null;

    public LEDStage(int[] pixelLocations) {
        this(pixelLocations[0], pixelLocations[1], pixelLocations[2], pixelLocations[3], pixelLocations[4]);

        if (pixelLocations.length != 5)
            throw new IllegalArgumentException("'pixelLocations' array should have exactly five values!");
    }

    public LEDStage(int corner1, int corner2, int corner3, int corner4, int ledCount) {
        this.corner1 = corner1;
        this.corner2 = corner2;
        this.corner3 = corner3;
        this.corner4 = corner4;
        this.ledCount = ledCount;
        if (corner1 < 0 || corner2 < 0 || corner3 < 0 || corner4 < 0) {
            throw new IllegalArgumentException("None of the corner values can be negative!");
        }
        if (ledCount < Math.max(Math.max(Math.max(corner1, corner2), corner3), corner4)) {
            throw new IllegalArgumentException("None of the corner values can be larger than ledCount!");
        }
    }

    public void setLightStrip(LightStrip lightStrip) {
        if (lightStrip == null) {
            throw new IllegalArgumentException("lightStrip can't be null!");
        }
        this.lightStrip = lightStrip;
    }

    public long[] renderAllOneColor(Color color) {
        long[] arr = new long[ledCount];
        Arrays.fill(arr, color.getColorBits());
        return arr;
    }

    public void setAll(Color color) {
        lightStrip.setAll(color);
    }

    public Color getPixelColor(int pixel) {
        return lightStrip.getPixel(pixel);
    }
}
