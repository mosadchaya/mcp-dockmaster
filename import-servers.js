#!/usr/bin/env node

const sqlite3 = require('sqlite3').verbose();
const fs = require('fs');
const path = require('path');
const os = require('os');

// Dev database path (matching the dev server local path)
const dbPath = process.argv[3] || path.join(
  os.homedir(),
  'Library/Application Support/com.mcp-dockmaster.desktop.local/mcp_dockmaster.db'
);

const inputPath = process.argv[2] || 'exported-servers.json';

console.log(`Importing to: ${dbPath}`);
console.log(`Reading from: ${inputPath}`);

// Read the exported servers
const exportData = JSON.parse(fs.readFileSync(inputPath, 'utf8'));

const db = new sqlite3.Database(dbPath, (err) => {
  if (err) {
    console.error('Error opening database:', err);
    process.exit(1);
  }
});

// Create tables if they don't exist (based on the schema from libs/mcp-core)
db.serialize(() => {
  // Create servers table
  db.run(`
    CREATE TABLE IF NOT EXISTS servers (
      id TEXT PRIMARY KEY,
      name TEXT NOT NULL,
      description TEXT,
      tools_type TEXT NOT NULL,
      enabled BOOLEAN NOT NULL DEFAULT 1,
      entry_point TEXT,
      command TEXT,
      args TEXT,
      distribution_type TEXT,
      distribution_package TEXT,
      created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
      updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
      server_type TEXT DEFAULT 'package',
      executable_path TEXT,
      working_directory TEXT
    )
  `);

  // Create server_env table
  db.run(`
    CREATE TABLE IF NOT EXISTS server_env (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      server_id TEXT NOT NULL,
      env_key TEXT NOT NULL,
      env_value TEXT,
      env_description TEXT,
      env_required BOOLEAN DEFAULT 0,
      FOREIGN KEY (server_id) REFERENCES servers(id) ON DELETE CASCADE
    )
  `);

  // Create other required tables
  db.run(`
    CREATE TABLE IF NOT EXISTS server_tools (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      server_id TEXT NOT NULL,
      tool_name TEXT NOT NULL,
      tool_description TEXT,
      FOREIGN KEY (server_id) REFERENCES servers(id) ON DELETE CASCADE
    )
  `);

  db.run(`
    CREATE TABLE IF NOT EXISTS app_settings (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      setting_key TEXT UNIQUE NOT NULL,
      setting_value TEXT,
      created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
      updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    )
  `);

  console.log('✅ Database tables created/verified');

  // Import servers
  let imported = 0;
  const servers = exportData.servers || [];

  if (servers.length === 0) {
    console.log('No servers to import');
    db.close();
    return;
  }

  servers.forEach((server, index) => {
    const stmt = db.prepare(`
      INSERT OR REPLACE INTO servers (
        id, name, description, tools_type, enabled, entry_point, 
        command, args, distribution_type, distribution_package, server_type
      ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
    `);

    const argsJson = server.args ? JSON.stringify(server.args) : null;
    
    stmt.run([
      server.id,
      server.name,
      server.description,
      server.tools_type,
      server.enabled ? 1 : 0,
      server.entry_point,
      server.command,
      argsJson,
      server.distribution_type,
      server.distribution_package,
      'package' // Default to package type for exported servers
    ], function(err) {
      if (err) {
        console.error(`Error importing server ${server.id}:`, err);
        return;
      }

      // Import environment variables if they exist
      if (server.env_vars) {
        Object.entries(server.env_vars).forEach(([key, config]) => {
          const envStmt = db.prepare(`
            INSERT OR REPLACE INTO server_env (server_id, env_key, env_value, env_description, env_required)
            VALUES (?, ?, ?, ?, ?)
          `);
          
          const value = typeof config === 'string' ? config : config.value;
          const description = typeof config === 'object' ? config.description : null;
          const required = typeof config === 'object' ? (config.required ? 1 : 0) : 0;
          
          envStmt.run([server.id, key, value, description, required]);
          envStmt.finalize();
        });
      }

      imported++;
      if (imported === servers.length) {
        console.log(`✅ Import completed successfully!`);
        console.log(`Imported ${imported}/${servers.length} servers`);
        db.close();
      }
    });

    stmt.finalize();
  });
});