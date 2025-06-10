import React, { useState } from "react";
// import { useTranslation } from "@mcp-dockmaster/i18n"; // Will be used for i18n in the future
import { Button } from "./ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "./ui/card";
import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle } from "./ui/dialog";
import { Input } from "./ui/input";
import { Label } from "./ui/label";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "./ui/select";
import { Textarea } from "./ui/textarea";
import { Badge } from "./ui/badge";
import { Plus, Github, FolderOpen, File } from "lucide-react";
import { invoke } from "@tauri-apps/api/core";
import { toast } from "sonner";
import MCPClient from "../lib/mcpClient";

interface CustomServerForm {
  name: string;
  description: string;
  server_type: string;
  runtime: string;
  command?: string;
  executable_path?: string;
  args?: string[];
  working_directory?: string;
  env_vars?: Record<string, string>;
}

const CustomServerRegistry: React.FC = () => {
  // const { t } = useTranslation(); // Currently unused, but may be needed for i18n
  
  // GitHub Import Modal State
  const [isGitHubImportModalOpen, setIsGitHubImportModalOpen] = useState(false);
  const [githubUrl, setGithubUrl] = useState("");
  const [importingServer, setImportingServer] = useState(false);
  const [importError, setImportError] = useState<string | null>(null);
  
  // Add Custom Server Modal State
  const [isAddServerModalOpen, setIsAddServerModalOpen] = useState(false);
  const [addingServer, setAddingServer] = useState(false);
  const [addServerError, setAddServerError] = useState<string | null>(null);
  const [customServerForm, setCustomServerForm] = useState<CustomServerForm>({
    name: "",
    description: "",
    server_type: "local",
    runtime: "node",
    command: "",
    executable_path: "",
    args: [],
    working_directory: "",
    env_vars: {},
  });
  
  // Environment variables state
  const [envVarKey, setEnvVarKey] = useState("");
  const [envVarValue, setEnvVarValue] = useState("");

  // GitHub Import Modal Functions
  const openGitHubImportModal = () => {
    setGithubUrl("");
    setImportError(null);
    setIsGitHubImportModalOpen(true);
  };
  
  const closeGitHubImportModal = () => {
    setIsGitHubImportModalOpen(false);
    setGithubUrl("");
    setImportError(null);
  };
  
  const importServerFromGitHub = async () => {
    if (!githubUrl?.trim()) {
      setImportError("GitHub URL is required");
      return;
    }
    
    if (!githubUrl?.startsWith("https://github.com/")) {
      setImportError("Please enter a valid GitHub repository URL");
      return;
    }
    
    setImportingServer(true);
    setImportError(null);
    
    try {
      const response = await MCPClient.importServerFromUrl(githubUrl);
      
      if (response.success) {
        closeGitHubImportModal();
        toast.success("Server imported successfully from GitHub!");
      } else {
        setImportError(response.message || "Failed to import server");
      }
    } catch (error) {
      console.error("Error importing server:", error);
      setImportError(`Import failed: ${error instanceof Error ? error.message : String(error)}`);
    } finally {
      setImportingServer(false);
    }
  };

  // Add Custom Server Modal Functions
  const openAddServerModal = () => {
    setCustomServerForm({
      name: "",
      description: "",
      server_type: "local",
      runtime: "node",
      command: "",
      executable_path: "",
      args: [],
      working_directory: "",
      env_vars: {},
    });
    setAddServerError(null);
    setIsAddServerModalOpen(true);
  };
  
  const closeAddServerModal = () => {
    setIsAddServerModalOpen(false);
    setAddServerError(null);
  };

  const addEnvironmentVariable = () => {
    if (!envVarKey.trim() || !envVarValue.trim()) return;
    
    setCustomServerForm(prev => ({
      ...prev,
      env_vars: {
        ...prev.env_vars,
        [envVarKey]: envVarValue,
      },
    }));
    
    setEnvVarKey("");
    setEnvVarValue("");
  };

  const removeEnvironmentVariable = (key: string) => {
    setCustomServerForm(prev => ({
      ...prev,
      env_vars: Object.fromEntries(
        Object.entries(prev.env_vars || {}).filter(([k]) => k !== key)
      ),
    }));
  };

  const addCustomServer = async () => {
    if (!customServerForm.name.trim() || !customServerForm.description.trim()) {
      setAddServerError("Name and description are required");
      return;
    }

    setAddingServer(true);
    setAddServerError(null);
    
    try {
      const response = await invoke("register_custom_server", {
        request: {
          name: customServerForm.name,
          description: customServerForm.description,
          server_type: customServerForm.server_type,
          runtime: customServerForm.runtime,
          command: customServerForm.command || null,
          executable_path: customServerForm.executable_path || null,
          args: customServerForm.args && customServerForm.args.length > 0 ? customServerForm.args : null,
          working_directory: customServerForm.working_directory || null,
          env_vars: Object.keys(customServerForm.env_vars || {}).length > 0 ? customServerForm.env_vars : null,
        },
      }) as { success: boolean; message?: string };

      if (response.success) {
        closeAddServerModal();
        toast.success("Custom server added successfully!");
      } else {
        setAddServerError(response.message || "Failed to add custom server");
      }
    } catch (error) {
      console.error("Error adding custom server:", error);
      setAddServerError(`Failed to add server: ${error instanceof Error ? error.message : String(error)}`);
    } finally {
      setAddingServer(false);
    }
  };

  return (
    <div className="mx-auto flex h-full w-full max-w-4xl flex-col gap-8 px-6 py-10 pb-4">
      <div className="flex flex-col space-y-1.5">
        <div className="flex justify-between items-center">
          <h1 className="text-2xl font-semibold tracking-tight">Custom Server Registry</h1>
          <div className="flex gap-2">
            <Button variant="outline" onClick={openGitHubImportModal}>
              <Github className="w-4 h-4 mr-2" />
              Import From Github
            </Button>
            <Button onClick={openAddServerModal}>
              <Plus className="w-4 h-4 mr-2" />
              Add Custom Server
            </Button>
          </div>
        </div>
        <p className="text-muted-foreground text-sm">
          Register and manage custom MCP servers from local filesystems, development environments, 
          or any custom configuration not available in the standard registry.
        </p>
      </div>

      {/* Feature Cards */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        <Card>
          <CardHeader className="pb-3">
            <CardTitle className="text-lg flex items-center gap-2">
              <File className="w-5 h-5" />
              Local Servers
            </CardTitle>
          </CardHeader>
          <CardContent>
            <CardDescription>
              Register servers from your local filesystem with custom executable paths and working directories.
            </CardDescription>
          </CardContent>
        </Card>
        
        <Card>
          <CardHeader className="pb-3">
            <CardTitle className="text-lg flex items-center gap-2">
              <Github className="w-5 h-5" />
              GitHub Import
            </CardTitle>
          </CardHeader>
          <CardContent>
            <CardDescription>
              Import MCP servers directly from GitHub repositories with automatic configuration detection.
            </CardDescription>
          </CardContent>
        </Card>
        
        <Card>
          <CardHeader className="pb-3">
            <CardTitle className="text-lg flex items-center gap-2">
              <FolderOpen className="w-5 h-5" />
              Custom Config
            </CardTitle>
          </CardHeader>
          <CardContent>
            <CardDescription>
              Configure servers with custom commands, environment variables, and runtime-specific settings.
            </CardDescription>
          </CardContent>
        </Card>
      </div>

      {/* GitHub Import Modal */}
      <Dialog open={isGitHubImportModalOpen} onOpenChange={setIsGitHubImportModalOpen}>
        <DialogContent className="sm:max-w-[500px]">
          <DialogHeader>
            <DialogTitle>Import MCP Server from GitHub</DialogTitle>
            <DialogDescription>
              Enter a GitHub repository URL to import a new MCP server. The repository should contain a package.json (for Node.js) or pyproject.toml (for Python) file.
            </DialogDescription>
          </DialogHeader>
          <div className="bg-amber-50 border border-amber-200 rounded-md p-3 mb-3">
            <p className="text-amber-800 text-sm">
              <strong>Note:</strong> We will attempt to extract required environment variables from the repository's README.md file. Please note that this process may not identify all required variables correctly.
            </p>
          </div>
          <div className="grid gap-4 py-4">
            <div className="space-y-2">
              <Label htmlFor="github-url">GitHub Repository URL</Label>
              <Input
                id="github-url"
                placeholder="https://github.com/owner/repo"
                value={githubUrl}
                onChange={(e: React.ChangeEvent<HTMLInputElement>) => setGithubUrl(e.target.value)}
                disabled={importingServer}
              />
              {importError && (
                <p className="text-destructive text-sm">{importError}</p>
              )}
            </div>
          </div>
          <DialogFooter>
            <Button variant="outline" onClick={closeGitHubImportModal} disabled={importingServer}>
              Cancel
            </Button>
            <Button onClick={importServerFromGitHub} disabled={!githubUrl.trim() || importingServer}>
              {importingServer ? "Importing..." : "Import"}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* Add Custom Server Modal */}
      <Dialog open={isAddServerModalOpen} onOpenChange={setIsAddServerModalOpen}>
        <DialogContent className="sm:max-w-[600px] max-h-[80vh] overflow-y-auto">
          <DialogHeader>
            <DialogTitle>Add Custom Server</DialogTitle>
            <DialogDescription>
              Configure a custom MCP server with your specific requirements and environment settings.
            </DialogDescription>
          </DialogHeader>
          
          <div className="grid gap-4 py-4">
            {/* Basic Information */}
            <div className="space-y-4">
              <div className="space-y-2">
                <Label htmlFor="server-name">Server Name *</Label>
                <Input
                  id="server-name"
                  placeholder="My Custom Server"
                  value={customServerForm.name}
                  onChange={(e) => setCustomServerForm(prev => ({ ...prev, name: e.target.value }))}
                  disabled={addingServer}
                />
              </div>
              
              <div className="space-y-2">
                <Label htmlFor="server-description">Description *</Label>
                <Textarea
                  id="server-description"
                  placeholder="Describe what this server does..."
                  value={customServerForm.description}
                  onChange={(e) => setCustomServerForm(prev => ({ ...prev, description: e.target.value }))}
                  disabled={addingServer}
                />
              </div>
              
              <div className="grid grid-cols-2 gap-4">
                <div className="space-y-2">
                  <Label htmlFor="server-type">Server Type</Label>
                  <Select 
                    value={customServerForm.server_type} 
                    onValueChange={(value: string) => setCustomServerForm(prev => ({ ...prev, server_type: value }))}
                  >
                    <SelectTrigger>
                      <SelectValue placeholder="Select server type" />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="package">Package</SelectItem>
                      <SelectItem value="local">Local</SelectItem>
                      <SelectItem value="custom">Custom</SelectItem>
                    </SelectContent>
                  </Select>
                </div>
                
                <div className="space-y-2">
                  <Label htmlFor="runtime">Runtime</Label>
                  <Select 
                    value={customServerForm.runtime} 
                    onValueChange={(value: string) => setCustomServerForm(prev => ({ ...prev, runtime: value }))}
                  >
                    <SelectTrigger>
                      <SelectValue placeholder="Select runtime" />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="node">Node.js</SelectItem>
                      <SelectItem value="python">Python</SelectItem>
                      <SelectItem value="docker">Docker</SelectItem>
                      <SelectItem value="custom">Custom</SelectItem>
                    </SelectContent>
                  </Select>
                </div>
              </div>
            </div>

            {/* Command Configuration */}
            <div className="space-y-4">
              <div className="space-y-2">
                <Label htmlFor="command">Command (optional)</Label>
                <Input
                  id="command"
                  placeholder="node, python, uv, etc."
                  value={customServerForm.command}
                  onChange={(e) => setCustomServerForm(prev => ({ ...prev, command: e.target.value }))}
                  disabled={addingServer}
                />
              </div>
              
              <div className="space-y-2">
                <Label htmlFor="executable-path">Executable Path (optional)</Label>
                <Input
                  id="executable-path"
                  placeholder="/path/to/executable or ./relative/path"
                  value={customServerForm.executable_path}
                  onChange={(e) => setCustomServerForm(prev => ({ ...prev, executable_path: e.target.value }))}
                  disabled={addingServer}
                />
              </div>
              
              <div className="space-y-2">
                <Label htmlFor="working-directory">Working Directory (optional)</Label>
                <Input
                  id="working-directory"
                  placeholder="/path/to/working/directory"
                  value={customServerForm.working_directory}
                  onChange={(e) => setCustomServerForm(prev => ({ ...prev, working_directory: e.target.value }))}
                  disabled={addingServer}
                />
              </div>
            </div>

            {/* Environment Variables */}
            <div className="space-y-4">
              <Label>Environment Variables</Label>
              <div className="space-y-2">
                <div className="flex gap-2">
                  <Input
                    placeholder="Variable name"
                    value={envVarKey}
                    onChange={(e) => setEnvVarKey(e.target.value)}
                    disabled={addingServer}
                  />
                  <Input
                    placeholder="Variable value"
                    value={envVarValue}
                    onChange={(e) => setEnvVarValue(e.target.value)}
                    disabled={addingServer}
                  />
                  <Button 
                    type="button" 
                    variant="outline" 
                    size="sm"
                    onClick={addEnvironmentVariable}
                    disabled={!envVarKey.trim() || !envVarValue.trim() || addingServer}
                  >
                    Add
                  </Button>
                </div>
                
                {/* Display existing environment variables */}
                <div className="flex flex-wrap gap-2">
                  {Object.entries(customServerForm.env_vars || {}).map(([key, value]) => (
                    <Badge key={key} variant="secondary" className="flex items-center gap-1">
                      {key}={value}
                      <button
                        type="button"
                        onClick={() => removeEnvironmentVariable(key)}
                        className="ml-1 text-xs hover:text-destructive"
                        disabled={addingServer}
                      >
                        ×
                      </button>
                    </Badge>
                  ))}
                </div>
              </div>
            </div>

            {addServerError && (
              <p className="text-destructive text-sm">{addServerError}</p>
            )}
          </div>
          
          <DialogFooter>
            <Button variant="outline" onClick={closeAddServerModal} disabled={addingServer}>
              Cancel
            </Button>
            <Button onClick={addCustomServer} disabled={addingServer}>
              {addingServer ? "Adding..." : "Add Server"}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
};

export default CustomServerRegistry;