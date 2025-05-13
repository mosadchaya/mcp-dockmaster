import * as fs from 'node:fs';
import * as path from 'node:path';

/**
 * Reads a JSON file synchronously.
 * @param filepath - Absolute path to the JSON file.
 * @returns Parsed JSON object.
 */
export function readJSON<T>(filepath: string): T {
  const content = fs.readFileSync(filepath, 'utf-8');
  return JSON.parse(content) as T;
}

/**
 * Writes an object to a JSON file synchronously with pretty printing.
 * @param filepath - Absolute path to the target JSON file.
 * @param data - The object to write.
 */
export function writeJSON(filepath: string, data: any): void {
  const dir = path.dirname(filepath);
  if (!fs.existsSync(dir)) {
    fs.mkdirSync(dir, { recursive: true });
  }
  const content = JSON.stringify(data, null, 2) + '\n'; // Add trailing newline
  fs.writeFileSync(filepath, content, 'utf-8');
}

/**
 * Simple console divider.
 * @param title - Optional title for the divider.
 */
export function divider(title = '') {
  const line = '-'.repeat(50);
  console.log(`\n${line}`);
  if (title) {
    console.log(title);
  }
  console.log(`${line}\n`);
} 