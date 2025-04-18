package com.oxideWorldgen;

import java.io.FileNotFoundException;
import java.io.IOException;
import java.io.InputStream;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.nio.file.StandardCopyOption;

public class NativeLibLoader {
    public static void loadNative(String libName) {
        Oxide.LOGGER.info("Loading Native lib: {}", libName);
        String os = System.getProperty("os.name").toLowerCase();
        String libPath;

        if (os.contains("win")) {
            libPath = "native/windows/" + libName + ".dll";
        } else if (os.contains("mac")) {
            libPath = "native/mac/lib" + libName + ".dylib";
        } else {
            libPath = "native/linux/lib" + libName + ".so";
        }

        try (InputStream in = Thread.currentThread().getContextClassLoader().getResourceAsStream(libPath)) {
            if (in == null) throw new FileNotFoundException("Native lib not found in JAR: " + libPath);

            Path tempFile = Files.createTempFile(libName, libPath.substring(libPath.lastIndexOf(".")));
            Files.copy(in, tempFile, StandardCopyOption.REPLACE_EXISTING);
            tempFile.toFile().deleteOnExit();


            System.load(tempFile.toAbsolutePath().toString());
            Oxide.LOGGER.info("Loaded Native lib from: {}", tempFile.toAbsolutePath());
        } catch (IOException e) {
            throw new RuntimeException("Failed to load native library: " + Paths.get(libPath).toAbsolutePath(), e);
        }
    }
}
