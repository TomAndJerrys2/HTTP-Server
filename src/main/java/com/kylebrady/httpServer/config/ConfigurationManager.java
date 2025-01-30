package com.kylebrady.httpServer.config;

import com.fasterxml.jackson.databind.JsonNode;
import com.kylebrady.httpServer.util.JSON;

import java.io.FileNotFoundException;
import java.io.FileReader;
import java.io.IOException;

public class ConfigurationManager {

    private static ConfigurationManager myConfigurationManager; // the config manager singleton
    private static Configuration myCurrentConfiguration; // the curremt config

    private ConfigurationManager () { }

    public static ConfigurationManager getInstance() {
        if(myConfigurationManager == null) {
            myConfigurationManager = new ConfigurationManager(); // If the instance is null i.e there isnt once
            // we create a new instance
        } return myConfigurationManager; // else return the config manager
    }

    // Used to load a configuration file from a specified path
    public void loadConfigurationFile(String filePath) {
        FileReader fileReader = null;
        try {
            fileReader = new FileReader(filePath);
        } catch(FileNotFoundException e) {
            throw new HttpConfigException(e);
        }

        StringBuffer sb = new StringBuffer();
        int i;

        try {
            while ((i = fileReader.read()) != -1) {
                sb.append((char) i);
            }
        } catch(IOException e) {
            e.printStackTrace();
        }

        JsonNode conf = null;
        try {
            conf = JSON.parse(sb.toString());
        } catch(IOException e) {
            throw new HttpConfigException("[-] Error Parsing the Config File :/ ...");
        }
        myCurrentConfiguration = JSON.fromJson(conf, Configuration.class);
    }

    // Returns the current loaded configuration file
    public Configuration getCurrentConfiguration() {
        if(myCurrentConfiguration == null) {
            throw new HttpConfigException("No current Config set.");
        }
        return myCurrentConfiguration;
    }
}
