package com.marcdif.ledagent.utils;

import lombok.Getter;
import lombok.Setter;
import org.yaml.snakeyaml.Yaml;

import java.io.File;
import java.io.FileInputStream;
import java.io.IOException;
import java.io.InputStream;

public class ConfigUtil {

    private LightConfig config;

    public ConfigUtil() {
        try {
            File configFile = new File("config.yml");

            System.out.println(configFile.getAbsolutePath());

            if (!configFile.exists()) configFile.createNewFile();

            InputStream targetStream = new FileInputStream(configFile);
            config = new Yaml().loadAs(targetStream, LightConfig.class);
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    public int[] getPixelLocations() {
        return config.getPixelLocations();
    }

    @Getter
    @Setter
    public static class LightConfig {
        private int corner1, corner2, corner3, corner4, ledCount;

        public int[] getPixelLocations() {
            return new int[]{corner1, corner2, corner3, corner4, ledCount};
        }
    }
}
