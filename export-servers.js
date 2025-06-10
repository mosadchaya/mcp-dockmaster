#!/usr/bin/env node

const sqlite3 = require('sqlite3').verbose();
const fs = require('fs');
const path = require('path');
const os = require('os');

// Database path
const dbPath = path.join(
  os.homedir(),
  'Library/Application Support/com.mcp-dockmaster.desktop/mcp_dockmaster.db'
);

const outputPath = process.argv[2] || 'exported-servers.json';

console.log(`Exporting from: ${dbPath}`);
console.log(`Output to: ${outputPath}`);

const db = new sqlite3.Database(dbPath, sqlite3.OPEN_READONLY, (err) => {
  if (err) {
    console.error('Error opening database:', err);
    process.exit(1);
  }
});

const exportData = {
  version: '1.0',
  servers: []
};

// Query all servers
db.all(`
  SELECT 
    id, name, description, tools_type, enabled, 
    entry_point, command, args, distribution_type, distribution_package
  FROM servers
`, [], (err, servers) => {
  if (err) {
    console.error('Error querying servers:', err);
    db.close();
    process.exit(1);
  }

  let pending = servers.length;
  if (pending === 0) {
    writeOutput();
    return;
  }

  servers.forEach(server => {
    // Parse args if it's a JSON string
    let parsedArgs = null;
    if (server.args) {
      try {
        parsedArgs = JSON.parse(server.args);
      } catch (e) {
        parsedArgs = [server.args];
      }
    }

    const exportedServer = {
      id: server.id,
      name: server.name,
      description: server.description,
      tools_type: server.tools_type,
      enabled: server.enabled === 1,
      entry_point: server.entry_point,
      command: server.command,
      args: parsedArgs,
      distribution_type: server.distribution_type,
      distribution_package: server.distribution_package,
      env_vars: {}
    };

    // Query environment variables for this server
    db.all(`
      SELECT env_key, env_value, env_description, env_required
      FROM server_env
      WHERE server_id = ?
    `, [server.id], (err, envVars) => {
      if (err) {
        console.error(`Error querying env vars for ${server.id}:`, err);
      } else {
        envVars.forEach(env => {
          exportedServer.env_vars[env.env_key] = {
            value: env.env_value,
            description: env.env_description,
            required: env.env_required === 1
          };
        });
      }

      exportData.servers.push(exportedServer);
      pending--;
      
      if (pending === 0) {
        writeOutput();
      }
    });
  });
});

function writeOutput() {
  try {
    fs.writeFileSync(outputPath, JSON.stringify(exportData, null, 2));
    console.log(`✅ Export completed successfully!`);
    console.log(`Found ${exportData.servers.length} servers`);
    db.close();
  } catch (err) {
    console.error('Error writing output:', err);
    db.close();
    process.exit(1);
  }
}