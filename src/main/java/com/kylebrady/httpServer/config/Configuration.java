package com.kylebrady.httpServer.config;

public class Configuration {
    private int port; // active listening ports
    private String webroot; // webroot is the active directory where the files are stored on the server

    // Getters and Setters
    public int getPort(int port) {
        return port;
    }

    public void setPort(int port) {
        this.port = port;
    }

    public String getWebRoot(String webroot) {
        return webroot;
    }

    public void setWebRoot(String webroot) {
        this.webroot = webroot;
    }
}
