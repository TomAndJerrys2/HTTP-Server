package com.kylebrady.httpServer.config;

public class HttpConfigException extends Exception {

    // Mainly just using the SuperClass inheritance for the constructors
    public HttpConfigException(String message) { super(message); }
    public HttpConfigException(String message, Throwable cause) { super(message, cause); }
    public HttpConfigException(Throwable cause) { super(cause); }
}
