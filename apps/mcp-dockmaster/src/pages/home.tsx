import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { openUrl } from "@tauri-apps/plugin-opener";

import dockerIcon from "../assets/docker.svg";
import nodeIcon from "../assets/node.svg";
import pythonIcon from "../assets/python.svg";

import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@radix-ui/react-collapsible";
import { Button } from "../components/ui/button";
import {
  ArrowRight,
  ChevronDown,
  ChevronUp,
  Loader2,
  RefreshCw,
} from "lucide-react";
import { toast } from "sonner";
import { Separator } from "../components/ui/separator";
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

const Home: React.FC = () => {
  const installUrls = {
    "Node.js": "https://nodejs.org/",
    "UV (Python)": "https://github.com/astral-sh/uv",
    "Docker": "https://www.docker.com/get-started/"
  };
  
  const [prerequisites, setPrerequisites] = useState<PrerequisiteStatus[]>([
    { name: "Node.js", installed: false, loading: true, icon: nodeIcon },
    { name: "UV (Python)", installed: false, loading: true, icon: pythonIcon },
    { name: "Docker", installed: false, loading: true, icon: dockerIcon },
  ]);
  const [isChecking, setIsChecking] = useState(false);
  const [showMCPConfig, setShowMCPConfig] = useState(false);
  
  const [isClaudeInstalled, setIsClaudeInstalled] = useState<boolean | null>(null);
  const [isCursorInstalled, setIsCursorInstalled] = useState<boolean | null>(null);
  const [claudeConfig, setClaudeConfig] = useState<string | null>(null);
  const [cursorConfig, setCursorConfig] = useState<string | null>(null);
  const [showConfirmDialog, setShowConfirmDialog] = useState(false);
  const [confirmDialogConfig, setConfirmDialogConfig] = useState<{
    title: string;
    onConfirm: () => Promise<void>;
  } | null>(null);

  const checkInstalled = async () => {
    // Check if Claude is installed
    const checkClaude = async () => {
      try {
        const installed = await invoke<boolean>("check_claude_installed");
        setIsClaudeInstalled(installed);
      } catch (error) {
        console.error("Failed to check Claude:", error);
        return false;
      }
    };

    // Check if Cursor is installed
    const checkCursor = async () => {
      try {
        const installed = await invoke<boolean>("check_cursor_installed");
        setIsCursorInstalled(installed);
      } catch (error) {
        console.error("Failed to check Cursor:", error);
        return false;
      }
    }
    await Promise.all([checkClaude(), checkCursor()]);
}


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

  const openInstallUrl = async (toolName: "Node.js" | "UV (Python)" | "Docker") => {
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
          toast.success(`${appName} installed successfully! Please restart ${appName} to apply the changes.`);
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
        <h1 className="text-2xl font-semibold tracking-tight">
          Welcome to MPC Dockmaster
        </h1>
        <p className="text-muted-foreground text-sm">
          Select an option from the sidebar to get started.
        </p>
      </div>
      <div className="space-y-2">
        <h2 className="text-lg font-medium">Integrate with MCP Clients</h2>
        <p className="text-muted-foreground text-sm">
          Using the proxy tool, you will be able to integrate with MCP clients
          like Claude offering all the tools you configure in MPC Dockmaster.
        </p>

        <div className="flex items-center gap-2">
          <span className="font-medium">Claude Desktop Status:</span>
          {isClaudeInstalled ?  (
            <Badge className="bg-green-500 text-white hover:bg-green-600">
              Active
            </Badge>
          ) : (
            <div className="flex items-center gap-2">
              <Badge variant="outline" className="border-red-500 bg-red-500/10 text-red-500">
                Inactive
              </Badge>
              <Button
                size="sm"
                variant="outline"
                className="ml-2"
                onClick={() => handleInstallClick("Claude")}
              >
                Install
              </Button>
            </div>
          )}
        </div>

        <div className="flex items-center gap-2">
          <span className="font-medium">Cursor Status:</span>
          {isCursorInstalled ? (
            <Badge className="bg-green-500 text-white hover:bg-green-600">
              Active
            </Badge>
          ) : (
            <div className="flex items-center gap-2">
              <Badge variant="outline" className="border-red-500 bg-red-500/10 text-red-500">
                Inactive
              </Badge>
              <Button
                size="sm"
                variant="outline"
                className="ml-2"
                onClick={() => handleInstallClick("Cursor")}
              >
                Install
              </Button>
            </div>
          )}
        </div>

        <Collapsible
          className="mt-4 space-y-2"
          open={showMCPConfig}
          onOpenChange={setShowMCPConfig}
        >
          <CollapsibleTrigger asChild>
            <Button
              variant="outline"
              className="flex w-full items-center justify-between gap-2 shadow-none"
            >
              <span className="flex items-center gap-2">
                <ArrowRight className="h-4 w-4" />
                Show MCP Configuration
              </span>
              {showMCPConfig ? (
                <ChevronUp className="text-muted-foreground h-4 w-4 shrink-0" />
              ) : (
                <ChevronDown className="text-muted-foreground h-4 w-4 shrink-0" />
              )}
            </Button>
          </CollapsibleTrigger>
          <CollapsibleContent className="bg-muted/30 space-y-8 rounded-md border p-4">
            <div className="space-y-4">
              <div>
                <h4 className="font-semibold">Claude Desktop Manual Configuration</h4>
                <p className="text-muted-foreground text-sm">
                  Use this configuration to connect Claude to your MCP servers:
                </p>
              </div>
              <div className="relative">
                <pre className="max-h-[300px] overflow-auto rounded-md bg-black p-4 text-sm text-white">
                  <code>{claudeConfig}</code>
                </pre>
              </div>
            </div>
            <Separator />
            <div className="space-y-4">
              <div>
                <h4 className="font-semibold">Cursor Manual Configuration</h4>
                <p className="text-muted-foreground text-sm">
                  Use this configuration to connect Cursor to your MCP servers:
                </p>
              </div>
              <div className="relative">
                <pre className="max-h-[300px] overflow-auto rounded-md bg-black p-4 text-sm text-white">
                  <code>{cursorConfig}</code>
                </pre>
              </div>
            </div>
          </CollapsibleContent>
        </Collapsible>
      </div>
      <div className="space-y-4">
        <div className="flex flex-row items-center justify-between">
          <h2 className="text-lg font-medium">Runner Environment Support</h2>
          <Button
            disabled={isChecking}
            variant="outline"
            size="sm"
            className="flex items-center gap-2"
            onClick={checkPrerequisites}
          >
            {isChecking ? (
              <Loader2 className="h-4 w-4 animate-spin" />
            ) : (
              <RefreshCw className="h-4 w-4" />
            )}
            {isChecking ? "Checking..." : "Refresh"}
          </Button>
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
                        onClick={() => openInstallUrl(prerequisite.name as "Node.js" | "UV (Python)" | "Docker")}
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

      <Dialog open={showConfirmDialog} onOpenChange={setShowConfirmDialog}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Confirm Installation</DialogTitle>
            <DialogDescription>
              Please make sure {confirmDialogConfig?.title} is closed before continuing.
            </DialogDescription>
          </DialogHeader>
          <DialogFooter>
            <Button variant="outline" onClick={() => setShowConfirmDialog(false)}>
              Cancel
            </Button>
            <Button onClick={async () => {
              if (confirmDialogConfig) {
                await confirmDialogConfig.onConfirm();
                setShowConfirmDialog(false);
              }
            }}>
              OK
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

    </div>
  );
};

export default Home;
