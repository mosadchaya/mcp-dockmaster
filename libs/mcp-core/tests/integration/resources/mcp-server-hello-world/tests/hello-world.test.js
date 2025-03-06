import { test } from 'node:test';
import assert from 'node:assert';
import { helloWorldHandler, helloWorldWithInputHandler, helloWorldWithConfigHandler } from '../build/handlers/hello-world.js';

test('helloWorldHandler should return hello world', async () => {
  const result = await helloWorldHandler();
  
  assert.strictEqual(result.isError, false);
  assert.strictEqual(result.content[0].text, 'hello world');
});

test('helloWorldWithInputHandler should return hello world with input', async () => {
  const result = await helloWorldWithInputHandler({
    message: 'test'
  });
  
  assert.strictEqual(result.isError, false);
  assert.strictEqual(result.content[0].text, 'hello world test');
});

test('helloWorldWithInputHandler should return hello world with empty string if no input is provided', async () => {
  const result = await helloWorldWithInputHandler({});
  
  assert.strictEqual(result.isError, false);
  assert.strictEqual(result.content[0].text, 'hello world ');
});

test('helloWorldWithConfigHandler should return hello configuration with config', async () => {
  const result = await helloWorldWithConfigHandler({
    config: 'test'
  });
  
  assert.strictEqual(result.isError, false);
  assert.strictEqual(result.content[0].text, 'hello configuration test');
});

test('helloWorldWithConfigHandler should return an error if no config is provided', async () => {
  const result = await helloWorldWithConfigHandler({});
  
  assert.strictEqual(result.isError, true);
  assert.strictEqual(result.content[0].text, 'Config is required');
});
