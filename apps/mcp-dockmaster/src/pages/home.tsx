import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { openUrl } from "@tauri-apps/plugin-opener";

import dockerIcon from "../assets/docker.svg";
import nodeIcon from "../assets/node.svg";
import pythonIcon from "../assets/python.svg";

import { Button } from "../components/ui/button";
import {
  Loader2,
  RefreshCw,
  ExternalLink,
  ChevronDown,
  ChevronRight,
  AppWindowIcon,
} from "lucide-react";
import { toast } from "sonner";
import { Badge } from "../components/ui/badge";
import { cn } from "@/lib/utils";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "../components/ui/dialog";
import {
  Collapsible,
  CollapsibleTrigger,
  CollapsibleContent,
} from "../components/ui/collapsible";
import MCPClient from "../lib/mcpClient";
import { McpClientApp, McpClientAppId, SUPPORTED_MCP_CLIENT_APPS } from "@/constants/mcp-client-apps";
import { getMCPProxyServerBinaryPath } from "@/lib/process";

interface PrerequisiteStatus {
  name: string;
  installed: boolean;
  loading: boolean;
  icon: string;
}

interface MCPClientStatus {
  app: McpClientApp;
  status: {
    isRunning: boolean;
    installed: boolean;
  }
}

const Home: React.FC = () => {
  const installUrls = {
    "Node.js": "https://nodejs.org/",
    "UV (Python)": "https://github.com/astral-sh/uv",
    Docker: "https://www.docker.com/get-started/",
    Claude: "https://claude.ai/download",
    Cursor: "https://www.cursor.com/",
  };

  const [prerequisites, setPrerequisites] = useState<PrerequisiteStatus[]>([
    { name: "Node.js", installed: false, loading: true, icon: nodeIcon },
    { name: "UV (Python)", installed: false, loading: true, icon: pythonIcon },
    { name: "Docker", installed: false, loading: true, icon: dockerIcon },
  ]);

  const [mcpClientApps, setMCPClientApps] = useState<{ [key in McpClientAppId]: MCPClientStatus }>(SUPPORTED_MCP_CLIENT_APPS.reduce((obj, mcpClientApp) => {
    obj[
      mcpClientApp.id] =
    {
      app: mcpClientApp,
      status: {
        isRunning: false,
        installed: false,
      }
    };
    return obj;
  }, {} as { [key in McpClientAppId]: MCPClientStatus }));

  const [isChecking, setIsChecking] = useState(false);
  const [mcpServers, setMCPServers] = useState<boolean>(false);
  const [mcpProxyServerBinaryPath, setMCPProxyServerBinaryPath] = useState('');

  // State variables for UI components
  const [showConfirmDialog, setShowConfirmDialog] = useState(false);
  const [isIntegrationOpen, setIsIntegrationOpen] = useState(true);
  const [isEnvDetailsOpen, setIsEnvDetailsOpen] = useState(true);
  const [isRegistryDetailsOpen, setIsRegistryDetailsOpen] = useState(false);
  const [isRestartOptionsOpen, setIsRestartOptionsOpen] = useState(true); // Default to open
  const [confirmDialogConfig, setConfirmDialogConfig] = useState<{
    title: string;
    onConfirm: () => Promise<void>;
  } | null>(null);

  const checkInstalled = async () => {
    for (const mcpClientApp of SUPPORTED_MCP_CLIENT_APPS) {
      const isInstalled = await mcpClientApp.isInstalled();
      const isRunning = await mcpClientApp.isRunning();

      setMCPClientApps({
        ...mcpClientApps,
        [mcpClientApp.id]: {
          ...mcpClientApps[mcpClientApp.id],
          status: {
            isRunning: isRunning,
            installed: isInstalled,
          }
        },
      });
    }
  };

  const checkPrerequisites = async () => {
    setIsChecking(true);
    setPrerequisites((prev) =>
      prev.map((item) => ({ ...item, loading: true })),
    );

    try {
      const checkNode = async () => {
        try {
          const installed = await invoke<boolean>("check_node_installed");
          return installed;
        } catch (error) {
          console.error("Failed to check Node.js:", error);
          return false;
        }
      };

      const checkUv = async () => {
        try {
          const installed = await invoke<boolean>("check_uv_installed");
          return installed;
        } catch (error) {
          console.error("Failed to check uv:", error);
          return false;
        }
      };

      const checkDocker = async () => {
        try {
          const installed = await invoke<boolean>("check_docker_installed");
          return installed;
        } catch (error) {
          console.error("Failed to check Docker:", error);
          return false;
        }
      };

      const [nodeInstalled, uvInstalled, dockerInstalled] = await Promise.all([
        checkNode(),
        checkUv(),
        checkDocker(),
      ]);

      setPrerequisites([
        {
          name: "Node.js",
          installed: nodeInstalled,
          loading: false,
          icon: nodeIcon,
        },
        {
          name: "UV (Python)",
          installed: uvInstalled,
          loading: false,
          icon: pythonIcon,
        },
        {
          name: "Docker",
          installed: dockerInstalled,
          loading: false,
          icon: dockerIcon,
        },
      ]);
    } catch (error) {
      console.error("Failed to check prerequisites:", error);
      setPrerequisites((prev) =>
        prev.map((item) => ({ ...item, loading: false })),
      );
    } finally {
      setIsChecking(false);
    }
  };

  const openInstallUrl = async (
    toolName: "Node.js" | "UV (Python)" | "Docker" | "Claude" | "Cursor" | "Generic",
  ) => {
    try {
      // Skip for Generic as it doesn't have an install URL
      if (toolName !== "Generic") {
        await openUrl(installUrls[toolName]);
      }
    } catch (error) {
      console.error(`Failed to open install URL for ${toolName}:`, error);
      toast.error(`Failed to open installation page for ${toolName}`);
    }
  };

  useEffect(() => {
    checkPrerequisites();
  }, []);

  useEffect(() => {
    checkInstalled();
    checkMCPServers();
  }, []);

  // Update collapsible sections based on prerequisites status
  useEffect(() => {
    // If all prerequisites are installed, collapse the environment details section
    if (prerequisites.every(p => p.installed) && !isChecking) {
      setIsEnvDetailsOpen(false);
    }
  }, [prerequisites, isChecking]);

  // Update collapsible sections based on MCP clients status
  useEffect(() => {
    // If any MCP client is installed, collapse the integration details section
    if (Object.values(mcpClientApps).some(c => c.status.installed)) {
      setIsIntegrationOpen(false);
    }
  }, [mcpClientApps]);

  useEffect(() => {
    getMCPProxyServerBinaryPath().then((path) => {
      setMCPProxyServerBinaryPath(path);
    });
  }, [])
  const checkMCPServers = async () => {
    try {
      const servers = await MCPClient.listServers();
      setMCPServers(servers.length > 0);
    } catch (error) {
      console.error("Failed to check MCP servers:", error);
      setMCPServers(false);
    }
  };

  const restartProcess = async (process_name: string) => {
    await invoke('restart_process', { process: { process_name } });
  }

  const reload = () => {
    checkPrerequisites();
    checkInstalled();
    checkMCPServers();
  };

  useEffect(() => {
    // Simplified config fetching since we're not using the configs in this view
    const fetchConfigs = async () => {
      try {
        // Check if configs can be fetched but don't store them
        const results = await Promise.allSettled([
          invoke<string>("get_claude_config"),
          invoke<string>("get_cursor_config"),
          invoke<string>("get_generic_config")
        ]);

        // If all configs failed, show error
        if (results.every(result => result.status === 'rejected')) {
          toast.error("Failed to fetch all configurations");
        }
      } catch (error) {
        console.error("Failed to fetch configurations:", error);
        toast.error("Failed to fetch configurations");
      }
    };

    fetchConfigs();
  }, []);

  const handleInstallClick = (mcpClientAppId: McpClientAppId) => {
    const mcpClientApp = mcpClientApps[mcpClientAppId];
    setConfirmDialogConfig({
      title: mcpClientApp.app.name,
      onConfirm: async () => {
        try {
          await invoke(`install_${mcpClientAppId.toLowerCase()}`);
          await checkInstalled();
          toast.success(
            `${mcpClientApp.app.name} installed successfully! Please restart ${mcpClientApp.app.name} to apply the changes.`,
          );
        } catch (error) {
          console.error(`Failed to install ${mcpClientAppId}:`, error);
          toast.error(`Failed to install ${mcpClientAppId}`);
        }
      },
    });
    setShowConfirmDialog(true);
  };

  return (
    <div className="mx-auto flex h-full w-full max-w-4xl flex-col gap-8 px-6 py-10">
      <div className="flex flex-col space-y-1.5">
        <div className="flex items-center justify-between">
          <h1 className="text-2xl font-semibold tracking-tight">
            Welcome to MCP Dockmaster
          </h1>
          <Button
            disabled={isChecking}
            variant="outline"
            size="sm"
            className="flex items-center gap-2"
            onClick={reload}
          >
            {isChecking ? (
              <Loader2 className="h-4 w-4 animate-spin" />
            ) : (
              <RefreshCw className="h-4 w-4" />
            )}
            {isChecking ? "Checking..." : "Refresh"}
          </Button>
        </div>
      </div>
      <div className="space-y-4">
        <div className="space-y-2">
          <p className="text-muted-foreground text-sm"><strong>What is MCP?</strong> MCP is an open-source standard from Anthropic that helps AI apps like Claude Desktop or Cursor easily access data from platforms such as Slack and Google Drive, interact with other applications, and connect to APIs.</p>
        </div>
      </div>

      <div className="space-y-4">
        <div className="space-y-2">
          <div className="flex items-center justify-between">
            <h2 className="text-lg font-medium">Getting Started</h2>
          </div>
          <div className="space-y-4">
            <div className="flex items-start gap-3">
              <div className="flex h-6 w-6 items-center justify-center rounded-full bg-primary text-primary-foreground">
                <span>1</span>
              </div>
              <div className="flex flex-col gap-2 flex-1">
                <div className="flex items-center gap-2">
                  <p className="text-muted-foreground text-sm">Make sure that you have Node.js, Python, and Docker installed so you can run MCPs.</p>
                  {prerequisites.every(p => p.installed) ? (
                    <Badge className="bg-green-500 text-white hover:bg-green-600 ml-2">
                      ✓
                    </Badge>
                  ) : (
                    <Badge variant="outline" className="border-red-500 bg-red-500/10 text-red-500 ml-2">
                      ✗
                    </Badge>
                  )}
                </div>

                <Collapsible
                  open={isEnvDetailsOpen}
                  onOpenChange={setIsEnvDetailsOpen}
                  className="ml-2 border-l-2 pl-4 border-muted"
                >
                  <CollapsibleTrigger asChild>
                    <Button variant="ghost" size="sm" className="flex items-center gap-1 h-7 px-2">
                      <span className="text-xs">Environment Details</span>
                      {isEnvDetailsOpen ? (
                        <ChevronDown className="h-3 w-3" />
                      ) : (
                        <ChevronRight className="h-3 w-3" />
                      )}
                    </Button>
                  </CollapsibleTrigger>
                  <CollapsibleContent className="space-y-2 mt-2">
                    <div className="grid grid-cols-3 gap-4">
                      {prerequisites.map((prerequisite) => (
                        <div
                          key={prerequisite.name}
                          className="hover:bg-muted/10 flex flex-col items-center rounded-lg border p-4 transition-colors"
                        >
                          <div className="flex flex-col items-center gap-3">
                            <div
                              className={cn(
                                "flex h-10 w-10 items-center justify-center rounded-full",
                                prerequisite.installed && "bg-green-500/10",
                                !prerequisite.installed && "bg-red-500/10",
                              )}
                            >
                              <img
                                src={prerequisite.icon}
                                alt={prerequisite.name}
                                className="h-5 w-5"
                              />
                            </div>
                            <div className="text-center">
                              <p className="font-medium">{prerequisite.name}</p>
                              <p className="text-muted-foreground text-sm">
                                {prerequisite.installed
                                  ? "Installed and running"
                                  : "Not installed or not running"}
                              </p>
                            </div>
                          </div>
                          {prerequisite.loading ? (
                            <div className="flex items-center gap-2 mt-3">
                              <span className="loading-indicator">Checking...</span>
                              <Loader2 className="h-4 w-4 animate-spin" />
                            </div>
                          ) : (
                            <div className="flex flex-col items-center gap-2 mt-3">
                              <span className="status-indicator">
                                {prerequisite.installed ? (
                                  <Badge className="bg-green-500 text-white hover:bg-green-600">
                                    Active
                                  </Badge>
                                ) : (
                                  <Badge
                                    variant="outline"
                                    className="border-red-500 bg-red-500/10 text-red-500"
                                  >
                                    Inactive
                                  </Badge>
                                )}
                              </span>
                              {!prerequisite.installed && (
                                <Button
                                  size="sm"
                                  variant="outline"
                                  className="mt-2"
                                  onClick={() =>
                                    openInstallUrl(
                                      prerequisite.name as
                                      | "Node.js"
                                      | "UV (Python)"
                                      | "Docker",
                                    )
                                  }
                                >
                                  Install
                                </Button>
                              )}
                            </div>
                          )}
                        </div>
                      ))}
                    </div>
                  </CollapsibleContent>
                </Collapsible>
              </div>
            </div>
            <div className="flex items-start gap-3">
              <div className="flex h-6 w-6 items-center justify-center rounded-full bg-primary text-primary-foreground">
                <span>2</span>
              </div>
              <div className="flex flex-col gap-2 flex-1">
                <div className="flex items-center gap-2">
                  <p className="text-muted-foreground text-sm">Add Dockmaster to Cursor, Claude Desktop, or any other MCP client.</p>
                  {Object.values(mcpClientApps).some(c => c.status.installed) ? (
                    <Badge className="bg-green-500 text-white hover:bg-green-600 ml-2">
                      ✓
                    </Badge>
                  ) : (
                    <Badge variant="outline" className="border-red-500 bg-red-500/10 text-red-500 ml-2">
                      ✗
                    </Badge>
                  )}
                </div>

                <Collapsible
                  open={isIntegrationOpen}
                  onOpenChange={setIsIntegrationOpen}
                  className="ml-2 border-l-2 pl-4 border-muted"
                >
                  <CollapsibleTrigger asChild>
                    <Button variant="ghost" size="sm" className="flex items-center gap-1 h-7 px-2">
                      <span className="text-xs">Integration Details</span>
                      {isIntegrationOpen ? (
                        <ChevronDown className="h-3 w-3" />
                      ) : (
                        <ChevronRight className="h-3 w-3" />
                      )}
                    </Button>
                  </CollapsibleTrigger>
                  <CollapsibleContent className="space-y-2 mt-2">
                    <div className="grid grid-cols-3 gap-4">
                      {Object.values(mcpClientApps).map((client) => (
                        <div
                          key={client.app.name}
                          className="hover:bg-muted/10 flex flex-col items-center rounded-lg border p-4 transition-colors"
                        >
                          <div className="flex flex-col items-center gap-3">
                            <div
                              className={cn(
                                "flex h-10 w-10 items-center justify-center rounded-full",
                                client.status.installed && "bg-green-500/10",
                                !client.status.installed && "bg-red-500/10",
                              )}
                            >
                              <img
                                src={client.app.icon}
                                alt={client.app.name}
                                className="h-5 w-5"
                              />
                            </div>
                            <div className="flex items-center gap-2">
                              <p className="font-medium">{client.app.name}</p>
                              <button
                                onClick={() =>
                                  openInstallUrl(client.app.name as "Claude" | "Cursor")
                                }
                                className="text-muted-foreground hover:text-foreground transition-colors"
                              >
                                <ExternalLink className="h-4 w-4" />
                              </button>
                            </div>
                          </div>
                          <div className="flex flex-col items-center gap-2 mt-3">
                            <span className="status-indicator">
                              {client.status.installed ? (
                                <Badge className="bg-green-500 text-white hover:bg-green-600">
                                  Active
                                </Badge>
                              ) : (
                                <Badge
                                  variant="outline"
                                  className="border-red-500 bg-red-500/10 text-red-500"
                                >
                                  Inactive
                                </Badge>
                              )}
                            </span>
                            <div className="flex items-center gap-2 mt-2">
                              <Button
                                size="sm"
                                variant="outline"
                                onClick={() => handleInstallClick(client.app.id)}
                              >
                                Install
                              </Button>
                            </div>
                          </div>
                        </div>
                      ))}
                      <div
                        key={'other'}
                        className="flex flex-col items-center justify-center rounded-lg transition-colors"
                      >
                        ...
                      </div>
                        <div
                        key={'other'}
                        className="hover:bg-muted/10 flex flex-col items-center rounded-lg border p-4 transition-colors"
                        >
                        <div className="flex flex-col items-center gap-3">
                          <div
                          className={cn(
                            "flex h-10 w-10 items-center justify-center rounded-full border",
                          )}
                          >
                          <AppWindowIcon
                            className="h-5 w-5"
                          />
                          </div>
                          <div className="flex items-center gap-2">
                          <p className="font-medium">Other Apps</p>
                          </div>
                        </div>
                        <div className="flex flex-col items-center gap-2 mt-3 text-center">
                          <p className="text-muted-foreground text-sm">
                          Add MCP Dockmaster to any other app that supports MCP Servers of type command:
                          </p>
                          <div className="bg-gray-100 p-2 rounded-lg text-xs w-full">
                          
                          <code className="break-all">
                            {mcpProxyServerBinaryPath}
                          </code>
                          </div>
                        </div>
                        </div>
                    </div>
                  </CollapsibleContent>
                </Collapsible>
              </div>
            </div>
            <div className="flex items-start gap-3">
              <div className="flex h-6 w-6 items-center justify-center rounded-full bg-primary text-primary-foreground">
                <span>3</span>
              </div>
              <div className="flex flex-col gap-2 flex-1">
                <div className="flex items-center gap-2">
                  <p className="text-muted-foreground text-sm">Install MCPs from the registry or a GitHub URL.</p>
                  {/* Check if at least one MCP server is installed */}
                  {mcpServers ? (
                    <Badge className="bg-green-500 text-white hover:bg-green-600 ml-2">
                      ✓
                    </Badge>
                  ) : (
                    <Badge variant="outline" className="border-gray-500 bg-gray-500/10 text-gray-500 ml-2">
                      ?
                    </Badge>
                  )}
                </div>

                <Collapsible
                  open={isRegistryDetailsOpen}
                  onOpenChange={setIsRegistryDetailsOpen}
                  className="ml-2 border-l-2 pl-4 border-muted"
                >
                  <CollapsibleTrigger asChild>
                    <Button variant="ghost" size="sm" className="flex items-center gap-1 h-7 px-2">
                      <span className="text-xs">Registry Details</span>
                      {isRegistryDetailsOpen ? (
                        <ChevronDown className="h-3 w-3" />
                      ) : (
                        <ChevronRight className="h-3 w-3" />
                      )}
                    </Button>
                  </CollapsibleTrigger>
                  <CollapsibleContent className="space-y-2 mt-2">
                    <p className="text-muted-foreground text-sm">
                      Browse available MCPs from the registry to extend your AI applications with various capabilities.
                    </p>
                    <Button
                      size="sm"
                      className="mt-2 flex items-center gap-2"
                      onClick={() => {
                        // Navigate to MCP Server Registry inside the app
                        window.location.href = "/registry";
                      }}
                    >
                      <span>View Registry</span>
                      <ExternalLink className="h-3 w-3" />
                    </Button>
                  </CollapsibleContent>
                </Collapsible>
              </div>
            </div>
            <div className="flex items-start gap-3">
              <div className="flex h-6 w-6 items-center justify-center rounded-full bg-primary text-primary-foreground">
                <span>4</span>
              </div>
              <div className="flex flex-col gap-2 flex-1">
                <div className="flex items-center gap-2">
                  <p className="text-muted-foreground text-sm">Restart Claude Desktop and Cursor and you are good to go!</p>
                  {Object.values(mcpClientApps).every(c => c.status.installed) ? (
                    <Badge className="bg-green-500 text-white hover:bg-green-600 ml-2">
                      ✓
                    </Badge>
                  ) : (
                    <Badge variant="outline" className="border-red-500 bg-red-500/10 text-red-500 ml-2">
                      ✗
                    </Badge>
                  )}
                </div>

                <Collapsible
                  open={isRestartOptionsOpen}
                  onOpenChange={setIsRestartOptionsOpen}
                  className="ml-2 border-l-2 pl-4 border-muted"
                >
                  <CollapsibleTrigger asChild>
                    <Button variant="ghost" size="sm" className="flex items-center gap-1 h-7 px-2">
                      <span className="text-xs">Restart Options</span>
                      {isRestartOptionsOpen ? (
                        <ChevronDown className="h-3 w-3" />
                      ) : (
                        <ChevronRight className="h-3 w-3" />
                      )}
                    </Button>
                  </CollapsibleTrigger>
                  <CollapsibleContent className="space-y-2 mt-2">
                    <p className="text-muted-foreground text-sm">
                      Restart your MCP clients to apply the changes and start using your MCPs.
                    </p>
                    <div className="grid grid-cols-3 gap-4 mt-2">
                      {Object.values(mcpClientApps).map((client) => (
                        <div
                          key={client.app.name}
                          className="hover:bg-muted/10 flex flex-col items-center rounded-lg border p-4 transition-colors"
                        >
                          <div className="flex flex-col items-center gap-3">
                            <div
                              className={cn(
                                "flex h-10 w-10 items-center justify-center rounded-full",
                                client.status.isRunning ? "bg-green-500/10" : client.status.isRunning && "bg-red-500/10",
                              )}
                            >
                              <img
                                src={client.app.icon}
                                alt={client.app.name}
                                className="h-5 w-5"
                              />
                            </div>
                            <p className="font-medium">{client.app.name}</p>
                          </div>
                          <div className="flex flex-col items-center gap-2 mt-3">
                            <span className="status-indicator">
                              {client.status.isRunning ? (
                                <Badge className="bg-green-500 text-white hover:bg-green-600">
                                  Running
                                </Badge>
                              ) : (
                                <Badge
                                  variant="outline"
                                  className="border-red-500 bg-red-500/10 text-red-500"
                                >
                                  Not Running
                                </Badge>
                              )}
                            </span>
                            <Button
                              size="sm"
                              variant="outline"
                              className="mt-2 flex items-center gap-2"
                              onClick={async () => {
                                try {
                                  await restartProcess(client.app.name);
                                  toast.success(`${client.app.name} restarted successfully!`);
                                  await checkInstalled();
                                } catch (error) {
                                  console.error(`Failed to restart ${client.app.name}:`, error);
                                  toast.error(`Failed to restart ${client.app.name}`);
                                }
                              }}
                            >
                              <RefreshCw className="h-3 w-3" />
                              <span>Restart</span>
                            </Button>
                          </div>
                        </div>
                      ))}
                    </div>
                  </CollapsibleContent>
                </Collapsible>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Config dialog removed as it's not being used in this view */}

      <Dialog open={showConfirmDialog} onOpenChange={setShowConfirmDialog}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Confirm Installation</DialogTitle>
            <DialogDescription>
              Please make sure {confirmDialogConfig?.title} is closed before
              continuing.
            </DialogDescription>
          </DialogHeader>
          <DialogFooter>
            <Button
              variant="outline"
              onClick={() => setShowConfirmDialog(false)}
            >
              Cancel
            </Button>
            <Button
              onClick={async () => {
                if (confirmDialogConfig) {
                  await confirmDialogConfig.onConfirm();
                  setShowConfirmDialog(false);
                }
              }}
            >
              OK
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
};

export default Home;
