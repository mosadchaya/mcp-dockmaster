# MCP Server Hello World

A Model Context Protocol server example with hello world methods.

## Installation

```bash
npm install
```

## Usage

```bash
npm run build
npm run inspector
```

## Testing

```bash
npm test
```

## Methods

### hello_world

Returns "hello world".

### hello_world_with_input

Returns "hello world" with the provided input.

Input:
- message (string, optional): The input to append to "hello world".

### hello_world_with_config

Returns "hello configuration" with the provided config.

Input:
- config (string, required): The configuration to append to "hello configuration".
```
