export interface Tools {
    tools: Tool[];
}

export interface Tool {
    description: string;
    short_description: string;
    inputSchema: InputSchema;
    name:        string;
    server_id:   string;
    installed:   boolean;
    categories: string[];
}

export interface InputSchema {
    description: string;
    properties:  Properties;
    required:    string[];
    title?:      string;
    type:        string;
    additionalProperties?: boolean;
    $schema?:    string;
}

export interface Properties {
    [key: string]: PropertyDefinition;
}

export interface PropertyDefinition {
    default?: any;
    description?: string;
    exclusiveMaximum?: number;
    exclusiveMinimum?: number;
    minimum?: number;
    maximum?: number;
    title?: string;
    type?: string | string[];
    format?: string;
    additionalProperties?: boolean;
    allOf?: PropertyDefinition[];
}

export interface RegistryTool {
    id:              string;
    name:            string;
    description:     string;
    short_description: string;
    publisher:       Publisher;
    isOfficial:      boolean;
    sourceUrl:       string;
    distribution:    Distribution;
    license:         string;
    runtime:         string;
    config:          Config;
    categories:      string[];
    tags:            string[];
}

export interface Config {
    command: string;
    args:    string[];
    env:     Env;
}

export interface Env {
    HELIUS_API_KEY?:      HeliusAPIKey;
    REPLICATE_API_TOKEN?: ReplicateAPIToken;
    VIRUSTOTAL_API_KEY?:  ReplicateAPIToken;
}

export interface HeliusAPIKey {
    required:    boolean;
    description: string;
}

export interface ReplicateAPIToken {
    description: string;
    type:        string;
    required:    boolean;
}

export interface Distribution {
    type:    string;
    package: string;
}

export interface Publisher {
    id:   string;
    name: string;
    url:  string;
}
