package com.kylebrady.httpServer.util;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.*;

import java.io.IOException;

public class JSON {

    private static ObjectMapper myObjectMapper = defaultObjectMapper();

    private static ObjectMapper defaultObjectMapper() {
        ObjectMapper oM = new ObjectMapper();
        oM.configure(DeserializationFeature.FAIL_ON_UNKNOWN_PROPERTIES, false);
        return oM;
    }

    public static JsonNode parse(String JsonFile)
            throws IOException {
        return myObjectMapper.readTree(JsonFile);
    }

    public static <TYPE> TYPE fromJson(JsonNode node, Class<TYPE> clazz)
        throws JsonProcessingException {
        return myObjectMapper.treeToValue(node, clazz);
    }

    public static JsonNode toJson(Object myObj) {
        return myObjectMapper.valueToTree(myObj);
    }

    public static String str(JsonNode node)
            throws JsonProcessingException {
        return genJson(node, false);
    }

    public static String strPretty(JsonNode node)
            throws JsonProcessingException {
        return genJson(node, true);
    }

    private static String genJson(Object o, boolean pretty)
            throws JsonProcessingException {
        ObjectWriter objectWriter = myObjectMapper.writer();
        if(pretty) {
            objectWriter = objectWriter.with(SerializationFeature.INDENT_OUTPUT);
        }
        return objectWriter.writeValueAsString(o);
    }
}
