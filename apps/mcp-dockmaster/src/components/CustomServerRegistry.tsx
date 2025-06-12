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
import { Plus, Github, Download, FolderOpen, File } from "lucide-react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
// import { exists } from "@tauri-apps/plugin-fs"; // Removed due to permission issues
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
  env_vars?: Record<string, CustomEnvConfig>;
}

interface CustomEnvConfig {
  value: string;
  description: string;
  required: boolean;
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
  
  // Runtime Selection Dialog State
  const [isRuntimeSelectionOpen, setIsRuntimeSelectionOpen] = useState(false);
  const [detectedRuntimes, setDetectedRuntimes] = useState<Array<{ runtime: string; command: string; priority: number }>>([]);
  const [selectedRuntimeIndex, setSelectedRuntimeIndex] = useState(0);
  
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
  
  // Environment Variables Guidance Dialog State
  const [showEnvVarsDialog, setShowEnvVarsDialog] = useState(false);
  const [envVarDialogValues, setEnvVarDialogValues] = useState<Record<string, string>>({});
  
  // Environment variables configuration state
  const [envVarKey, setEnvVarKey] = useState("");
  const [envVarValue, setEnvVarValue] = useState("");
  const [envVarDescription, setEnvVarDescription] = useState("");
  const [envVarRequired, setEnvVarRequired] = useState(false);
  
  // Environment variable templates
  const envVarTemplates = [
    { name: "API Key", key: "API_KEY", value: "", description: "API authentication key", required: true },
    { name: "API URL", key: "API_URL", value: "https://api.example.com", description: "Base API endpoint URL", required: false },
    { name: "Database URL", key: "DATABASE_URL", value: "sqlite://./database.db", description: "Database connection string", required: false },
    { name: "Port", key: "PORT", value: "3000", description: "Server port number", required: false },
    { name: "Debug Mode", key: "DEBUG", value: "false", description: "Enable debug logging", required: false },
    { name: "Log Level", key: "LOG_LEVEL", value: "info", description: "Logging level (error, warn, info, debug)", required: false },
    { name: "Working Directory", key: "WORKDIR", value: "./", description: "Working directory path", required: false },
    { name: "Config Path", key: "CONFIG_PATH", value: "./config.json", description: "Configuration file path", required: false },
  ];

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

  // Auto-detect environment variables from GitHub URL
  const analyzeGitHubRepository = async (url: string) => {
    if (!url?.startsWith("https://github.com/")) {
      return;
    }

    try {
      toast.info("Analyzing repository for environment variables...");
      const envVars = await MCPClient.analyzeGitHubRepository(url);
      
      if (envVars.length > 0) {
        // Convert to CustomEnvConfig format and add to form
        const newEnvVars: Record<string, CustomEnvConfig> = {};
        envVars.forEach(envVar => {
          newEnvVars[envVar.key] = {
            value: envVar.value,
            description: envVar.description,
            required: envVar.required,
          };
        });

        setCustomServerForm(prev => ({
          ...prev,
          env_vars: {
            ...prev.env_vars,
            ...newEnvVars,
          },
        }));

        toast.success(`Found ${envVars.length} environment variables! Added to your server configuration.`);
      } else {
        toast.info("No environment variables detected in repository.");
      }
    } catch (error) {
      console.error("Error analyzing repository:", error);
      toast.error("Failed to analyze repository for environment variables.");
    }
  };

  // Read and display README content from local directory
  const [readmeContent, setReadmeContent] = useState<string>("");

