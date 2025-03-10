import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { openUrl } from "@tauri-apps/plugin-opener";

import dockerIcon from "../assets/docker.svg";
import nodeIcon from "../assets/node.svg";
import pythonIcon from "../assets/python.svg";
import claudeIcon from "../assets/claude.svg";
import cursorIcon from "../assets/cursor.svg";

import { Button } from "../components/ui/button";
import {
  Loader2,
  RefreshCw,
  ExternalLink,
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

interface PrerequisiteStatus {
  name: string;
  installed: boolean;
  loading: boolean;
  icon: string;
}

interface MCPClientStatus {
  name: string;
  installed: boolean;
  icon: string;
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

  const [mcpClients, setMCPClients] = useState<MCPClientStatus[]>([
    { name: "Claude", installed: false, icon: claudeIcon },
    { name: "Cursor", installed: false, icon: cursorIcon },
  ]);

  const [isChecking, setIsChecking] = useState(false);

  const [claudeConfig, setClaudeConfig] = useState<string | null>(null);
  const [cursorConfig, setCursorConfig] = useState<string | null>(null);
  const [showConfirmDialog, setShowConfirmDialog] = useState(false);
  const [confirmDialogConfig, setConfirmDialogConfig] = useState<{
    title: string;
    onConfirm: () => Promise<void>;
  } | null>(null);
  const [showConfigDialog, setShowConfigDialog] = useState(false);
  const [configDialogContent, setConfigDialogContent] = useState<{
    title: string;
    config: string;
  } | null>(null);

  const checkInstalled = async () => {
    // Check if Claude is installed
    const checkClaude = async () => {
      try {
        return await invoke<boolean>("check_claude_installed");
      } catch (error) {
        console.error("Failed to check Claude:", error);
        return false;
      }
    };

    // Check if Cursor is installed
    const checkCursor = async () => {
      try {
        return await invoke<boolean>("check_cursor_installed");
      } catch (error) {
        console.error("Failed to check Cursor:", error);
        return false;
      }
    };
    const [claudeInstalled, cursorInstalled] = await Promise.all([
      checkClaude(),
      checkCursor(),
    ]);
    setMCPClients([
      { name: "Claude", installed: claudeInstalled, icon: claudeIcon },
      { name: "Cursor", installed: cursorInstalled, icon: cursorIcon },
    ]);
  };

  const checkPrerequisites = async () => {
    setIsChecking(true);
    setPrerequisites((prev) =>
      prev.map((item) => ({ ...item, loading: true })),
    );

    try {
      // Check if Node.js is installed
      const checkNode = async () => {
        try {
          const installed = await invoke<boolean>("check_node_installed");
          return installed;
        } catch (error) {
          console.error("Failed to check Node.js:", error);
          return false;
        }
      };

      // Check if uv is installed
      const checkUv = async () => {
        try {
          const installed = await invoke<boolean>("check_uv_installed");
          return installed;
        } catch (error) {
          console.error("Failed to check uv:", error);
          return false;
        }
      };

      // Check if Docker is installed
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
    toolName: "Node.js" | "UV (Python)" | "Docker" | "Claude" | "Cursor",
  ) => {
    try {
      await openUrl(installUrls[toolName]);
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
  }, []);

  const reload = () => {
    checkPrerequisites();
    checkInstalled();
  };

  useEffect(() => {
    const fetchConfigs = async () => {
      try {
        const claude = await invoke<string>("get_claude_config");
        const cursor = await invoke<string>("get_cursor_config");
        setClaudeConfig(claude);
        setCursorConfig(cursor);
      } catch (error) {
        console.error("Failed to fetch configurations:", error);
      }
    };

    fetchConfigs();
  }, []);

  const handleInstallClick = (appName: "Claude" | "Cursor") => {
    setConfirmDialogConfig({
      title: appName,
      onConfirm: async () => {
        try {
          await invoke(`install_${appName.toLowerCase()}`);
          await checkInstalled();
          toast.success(
            `${appName} installed successfully! Please restart ${appName} to apply the changes.`,
          );
        } catch (error) {
          console.error(`Failed to install ${appName}:`, error);
          toast.error(`Failed to install ${appName}`);
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
        <p className="text-muted-foreground text-sm">
          Select an option from the sidebar to get started.
        </p>
      </div>

      <div className="space-y-4">
        <div className="space-y-2">
          <h2 className="text-lg font-medium">Integrate with MCP Clients</h2>
          <p className="text-muted-foreground text-sm">
            Using the proxy tool, you will be able to integrate with MCP clients
            like Claude offering all the tools you configure in MCP Dockmaster.
          </p>
        </div>

        <div className="space-y-4">
          <div className="grid gap-4">
            {mcpClients.map((client) => (
              <div
                key={client.name}
                className="hover:bg-muted/10 flex items-center justify-between rounded-lg border p-4 transition-colors"
              >
                <div className="flex items-center gap-3">
                  <div
                    className={cn(
                      "flex h-10 w-10 items-center justify-center rounded-full",
                      client.installed && "bg-green-500/10",
                      !client.installed && "bg-red-500/10",
                    )}
                  >
                    <img
                      src={client.icon}
                      alt={client.name}
                      className="h-5 w-5"
                    />
                  </div>
                  <div className="flex items-center gap-2">
                    <p className="font-medium">{client.name}</p>
                    <button
                      onClick={() =>
                        openInstallUrl(client.name as "Claude" | "Cursor")
                      }
                      className="text-muted-foreground hover:text-foreground transition-colors"
                    >
                      <ExternalLink className="h-4 w-4" />
                    </button>
                  </div>
                </div>
                <div className="flex items-center gap-2">
                  <Button
                    size="sm"
                    variant="outline"
                    onClick={() => {
                      const config =
                        client.name === "Claude" ? claudeConfig : cursorConfig;
                      if (config) {
                        setConfigDialogContent({
                          title: client.name,
                          config: config,
                        });
                        setShowConfigDialog(true);
                      }
                    }}
                  >
                    Show Config
                  </Button>
                  {!client.installed && (
                    <Button
                      size="sm"
                      variant="outline"
                      onClick={() =>
                        handleInstallClick(client.name as "Claude" | "Cursor")
                      }
                    >
                      Install
                    </Button>
                  )}
                  <span className="status-indicator">
                    {client.installed ? (
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
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>

      <div className="space-y-4">
        <div className="space-y-2">
          <h2 className="text-lg font-medium">Runner Environment Support</h2>
          <p className="text-muted-foreground text-sm">
            The following tools are required to run MCP Dockmaster.
          </p>
        </div>
        <div className="space-y-4">
          <div className="grid gap-4">
            {prerequisites.map((prerequisite) => (
              <div
                key={prerequisite.name}
                className="hover:bg-muted/10 flex items-center justify-between rounded-lg border p-4 transition-colors"
              >
                <div className="flex items-center gap-3">
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
                  <div>
                    <p className="font-medium">{prerequisite.name}</p>
                    <p className="text-muted-foreground text-sm">
                      {prerequisite.installed
                        ? "Installed and running"
                        : "Not installed or not running"}
                    </p>
                  </div>
                </div>
                {prerequisite.loading ? (
                  <div className="flex items-center gap-2">
                    <span className="loading-indicator">Checking...</span>
                    <Loader2 className="h-4 w-4 animate-spin" />
                  </div>
                ) : (
                  <div className="flex items-center gap-2">
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
                        className="ml-2"
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
        </div>
      </div>

      <Dialog open={showConfigDialog} onOpenChange={setShowConfigDialog}>
        <DialogContent className="flex max-h-[80vh] max-w-3xl flex-col">
          <DialogHeader>
            <DialogTitle>
              {configDialogContent?.title} Configuration
            </DialogTitle>
            <DialogDescription>
              Use this configuration to connect {configDialogContent?.title} to
              your MCP servers:
            </DialogDescription>
          </DialogHeader>
          <div className="flex-1 overflow-auto">
            <pre className="rounded-md bg-black p-4 text-sm text-white">
              <code className="break-words whitespace-pre-wrap">
                {configDialogContent?.config}
              </code>
            </pre>
          </div>
          <DialogFooter className="mt-4">
            <Button
              variant="outline"
              onClick={() => setShowConfigDialog(false)}
            >
              Close
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

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