  const readLocalReadme = async (directory?: string) => {
    const targetDirectory = directory || customServerForm.working_directory;
    
    if (!targetDirectory?.trim()) {
      toast.error("Please specify a working directory first.");
      return;
    }

    try {
      // Try to read README files from the directory
      const readmeFiles = ["README.md", "readme.md", "Readme.md", "README.txt", "readme.txt"];
      let foundContent = "";
      
      for (const filename of readmeFiles) {
        try {
          const filePath = `${targetDirectory}/${filename}`;
          const content = await invoke('read_text_file', { path: filePath }) as string;
          if (content.trim()) {
            foundContent = content;
            break;
          }
        } catch (error) {
          // File doesn't exist, try next one
          continue;
        }
      }

      if (foundContent) {
        setReadmeContent(foundContent);
        toast.success("README content loaded successfully!");
      } else {
        setReadmeContent("No README file found in the specified directory.");
        toast.info("No README file found in directory.");
      }
    } catch (error) {
      console.error("Error reading README:", error);
      setReadmeContent("Failed to read README file.");
      toast.error("Failed to read README file.");
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

  // Auto-detect runtime and command based on executable path
  const detectRuntimeFromPath = (path: string): { runtime: string; command: string; needsUvCheck?: boolean } => {
    const lowerPath = path.toLowerCase();
    const filename = path.split('/').pop() || '';
    
    // Node.js detection
    if (lowerPath.endsWith('.js') || lowerPath.endsWith('.mjs') || lowerPath.endsWith('.cjs')) {
      return { runtime: 'node', command: 'node' };
    }
    
    // Python detection
    if (lowerPath.endsWith('.py')) {
      // For Python files, we should check if the project uses uv
      // This will be done when we have the working directory
      return { runtime: 'python', command: 'python', needsUvCheck: true };
    }
    
    // Docker detection
    if (filename === 'dockerfile' || filename.startsWith('docker-compose')) {
      return { runtime: 'docker', command: 'docker' };
    }
    
    // Binary/executable detection
    if (!filename.includes('.') || lowerPath.endsWith('.sh') || lowerPath.endsWith('.exe')) {
      return { runtime: 'custom', command: '' };
    }
    
    // Default
    return { runtime: 'custom', command: '' };
  };

  // Auto-detect runtime and command based on working directory
  const detectRuntimeFromDirectory = async (directory: string): Promise<{ runtime: string; command: string; alternatives?: string[] } | null> => {
    try {
      const detectedRuntimes: Array<{ runtime: string; command: string; priority: number }> = [];
      
      // Check for uv project (Python with uv) - highest priority for Python
      const isUvProject = await checkForUvProject(directory);
      if (isUvProject) {
        detectedRuntimes.push({ runtime: 'python', command: 'uv', priority: 1 });
      }
      
      // Check for Node.js project - high priority for development
      const nodeProject = await checkForNodeProject(directory);
      if (nodeProject) {
        detectedRuntimes.push({ runtime: 'node', command: nodeProject, priority: 2 });
      }
      
      // Check for Docker project - lower priority (usually for deployment)
      const dockerProject = await checkForDockerProject(directory);
      if (dockerProject) {
        detectedRuntimes.push({ runtime: 'docker', command: dockerProject, priority: 3 });
      }
      
      if (detectedRuntimes.length === 0) {
        return null;
      }
      
      // Sort by priority
      detectedRuntimes.sort((a, b) => a.priority - b.priority);
      
      // If multiple runtimes detected, show selection dialog
      if (detectedRuntimes.length > 1) {
        setDetectedRuntimes(detectedRuntimes);
        setSelectedRuntimeIndex(0); // Default to highest priority (first item)
        setIsRuntimeSelectionOpen(true);
        return null; // Return null to pause the auto-detection, user will select via dialog
      }
      
      // Single runtime detected - auto-select and show simple toast
      const single = detectedRuntimes[0];
      toast.success(`Detected ${single.runtime} project`);
      
      return { 
        runtime: single.runtime, 
        command: single.command
      };
    } catch (error) {
      console.error('Error detecting runtime from directory:', error);
      return null;
    }
  };

  // Check if a directory contains uv.lock file (for Python projects)
  const checkForUvProject = async (directory: string): Promise<boolean> => {
    try {
      // Use a Tauri command to check for uv project files
      const result = await invoke('check_uv_project', { directory }) as boolean;
      return result;
    } catch (error) {
      console.error('Error checking for uv project:', error);
      return false;
    }
  };

  // Check if a directory contains Node.js project files
  const checkForNodeProject = async (directory: string): Promise<string | null> => {
    try {
      // Use a Tauri command to check for Node.js project files and get suggested command
      const result = await invoke('check_node_project', { directory }) as string | null;
      return result;
    } catch (error) {
      console.error('Error checking for Node.js project:', error);
      return null;
    }
  };

  // Check if a directory contains Docker project files
  const checkForDockerProject = async (directory: string): Promise<string | null> => {
    try {
      // Use a Tauri command to check for Docker project files and get suggested command
      const result = await invoke('check_docker_project', { directory }) as string | null;
      return result;
    } catch (error) {
      console.error('Error checking for Docker project:', error);
      return null;
    }
  };

  // File browser functions
  const openFileDialog = async (fieldName: "executable_path" | "working_directory") => {
    try {
      const result = fieldName === "working_directory" 
        ? await open({ directory: true, multiple: false })
        : await open({ multiple: false });
      
      if (result && typeof result === 'string') {
        if (fieldName === "executable_path") {
          // Auto-detect runtime and command when executable path is selected
          const { runtime, command, needsUvCheck } = detectRuntimeFromPath(result);
          
          // For Python projects, check if we should use uv
          let finalCommand = command;
          if (needsUvCheck && customServerForm.working_directory) {
            const isUvProject = await checkForUvProject(customServerForm.working_directory);
            if (isUvProject) {
              finalCommand = 'uv';
            }
          }
          
          setCustomServerForm(prev => ({
            ...prev,
            [fieldName]: result,
            runtime: runtime,
            command: finalCommand || prev.command,
          }));
          
          // Show toast notification about auto-detection
          if (runtime !== 'custom') {
            toast.success(`Auto-detected ${runtime} runtime${finalCommand === 'uv' ? ' with uv' : ''}`);
          }
        } else if (fieldName === "working_directory") {
          // When working directory is set, auto-detect runtime from directory
          setCustomServerForm(prev => ({
            ...prev,
            [fieldName]: result,
          }));
          
          // Auto-detect runtime and command from directory
          const detected = await detectRuntimeFromDirectory(result);
          if (detected) {
            setCustomServerForm(prev => ({
              ...prev,
              runtime: detected.runtime,
              command: detected.command,
            }));
            toast.success(`Auto-detected ${detected.runtime} runtime with ${detected.command}`);
          }
          
          // Also read README for reference
          setTimeout(() => {
            readLocalReadme(result);
          }, 1000); // Delay to let runtime detection finish
        } else {
          setCustomServerForm(prev => ({
            ...prev,
            [fieldName]: result,
          }));
        }
      }
    } catch (error) {
      console.error("Error opening file dialog:", error);
      toast.error("Failed to open file dialog");
    }
  };

  const addEnvironmentVariable = () => {
    if (!envVarKey.trim()) return;
    
    setCustomServerForm(prev => ({
      ...prev,
      env_vars: {
        ...prev.env_vars,
        [envVarKey]: {
          value: envVarValue,
          description: envVarDescription,
          required: envVarRequired,
        },
      },
    }));
    
    setEnvVarKey("");
    setEnvVarValue("");
    setEnvVarDescription("");
    setEnvVarRequired(false);
  };

  const addTemplateEnvVar = (template: typeof envVarTemplates[0]) => {
    setEnvVarKey(template.key);
    setEnvVarValue(template.value);
    setEnvVarDescription(template.description);
    setEnvVarRequired(template.required);
  };

  const removeEnvironmentVariable = (key: string) => {
    setCustomServerForm(prev => ({
      ...prev,
      env_vars: Object.fromEntries(
        Object.entries(prev.env_vars || {}).filter(([k]) => k !== key)
      ),
    }));
  };

  // Environment Variables Guidance Dialog Functions
  const hasEnvVars = () => {
    return customServerForm.env_vars && Object.keys(customServerForm.env_vars).length > 0;
  };

  const openEnvVarsDialog = () => {
    if (!hasEnvVars()) {
      // No environment variables, proceed directly
      performServerRegistration();
      return;
    }

    // Initialize dialog values with defaults
    const initialValues: Record<string, string> = {};
    Object.entries(customServerForm.env_vars || {}).forEach(([key, config]) => {
      initialValues[key] = config.value || "";
    });

    setEnvVarDialogValues(initialValues);
    setShowEnvVarsDialog(true);
  };

  const handleEnvVarDialogChange = (key: string, value: string) => {
    setEnvVarDialogValues(prev => ({
      ...prev,
      [key]: value,
    }));
  };

  const isEnvVarDialogValid = () => {
    if (!customServerForm.env_vars) return true;
    
    // Check if all required environment variables have values
    return Object.entries(customServerForm.env_vars).every(([key, config]) => {
      if (config.required) {
        const value = envVarDialogValues[key];
        return value && value.trim() !== "";
      }
      return true;
    });
  };

  const addCustomServer = async () => {
    if (!customServerForm.name.trim() || !customServerForm.description.trim()) {
      setAddServerError("Name and description are required");
      return;
    }

    // Check if environment variables need configuration
    openEnvVarsDialog();
  };

  const performServerRegistration = async (finalEnvVars?: Record<string, string>) => {
    setAddingServer(true);
    setAddServerError(null);
    
    try {
      // Convert environment variables to the format expected by backend
      let envVarsForBackend: Record<string, string> | null = null;
      
      if (customServerForm.env_vars && Object.keys(customServerForm.env_vars).length > 0) {
        envVarsForBackend = {};
        Object.entries(customServerForm.env_vars).forEach(([key, config]) => {
          // Use dialog values if provided, otherwise use default values
          const value = finalEnvVars?.[key] || config.value || "";
          envVarsForBackend![key] = value;
        });
      }

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
          env_vars: envVarsForBackend,
        },
      }) as { success: boolean; message?: string };

      if (response.success) {
        closeAddServerModal();
        setShowEnvVarsDialog(false);
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
        <h1 className="text-2xl font-semibold tracking-tight">Custom Server Registry</h1>
        <p className="text-muted-foreground text-sm">
          Register and manage custom MCP servers from local filesystems, development environments, 
          or any custom configuration not available in the standard registry.
        </p>
      </div>

      {/* Feature Cards */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        <Card className="flex flex-col h-full">
          <CardHeader className="pb-3">
            <CardTitle className="text-lg flex items-center gap-2">
              <Github className="w-5 h-5" />
              GitHub Import
            </CardTitle>
          </CardHeader>
          <CardContent className="flex flex-col flex-1 justify-between">
            <CardDescription className="mb-4">
              Import MCP servers directly from GitHub repositories with automatic configuration detection.
            </CardDescription>
            <div className="flex justify-start mt-auto">
              <Button 
                variant="outline" 
                className="bg-background border-input hover:bg-accent hover:text-accent-foreground"
                onClick={openGitHubImportModal}
              >
                <Github className="w-4 h-4 mr-2" />
                Import From Github
              </Button>
            </div>
          </CardContent>
        </Card>
        
        <Card className="flex flex-col h-full">
          <CardHeader className="pb-3">
            <CardTitle className="text-lg flex items-center gap-2">
              <Download className="w-5 h-5" />
              Custom Servers
            </CardTitle>
          </CardHeader>
          <CardContent className="flex flex-col flex-1 justify-between">
            <CardDescription className="mb-4">
              Register any custom server: local filesystem, development environments, or custom configurations with runtime-specific settings.
            </CardDescription>
            <div className="flex justify-start mt-auto">
              <Button 
                variant="outline" 
                className="bg-background border-input hover:bg-accent hover:text-accent-foreground"
                onClick={openAddServerModal}
              >
                <Plus className="w-4 h-4 mr-2" />
                Add Custom Server
              </Button>
            </div>
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
          <div className="bg-blue-50 border border-blue-200 rounded-md p-3 mb-3">
            <p className="text-blue-800 text-sm">
              <strong>Enhanced Detection:</strong> We automatically analyze the repository for environment variables from README.md and .env.example files. You can also preview detected variables before importing.
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
            <Button 
              variant="secondary" 
              onClick={() => analyzeGitHubRepository(githubUrl)} 
              disabled={!githubUrl.trim() || importingServer}
            >
              Preview Variables
            </Button>
            <Button onClick={importServerFromGitHub} disabled={!githubUrl.trim() || importingServer}>
              {importingServer ? "Importing..." : "Import"}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* Add Custom Server Modal */}
      <Dialog open={isAddServerModalOpen} onOpenChange={setIsAddServerModalOpen}>
        <DialogContent className="sm:max-w-[650px] max-h-[80vh] overflow-y-auto">
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
                <Label htmlFor="command">Command</Label>
                <Input
                  id="command"
                  placeholder="node, python, uv, etc."
                  value={customServerForm.command}
                  onChange={(e) => setCustomServerForm(prev => ({ ...prev, command: e.target.value }))}
                  disabled={addingServer}
                />
              </div>
              
              <div className="space-y-2">
                <Label htmlFor="executable-path">Executable Path</Label>
                <div className="flex gap-2">
                  <Input
                    id="executable-path"
                    placeholder="/path/to/executable or ./relative/path"
                    value={customServerForm.executable_path}
                    onChange={(e) => setCustomServerForm(prev => ({ ...prev, executable_path: e.target.value }))}
                    onBlur={async (e) => {
                      // Auto-detect runtime when user finishes typing
                      if (e.target.value) {
                        const { runtime, command, needsUvCheck } = detectRuntimeFromPath(e.target.value);
                        
                        // For Python projects, check if we should use uv
                        let finalCommand = command;
                        if (needsUvCheck && customServerForm.working_directory) {
                          const isUvProject = await checkForUvProject(customServerForm.working_directory);
                          if (isUvProject) {
                            finalCommand = 'uv';
                          }
                        }
                        
                        setCustomServerForm(prev => ({
                          ...prev,
                          runtime: runtime,
                          command: finalCommand || prev.command,
                        }));
                      }
                    }}
                    disabled={addingServer}
                  />
                  <Button
                    type="button"
                    variant="outline"
                    size="sm"
                    onClick={() => openFileDialog("executable_path")}
                    disabled={addingServer}
                    className="px-3"
                  >
                    <File className="w-4 h-4" />
                  </Button>
                </div>
              </div>
              
              <div className="space-y-2">
                <Label htmlFor="working-directory">Working Directory</Label>
                <div className="flex gap-2">
                  <Input
                    id="working-directory"
                    placeholder="/path/to/working/directory"
                    value={customServerForm.working_directory}
                    onChange={(e) => setCustomServerForm(prev => ({ ...prev, working_directory: e.target.value }))}
                    onBlur={async (e) => {
                      // Auto-detect runtime when user finishes typing directory path
                      if (e.target.value) {
                        const detected = await detectRuntimeFromDirectory(e.target.value);
                        if (detected) {
                          setCustomServerForm(prev => ({
                            ...prev,
                            runtime: detected.runtime,
                            command: detected.command,
                          }));
                          toast.success(`Auto-detected ${detected.runtime} runtime with ${detected.command}`);
                        }
                      }
                    }}
                    disabled={addingServer}
                  />
                  <Button
                    type="button"
                    variant="outline"
                    size="sm"
                    onClick={() => openFileDialog("working_directory")}
                    disabled={addingServer}
                    className="px-3"
                  >
                    <FolderOpen className="w-4 h-4" />
                  </Button>
                </div>
              </div>
            </div>

            {/* GitHub Repository Analysis */}
            <div className="space-y-4">
              <div className="flex items-center justify-between">
                <Label>GitHub Repository (Optional)</Label>
              </div>
              <div className="flex gap-2">
                <Input
                  placeholder="https://github.com/owner/repo"
                  value={githubUrl}
                  onChange={(e) => setGithubUrl(e.target.value)}
                  disabled={addingServer}
                />
                <Button 
                  type="button" 
                  variant="outline" 
                  size="sm"
                  onClick={() => analyzeGitHubRepository(githubUrl)}
                  disabled={!githubUrl.trim() || addingServer}
                >
                  Analyze
                </Button>
              </div>
              <p className="text-sm text-muted-foreground">
                Automatically detect environment variables from GitHub repository README and .env.example files.
              </p>
            </div>

            {/* Environment Variables */}
            <div className="space-y-4">
              <div className="flex items-center justify-between">
                <Label>Environment Variables</Label>
                <Select onValueChange={(value) => {
                  const template = envVarTemplates.find(t => t.key === value);
                  if (template) addTemplateEnvVar(template);
                }}>
                  <SelectTrigger className="w-40">
                    <SelectValue placeholder="Add template" />
                  </SelectTrigger>
                  <SelectContent>
                    {envVarTemplates.map((template) => (
                      <SelectItem key={template.key} value={template.key}>
                        {template.name}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
              <p className="text-sm text-muted-foreground">
                Add environment variables required by your server. Use the template dropdown for common patterns or add custom variables manually.
              </p>
              
              {/* README Content Display */}
              {readmeContent && (
                <div className="border rounded-md p-3 bg-muted/30">
                  <div className="flex items-center justify-between mb-2">
                    <Label className="text-sm font-medium">README Content</Label>
                    <Button
                      type="button"
                      variant="ghost"
                      size="sm"
                      onClick={() => setReadmeContent("")}
                      className="h-6 w-6 p-0"
                    >
                      ×
                    </Button>
                  </div>
                  <div className="max-h-40 overflow-y-auto bg-background border rounded p-3 text-xs font-mono whitespace-pre-wrap">
                    {readmeContent}
                  </div>
                </div>
              )}
              <div className="space-y-3">
                <div className="grid grid-cols-2 gap-2">
                  <Input
                    placeholder="Variable name"
                    value={envVarKey}
                    onChange={(e) => setEnvVarKey(e.target.value)}
                    disabled={addingServer}
                  />
                  <Input
                    placeholder="Default value"
                    value={envVarValue}
                    onChange={(e) => setEnvVarValue(e.target.value)}
                    disabled={addingServer}
                  />
                </div>
                <Input
                  placeholder="Description (optional)"
                  value={envVarDescription}
                  onChange={(e) => setEnvVarDescription(e.target.value)}
                  disabled={addingServer}
                />
                <div className="flex items-center justify-between">
                  <div className="flex items-center space-x-2">
                    <input
                      type="checkbox"
                      id="env-required"
                      checked={envVarRequired}
                      onChange={(e) => setEnvVarRequired(e.target.checked)}
                      disabled={addingServer}
                      className="w-4 h-4"
                    />
                    <Label htmlFor="env-required" className="text-sm">
                      Required variable
                    </Label>
                  </div>
                  <Button 
                    type="button" 
                    variant="outline" 
                    size="sm"
                    onClick={addEnvironmentVariable}
                    disabled={!envVarKey.trim() || addingServer}
                  >
                    Add
                  </Button>
                </div>
              </div>
                
                {/* Display manually added environment variables */}
                <div className="space-y-2">
                  {Object.entries(customServerForm.env_vars || {}).map(([key, config]) => (
                    <div key={key} className="border rounded-md p-3 space-y-2">
                      <div className="flex items-center justify-between">
                        <div className="flex items-center gap-2">
                          <span className="font-medium">{key}</span>
                          {config.required ? (
                            <Badge variant="outline" className="bg-amber-100 text-amber-800 border-amber-300 text-xs">
                              Required
                            </Badge>
                          ) : (
                            <Badge variant="outline" className="bg-slate-100 text-slate-800 border-slate-300 text-xs">
                              Optional
                            </Badge>
                          )}
                        </div>
                        <button
                          type="button"
                          onClick={() => removeEnvironmentVariable(key)}
                          className="text-xs hover:text-destructive p-1"
                          disabled={addingServer}
                        >
                          ×
                        </button>
                      </div>
                      {config.description && (
                        <div className="text-sm text-muted-foreground">{config.description}</div>
                      )}
                      <div className="space-y-1">
                        <Input
                          placeholder={config.value || `Enter value for ${key}`}
                          value={config.value || ""}
                          onChange={(e) => {
                            setCustomServerForm(prev => ({
                              ...prev,
                              env_vars: {
                                ...prev.env_vars,
                                [key]: {
                                  ...config,
                                  value: e.target.value,
                                },
                              },
                            }));
                          }}
                          disabled={addingServer}
                        />
                      </div>
                    </div>
                  ))}
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

      {/* Runtime Selection Dialog */}
      <Dialog open={isRuntimeSelectionOpen} onOpenChange={setIsRuntimeSelectionOpen}>
        <DialogContent className="sm:max-w-[500px]">
          <DialogHeader>
            <DialogTitle>Multiple Runtimes Detected</DialogTitle>
            <DialogDescription>
              We detected multiple ways to run this project. Please select which runtime you'd like to use:
            </DialogDescription>
          </DialogHeader>
          <div className="grid gap-4 py-4">
            <div className="space-y-3">
              {detectedRuntimes.map((runtime, index) => (
                <div key={index} className="flex items-center space-x-2">
                  <input
                    type="radio"
                    id={`runtime-${index}`}
                    name="runtime-selection"
                    checked={selectedRuntimeIndex === index}
                    onChange={() => setSelectedRuntimeIndex(index)}
                    className="w-4 h-4 text-blue-600"
                  />
                  <label htmlFor={`runtime-${index}`} className="flex-1 cursor-pointer">
                    <div className="flex items-center justify-between">
                      <div>
                        <span className="font-medium capitalize">{runtime.runtime}</span>
                        <span className="text-muted-foreground ml-2">({runtime.command})</span>
                      </div>
                      {index === 0 && (
                        <Badge variant="secondary" className="text-xs">Recommended</Badge>
                      )}
                    </div>
                  </label>
                </div>
              ))}
            </div>
          </div>
          <DialogFooter>
            <Button variant="outline" onClick={() => setIsRuntimeSelectionOpen(false)}>
              Cancel
            </Button>
            <Button 
              onClick={() => {
                const selected = detectedRuntimes[selectedRuntimeIndex];
                setCustomServerForm(prev => ({
                  ...prev,
                  runtime: selected.runtime,
                  command: selected.command
                }));
                setIsRuntimeSelectionOpen(false);
                toast.success(`Selected ${selected.runtime} runtime`);
              }}
            >
              Use Selected Runtime
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* Environment Variables Guidance Dialog */}
      <Dialog open={showEnvVarsDialog} onOpenChange={setShowEnvVarsDialog}>
        <DialogContent className="sm:max-w-[600px]">
          <DialogHeader>
            <DialogTitle>Configure Environment Variables - {customServerForm.name}</DialogTitle>
            <DialogDescription>
              {(() => {
                const requiredCount = Object.values(customServerForm.env_vars || {}).filter(config => config.required).length;
                const optionalCount = Object.values(customServerForm.env_vars || {}).filter(config => !config.required).length;
                
                if (requiredCount > 0 && optionalCount > 0) {
                  return `This server requires ${requiredCount} environment variable${requiredCount > 1 ? 's' : ''} and has ${optionalCount} optional variable${optionalCount > 1 ? 's' : ''}.`;
                } else if (requiredCount > 0) {
                  return `This server requires ${requiredCount} environment variable${requiredCount > 1 ? 's' : ''}.`;
                } else {
                  return `This server has ${optionalCount} optional environment variable${optionalCount > 1 ? 's' : ''}.`;
                }
              })()}
            </DialogDescription>
          </DialogHeader>
          
          <div className="grid gap-4 py-4 max-h-[60vh] overflow-y-auto">
            {/* Required Variables Section */}
            {(() => {
              const requiredVars = Object.entries(customServerForm.env_vars || {}).filter(([_, config]) => config.required);
              if (requiredVars.length === 0) return null;
              
              return (
                <div className="space-y-3">
                  <h3 className="text-sm font-medium text-red-700">Required Variables *</h3>
                  {requiredVars.map(([key, config]) => (
                    <div key={key} className="space-y-2">
                      <Label htmlFor={`env-dialog-${key}`} className="flex items-center gap-1">
                        {key}
                        <span className="text-red-500">*</span>
                      </Label>
                      <Input
                        id={`env-dialog-${key}`}
                        placeholder={config.description || key}
                        value={envVarDialogValues[key] || ""}
                        onChange={(e) => handleEnvVarDialogChange(key, e.target.value)}
                        disabled={addingServer}
                      />
                      {config.description && (
                        <div className="text-sm text-muted-foreground">{config.description}</div>
                      )}
                    </div>
                  ))}
                </div>
              );
            })()}
            
            {/* Optional Variables Section */}
            {(() => {
              const optionalVars = Object.entries(customServerForm.env_vars || {}).filter(([_, config]) => !config.required);
              if (optionalVars.length === 0) return null;
              
              return (
                <div className="space-y-3">
                  <h3 className="text-sm font-medium text-slate-700">Optional Variables</h3>
                  {optionalVars.map(([key, config]) => (
                    <div key={key} className="space-y-2">
                      <Label htmlFor={`env-dialog-${key}`}>{key}</Label>
                      <Input
                        id={`env-dialog-${key}`}
                        placeholder={config.description || config.value || key}
                        value={envVarDialogValues[key] || config.value || ""}
                        onChange={(e) => handleEnvVarDialogChange(key, e.target.value)}
                        disabled={addingServer}
                      />
                      {config.description && (
                        <div className="text-sm text-muted-foreground">{config.description}</div>
                      )}
                    </div>
                  ))}
                </div>
              );
            })()}
          </div>
          
          <DialogFooter>
            <Button variant="outline" onClick={() => setShowEnvVarsDialog(false)} disabled={addingServer}>
              Cancel
            </Button>
            <Button 
              onClick={() => performServerRegistration(envVarDialogValues)} 
              disabled={!isEnvVarDialogValid() || addingServer}
            >
              {addingServer ? "Adding..." : "Add Server"}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
};

export default CustomServerRegistry;