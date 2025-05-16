// @ts-nocheck
import React, { useState, useEffect } from "react";
import { useVirtualizer } from "@tanstack/react-virtual";
import MCPClient, { RegistryServer } from "../lib/mcpClient";
import { getAvailableServers, getCategories } from "../lib/registry";
import {
  SERVER_UNINSTALLED,
  SERVER_INSTALLED,
  dispatchServerInstalled,
  dispatchServerUninstalled,
} from "../lib/events";
import "./Registry.css";
import { toast } from "sonner";
import { invoke } from "@tauri-apps/api/core";
import { isProcessRunning } from "../lib/process";
import { useTranslation } from "@mcp-dockmaster/i18n";

// Import runner icons
// @ts-ignore
import dockerIcon from "../assets/docker.svg";
// @ts-ignore
import nodeIcon from "../assets/node.svg";
// @ts-ignore
import pythonIcon from "../assets/python.svg";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "./ui/card";
import { Button } from "./ui/button";
import { Badge } from "./ui/badge";
import { Skeleton } from "./ui/skeleton";
// @ts-ignore
import { Search, ChevronRight, ChevronLeft, Link, Info } from "lucide-react";
import { Label } from "./ui/label";
import { Input } from "./ui/input";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "./ui/dialog";

const Registry: React.FC = () => {
  const { t } = useTranslation();
  const [availableServers, setAvailableServers] = useState<RegistryServer[]>([]);
  const [loading, setLoading] = useState(true);
  const [installing, setInstalling] = useState<string | null>(null);
  const [uninstalling, setUninstalling] = useState<string | null>(null);
  const [searchTerm, setSearchTerm] = useState("");
  const [categories, setCategories] = useState<[string, number][]>([]);
  const [selectedCategory, setSelectedCategory] = useState<string | null>(null);
  const [showAllCategories, setShowAllCategories] = useState(false);
  const [detailsPopupVisible, setDetailsPopupVisible] = useState(false);
  const [currentServerDetails, setCurrentServerDetails] = useState<RegistryServer | null>(null);
  
  // Add state for GitHub import modal
  const [isGitHubImportModalOpen, setIsGitHubImportModalOpen] = useState(false);
  const [githubUrl, setGithubUrl] = useState("");
  const [importingServer, setImportingServer] = useState(false);
  const [importError, setImportError] = useState<string | null>(null);

  // Add state for restart dialog
  const [showConfirmDialog, setShowConfirmDialog] = useState(false);
  const [confirmDialogConfig, setConfirmDialogConfig] = useState<{ 
    title: string; 
    explanation?: string; 
    showClaude?: boolean;
    showCursor?: boolean;
    onRestartClaude?: () => Promise<void>;
    onRestartCursor?: () => Promise<void>;
    onRestartBoth?: () => Promise<void>;
    onConfirm?: () => Promise<void>;
  } | null>(null);

  // Add state for ENV variable collection dialog
  const [showEnvVarsDialog, setShowEnvVarsDialog] = useState(false);
  const [envVarValues, setEnvVarValues] = useState<Record<string, string>>({});
  const [currentServerForEnvVars, setCurrentServerForEnvVars] = useState<RegistryServer | null>(null);

  // Load tools and categories on initial mount
  useEffect(() => {
    const init = async () => {
      await loadAvailableServers();
      await loadCategories();
    };
    init();

    // Add event listener for visibility change to reload tools when component becomes visible
    const handleVisibilityChange = async () => {
      if (document.visibilityState === "visible") {
        await loadAvailableServers();
        await loadCategories();
      }
    };

    document.addEventListener("visibilitychange", handleVisibilityChange);
    window.addEventListener("focus", async () => {
      await loadAvailableServers();
      await loadCategories();
    });

    return () => {
      document.removeEventListener("visibilitychange", handleVisibilityChange);
      window.removeEventListener("focus", loadAvailableServers);
    };
  }, []);

  // Listen for tool installation/uninstallation events
  useEffect(() => {
    // When a tool is uninstalled, update its status in the registry
    const handleToolUninstalled = (event: CustomEvent<{ toolId: string }>) => {
      const { toolId } = event.detail;
      setAvailableServers((prev: RegistryServer[]) =>
        prev.map((tool: RegistryServer) =>
          tool.id === toolId ? { ...tool, installed: false } : tool,
        ),
      );
    };

    // When a tool is installed elsewhere, update its status in the registry
    const handleToolInstalled = (event: CustomEvent<{ toolId: string }>) => {
      const { toolId } = event.detail;
      setAvailableServers((prev: RegistryServer[]) =>
        prev.map((tool: RegistryServer) =>
          tool.id === toolId ? { ...tool, installed: true } : tool,
        ),
      );
    };

    // Add event listeners
    document.addEventListener(
      SERVER_UNINSTALLED,
      handleToolUninstalled as EventListener,
    );
    document.addEventListener(
      SERVER_INSTALLED,
      handleToolInstalled as EventListener,
    );

    // Clean up event listeners on unmount
    return () => {
      document.removeEventListener(
        SERVER_UNINSTALLED,
        handleToolUninstalled as EventListener,
      );
      document.removeEventListener(
        SERVER_INSTALLED,
        handleToolInstalled as EventListener,
      );
    };
  }, []);

  const loadAvailableServers = async () => {
    setLoading(true);
    try {
      // Get tools from registry
      const registryTools = await getAvailableServers();

      // Get installed tools to check status
      const installedServers = await MCPClient.listServers();
      console.log("installedServers: ", installedServers);

      // Create a map to ensure we don't have duplicate tools by name
      const uniqueServersMap = new Map();

      // Process registry tools and mark as installed if they match installed tools
      registryTools.forEach((tool) => {
        // Check if tool is installed by ID or by name (in case IDs don't match)
        const isInstalled = installedServers.some(
          (installedServer) =>
            installedServer.id === tool.id ||
            installedServer.name.toLowerCase() === tool.name.toLowerCase(),
        );

        // Use lowercase name as key to avoid case-sensitivity issues
        const key = tool.name.toLowerCase();

        // Only add if not already in the map
        if (!uniqueServersMap.has(key)) {
          uniqueServersMap.set(key, {
            ...tool,
            installed: isInstalled,
          });
        }
      });

      // Convert map values to array
      const serversWithStatus = Array.from(uniqueServersMap.values());

      setAvailableServers(serversWithStatus);
    } catch (error) {
      console.error("Failed to load available tools:", error);
    } finally {
      setLoading(false);
    }
  };

  const loadCategories = async () => {
    try {
      const categoriesData = await getCategories();
      // Count featured items from current availableServers state
      const featuredCount = availableServers.filter(server => server.featured).length;
      // Add Featured as first category if there are featured items
      const allCategories: [string, number][] = featuredCount > 0 
        ? [[t('registry.featured_category'), featuredCount], ...categoriesData] 
        : categoriesData;
      setCategories(allCategories);
    } catch (error) {
      console.error("Failed to load categories:", error);
    }
  };

  const restartProcess = async (process_name: string) => {
    await invoke('restart_process', { process: { process_name } });
  }


  const isClaudeInstalled = async () => {
    try {
      return await invoke<boolean>("check_claude_installed");
    } catch (error) {
      console.error("Failed to check Claude:", error);
      return false;
    }
  };

  // Check if Cursor is installed
  const isCursorInstalled = async () => {
    try {
      return await invoke<boolean>("check_cursor_installed");
    } catch (error) {
      console.error("Failed to check Cursor:", error);
      return false;
    }
  };

  // Helper function to check if a server has any ENV variables (required or optional)
  const hasEnvVars = (server: RegistryServer): boolean => {
    if (!server.config?.env) return false;
    
    // Check if any env vars exist
    return Object.keys(server.config.env).length > 0;
  };

  const showRestartDialog = async () => {
    const processName = ["Claude", "Cursor"];
    const [isRunning1, isRunning2, isInstalled1, isInstalled2] = await Promise.all([
      isProcessRunning(processName[0]),
      isProcessRunning(processName[1]),
      isClaudeInstalled(),
      isCursorInstalled(),
    ]);

    const showClaude = isRunning1 && isInstalled1;
    const showCursor = isRunning2 && isInstalled2;
    if (showClaude || showCursor) {
      let title = "";
      if (showClaude && showCursor) {
        title = t('registry.confirm_dialog.restart_both_title');
      } else if (showClaude) {
        title = t('registry.confirm_dialog.restart_claude_title');
      } else if (showCursor) {
        title = t('registry.confirm_dialog.restart_cursor_title');
      }
      setConfirmDialogConfig({
        title,
        explanation: t('registry.confirm_dialog.restart_explanation'),
        showClaude,
        showCursor,
        onRestartClaude: async () => {
          await restartProcess(processName[0]);
          toast.success(t('registry.confirm_dialog.restart_claude_success'));
        },
        onRestartCursor: async () => {
          await restartProcess(processName[1]);
          toast.success(t('registry.confirm_dialog.restart_cursor_success'));
        },
        onRestartBoth: async () => {
          await restartProcess(processName[0]);
          await restartProcess(processName[1]);
          toast.success(t('registry.confirm_dialog.restart_both_success'));
        },
      });
      setShowConfirmDialog(true);
    }
  };

  const handleEnvVarChange = (key: string, value: string) => {
    setEnvVarValues((prev) => ({ ...prev, [key]: value }));
  };

  const installServer = async (server: RegistryServer) => {
    const installedServers = await MCPClient.listServers();
    console.log("installedServers: ", installedServers);

    const isAlreadyInstalled = installedServers.some(
      (installedServer) =>
        installedServer.id === server.id ||
        installedServer.name.toLowerCase() === server.name.toLowerCase(),
    );

    if (isAlreadyInstalled) {
      setAvailableServers((prev: RegistryServer[]) =>
        prev.map((item: RegistryServer) =>
          item.id === server.id ? { ...item, installed: true } : item,
        ),
      );
      return;
    }

    // Check if the server has any ENV variables (required or optional)
    if (hasEnvVars(server)) {
      // Initialize env var values with defaults
      const initialEnvVars: Record<string, string> = {};
      if (server.config?.env) {
        Object.entries(server.config.env).forEach(([key, value]) => {
          if (typeof value === 'object') {
            initialEnvVars[key] = value.default || '';
          }
        });
      }
      
      setEnvVarValues(initialEnvVars);
      setCurrentServerForEnvVars(server);
      setShowEnvVarsDialog(true);
      return;
    }

    // If no required ENV vars or they're already set, proceed with installation
    await performInstallation(server);
  };

  const performInstallation = async (server: RegistryServer, customEnvVars?: Record<string, string>) => {
    setInstalling(server.id);
    try {
      const entryPoint = getDefaultEntryPoint(server.name);

      let authentication = null;
      if (server.config && server.config.env) {
        // Create a copy of the original env configuration
        const envConfig = { ...server.config.env };
        
        // Update with custom values if provided
        if (customEnvVars) {
          Object.entries(customEnvVars).forEach(([key, value]) => {
            if (typeof envConfig[key] === 'object') {
              // Update the default value while preserving other properties
              envConfig[key] = {
                ...envConfig[key],
                default: value
              };
            } else {
              envConfig[key] = value;
            }
          });
        }
        
        authentication = { env: envConfig };
      }

      console.log(
        "Registering server:",
        JSON.stringify(server, null, 2),
        server.runtime,
        entryPoint,
        authentication,
      );

      // Create a modified configuration with updated ENV values
      let modifiedConfig = server.config;
      
      if (customEnvVars && server.config?.env) {
        // Create a deep copy of the configuration
        modifiedConfig = {
          ...server.config,
          env: { ...server.config.env }
        };
        
        // Update the env values with user input
        Object.entries(customEnvVars).forEach(([key, value]) => {
          if (modifiedConfig.env && typeof modifiedConfig.env[key] === 'object') {
            modifiedConfig.env[key] = {
              ...modifiedConfig.env[key],
              default: value
            };
          }
        });
      }
      
      const response = await MCPClient.registerServer({
        server_id: server.id,
        server_name: server.name,
        description: server.description,
        tools_type: server.runtime,
        configuration: modifiedConfig,
        distribution: server.distribution,
        authentication: authentication,
      });

      if (response.success) {
        setAvailableServers((prev: RegistryServer[]) =>
          prev.map((item: RegistryServer) =>
            item.id === server.id ? { ...item, installed: true } : item,
          ),
        );

        dispatchServerInstalled(server.id);
        
        await showRestartDialog();
      }
    } catch (error) {
      console.error("Failed to install server:", error);
    } finally {
      setInstalling(null);
    }
  };

  const uninstallServer = async (id: string) => {
    try {
      console.log("Uninstalling server:", id);
      setUninstalling(id);

      // Update the UI optimistically
      setAvailableServers((prev: RegistryServer[]) =>
        prev.map((server: RegistryServer) =>
          server.id === id ? { ...server, installed: false } : server,
        ),
      );

      // Get the tool from the registry
      const registryServer = availableServers.find((server: RegistryServer) => server.id === id);
      if (!registryServer) {
        console.error("Server not found in registry:", id);
        return;
      }

      // Get the actual tool ID from the backend by matching names
      const installedServers = await MCPClient.listServers();
      const matchingServer = installedServers.find(
        (server) => server.name.toLowerCase() === registryServer.name.toLowerCase(),
      );

      console.log("matchingServer: ", matchingServer);

      if (!matchingServer) {
        console.error("Tool not found in installed tools:", registryServer.name);
        // Revert UI change
        setAvailableServers((prev: RegistryServer[]) =>
          prev.map((server: RegistryServer) =>
            server.id === id ? { ...server, installed: true } : server,
          ),
        );
        return;
      }

      // Use the actual tool ID from the backend
      const actualServerId = matchingServer.id;

      // Call the backend API to uninstall the tool
      const response = await MCPClient.uninstallServer({
        server_id: actualServerId,
      });

      if (response.success) {
        // Dispatch event that a tool was uninstalled with the registry ID for UI updates
        dispatchServerUninstalled(id);

        // Check if the process needs to be restarted
        await showRestartDialog();
        
      } else {
        // If the API call fails, revert the UI change
        console.error("Failed to uninstall tool:", response.message);
        setAvailableServers((prev: RegistryServer[]) =>
          prev.map((tool: RegistryServer) =>
            tool.id === id ? { ...tool, installed: true } : tool,
          ),
        );
      }
    } catch (error) {
      console.error("Error uninstalling server:", error);
      // Refresh the list to ensure UI is in sync with backend
      loadAvailableServers();
    } finally {
      setUninstalling(null);
    }
  };

  // Helper function to get a default entry point based on tool type and name
  const getDefaultEntryPoint = (toolName: string): string => {
    // Try to find the tool in the available tools to get its distribution info
    const tool = availableServers.find((t: RegistryServer) => t.name === toolName);

    if (tool && tool.distribution && tool.config) {
      // Run the command with the args if provided
      if (tool.config.command && tool.config.args) {
        return `${tool.config.command} ${tool.config.args.join(" ")}`;
      }

      // Fallback 1: If the tool is an npm package, use npx to run it
      if (tool.distribution.type === "npm" && tool.distribution.package) {
        return `npx -y ${tool.distribution.package}`;
      }

      // Fallback 2: If the tool is a docker image, use the image name
      if (tool.distribution.type === "dockerhub" && tool.distribution.package) {
        return `docker run --name ${tool.id} ${tool.distribution.package}`;
      }
    }

    throw new Error(`No entry point found for tool: ${toolName}`);
  };

  const getRunnerIcon = (runtime: string) => {
    switch (runtime.toLowerCase()) {
      case "docker":
        return (
          <img
            src={dockerIcon}
            alt="Docker"
            className="runner-icon"
            title="Docker"
          />
        );
      case "node":
        return (
          <img
            src={nodeIcon}
            alt="Node.js"
            className="runner-icon"
            title="Node.js"
          />
        );
      case "python":
        return (
          <img
            src={pythonIcon}
            alt="Python/UV"
            className="runner-icon"
            title="Python/UV"
          />
        );
      default:
        return <span className="runner-icon unknown">?</span>;
    }
  };
  const parentRef = React.useRef<HTMLDivElement>(null);

  const filteredTools = availableServers.filter((tool: RegistryServer) => {
    const matchesSearch = searchTerm
      ? tool.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
        tool.description.toLowerCase().includes(searchTerm.toLowerCase())
      : true;

    const matchesCategory = selectedCategory
      ? selectedCategory === "Featured" 
        ? tool.featured 
        : tool.categories?.includes(selectedCategory)
      : true;

    return matchesSearch && matchesCategory;
  });

  // Get visible categories (first 5 if not showing all)
  const visibleCategories = showAllCategories
    ? categories
    : categories.slice(0, 12);
    
  // Functions to handle the details popup
  const openDetailsPopup = (tool: RegistryServer, e: React.MouseEvent) => {
    e.stopPropagation(); // Prevent any parent click handlers
    setCurrentServerDetails(tool);
    setDetailsPopupVisible(true);
  };
  
  const closeDetailsPopup = () => {
    setDetailsPopupVisible(false);
    setCurrentServerDetails(null);
  };
  
  // Functions to handle the GitHub import modal
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
      setImportError(t('registry.import_modal.error_empty_url'));
      return;
    }
    
    // Simple validation for GitHub URL
    if (!githubUrl?.startsWith("https://github.com/")) {
      setImportError(t('registry.import_modal.error_invalid_url'));
      return;
    }
    
    setImportingServer(true);
    setImportError(null);
    
    try {
      const response = await MCPClient.importServerFromUrl(githubUrl);
      
      if (response.success) {
        closeGitHubImportModal();
        loadAvailableServers(); // Refresh the server list
      } else {
        setImportError(response.message || t('registry.import_modal.import_generic_error'));
      }
    } catch (error) {
      console.error("Error importing server:", error);
      setImportError(t('registry.import_modal.import_error', { message: (error instanceof Error ? error.message : String(error)) }));
    } finally {
      setImportingServer(false);
    }
  };
  
  // Function to render the GitHub import modal
  const renderGitHubImportModal = () => {
    return (
      <Dialog open={isGitHubImportModalOpen} onOpenChange={setIsGitHubImportModalOpen}>
        <DialogContent className="sm:max-w-[500px]">
          <DialogHeader>
            <DialogTitle>{t('registry.import_modal.title')}</DialogTitle>
            <DialogDescription>
              {t('registry.import_modal.description')}
            </DialogDescription>
          </DialogHeader>
          <div className="bg-amber-50 border border-amber-200 rounded-md p-3 mb-3">
            <p className="text-amber-800 text-sm">
              <strong>{t('registry.import_modal.note_title')}</strong> {t('registry.import_modal.note_description')}
            </p>
          </div>
          <div className="grid gap-4 py-4">
            <div className="space-y-2">
              <Label htmlFor="github-url">{t('registry.import_modal.url_label')}</Label>
              <Input
                id="github-url"
                placeholder={t('registry.import_modal.url_placeholder')}
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
              {t('registry.import_modal.cancel_button')}
            </Button>
            <Button onClick={importServerFromGitHub} disabled={!githubUrl.trim() || importingServer}>
              {importingServer ? t('registry.import_modal.importing_button') : t('registry.import_modal.import_button')}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    );
  };
  
  // Function to render the details popup
  const renderEnvVarsDialog = () => {
    if (!showEnvVarsDialog || !currentServerForEnvVars) return null;
    
    // Check if there are any ENV variables
    const hasEnvVars = currentServerForEnvVars.config?.env && 
                      Object.keys(currentServerForEnvVars.config.env).length > 0;
    
    // Count required ENV variables
    const requiredEnvVarsCount = hasEnvVars ? 
      Object.values(currentServerForEnvVars.config.env).filter(
        value => typeof value === 'object' && value.required
      ).length : 0;
    
    return (
      <Dialog open={showEnvVarsDialog} onOpenChange={setShowEnvVarsDialog}>
        <DialogContent className="sm:max-w-[600px]">
          <DialogHeader>
            <DialogTitle>{t('registry.env_vars_dialog.title')}</DialogTitle>
            <DialogDescription>
              {requiredEnvVarsCount > 0 
                ? t('registry.env_vars_dialog.description_required', { serverName: currentServerForEnvVars.name, count: requiredEnvVarsCount })
                : t('registry.env_vars_dialog.description_optional', { serverName: currentServerForEnvVars.name })
              }
            </DialogDescription>
          </DialogHeader>
          <div className="grid gap-4 py-4 max-h-[60vh] overflow-y-auto">
            {hasEnvVars && (
              <>
                {/* Required ENV variables section */}
                {requiredEnvVarsCount > 0 && (
                  <div className="mb-2">
                    <h3 className="text-sm font-medium mb-2">{t('registry.env_vars_dialog.required_title')}</h3>
                    {Object.entries(currentServerForEnvVars.config.env).map(([key, value]) => {
                      // Only show required ENV variables in this section
                      if (typeof value !== 'object' || !value.required) return null;
                      
                      const description = value.description || '';
                      const defaultValue = value.default || '';
                      
                      return (
                        <div key={key} className="grid grid-cols-4 items-start gap-4 mb-3">
                          <Label className="text-right text-xs pt-1" htmlFor={`env-${key}`}>
                            {key}
                            <span className="text-red-500 ml-1">{t('registry.env_vars_dialog.required_marker')}</span>
                          </Label>
                          <div className="col-span-3 space-y-1">
                            <Input
                              id={`env-${key}`}
                              value={envVarValues[key] || defaultValue}
                              onChange={(e) => handleEnvVarChange(key, e.target.value)}
                              placeholder={description}
                            />
                            {description && (
                              <p className="text-muted-foreground text-xs">{description}</p>
                            )}
                          </div>
                        </div>
                      );
                    })}
                  </div>
                )}
                
                {/* Optional ENV variables section */}
                {Object.entries(currentServerForEnvVars.config.env).some(
                  ([_, value]) => typeof value === 'object' && !value.required
                ) && (
                  <div>
                    <h3 className="text-sm font-medium mb-2">{t('registry.env_vars_dialog.optional_title')}</h3>
                    {Object.entries(currentServerForEnvVars.config.env).map(([key, value]) => {
                      // Only show optional ENV variables in this section
                      if (typeof value !== 'object' || value.required) return null;
                      
                      const description = value.description || '';
                      const defaultValue = value.default || '';
                      
                      return (
                        <div key={key} className="grid grid-cols-4 items-start gap-4 mb-3">
                          <Label className="text-right text-xs pt-1" htmlFor={`env-${key}`}>
                            {key}
                          </Label>
                          <div className="col-span-3 space-y-1">
                            <Input
                              id={`env-${key}`}
                              value={envVarValues[key] || defaultValue}
                              onChange={(e) => handleEnvVarChange(key, e.target.value)}
                              placeholder={description}
                            />
                            {description && (
                              <p className="text-muted-foreground text-xs">{description}</p>
                            )}
                          </div>
                        </div>
                      );
                    })}
                  </div>
                )}
              </>
            )}
          </div>
          <DialogFooter>
            <Button variant="outline" onClick={() => setShowEnvVarsDialog(false)}>
              {t('registry.env_vars_dialog.cancel_button')}
            </Button>
            <Button 
              onClick={() => {
                setShowEnvVarsDialog(false);
                performInstallation(currentServerForEnvVars, envVarValues);
              }}
              disabled={Object.entries(currentServerForEnvVars.config?.env || {}).some(([key, value]) => {
                // Check if any required field is empty
                return typeof value === 'object' && 
                       value.required && 
                       (!envVarValues[key] || envVarValues[key].trim() === '');
              })}
            >
              {t('registry.env_vars_dialog.install_button')}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    );
  };

  const renderDetailsPopup = () => {
    if (!detailsPopupVisible || !currentServerDetails) return null;
    
    return (
      <Dialog open={detailsPopupVisible} onOpenChange={closeDetailsPopup}>
        <DialogContent className="sm:max-w-[600px]">
          <DialogHeader>
            <DialogTitle>{t('registry.details_popup.title', { serverName: currentServerDetails.name })}</DialogTitle>
            <DialogDescription>
              {t('registry.details_popup.description')}
            </DialogDescription>
          </DialogHeader>
          <div className="grid gap-4 py-4 max-h-[60vh] overflow-y-auto">
            {/* Basic Information */}
            <div className="space-y-2">
              <h3 className="text-sm font-medium">{t('registry.details_popup.basic_info_title')}</h3>
              <div className="rounded-md border p-3 space-y-2">
                <div className="grid grid-cols-4 items-start gap-4">
                  <Label className="text-right text-xs pt-1">{t('registry.details_popup.description_label')}</Label>
                  <div className="col-span-3 text-sm">
                    {currentServerDetails.description}
                  </div>
                </div>
                <div className="grid grid-cols-4 items-start gap-4">
                  <Label className="text-right text-xs pt-1">{t('registry.details_popup.id_label')}</Label>
                  <div className="col-span-3 text-sm font-mono">{currentServerDetails.id}</div>
                </div>
                <div className="grid grid-cols-4 items-start gap-4">
                  <Label className="text-right text-xs pt-1">{t('registry.details_popup.runtime_label')}</Label>
                  <div className="col-span-3 text-sm">{currentServerDetails.runtime}</div>
                </div>
                {currentServerDetails.license && (
                  <div className="grid grid-cols-4 items-start gap-4">
                    <Label className="text-right text-xs pt-1">{t('registry.details_popup.license_label')}</Label>
                    <div className="col-span-3 text-sm">{currentServerDetails.license}</div>
                  </div>
                )}
                {currentServerDetails.categories && currentServerDetails.categories.length > 0 && (
                  <div className="grid grid-cols-4 items-start gap-4">
                    <Label className="text-right text-xs pt-1">{t('registry.details_popup.categories_label')}</Label>
                    <div className="col-span-3 flex flex-wrap gap-1">
                      {currentServerDetails.categories.map((category) => (
                        <Badge key={category} variant="outline" className="text-xs">
                          {category}
                        </Badge>
                      ))}
                    </div>
                  </div>
                )}
              </div>
            </div>
            {/* Tools */}
            {currentServerDetails.tools && currentServerDetails.tools.length > 0 && (
              <div className="space-y-2">
                <h3 className="text-sm font-medium">{t('registry.details_popup.tools_title')}</h3>
                <div className="rounded-md border p-3 space-y-2">
                  {currentServerDetails.tools.map((tool) => (
                    <div key={tool.signature}>
                      <div className="text-sm font-medium">
                        {tool.signature.match(/^(.+)\(/)?.[1]}
                      </div>
                      <div className="text-sm font-small">                      
                        {tool.description}
                      </div>
                      <div className="text-xs font-mono">
                        {tool.signature}
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            )}
            
            {/* Publisher Information */}
            {currentServerDetails.publisher && (
              <div className="space-y-2">
                <h3 className="text-sm font-medium">{t('registry.details_popup.publisher_title')}</h3>
                <div className="rounded-md border p-3 space-y-2">
                  <div className="grid grid-cols-4 items-start gap-4">
                    <Label className="text-right text-xs pt-1">{t('registry.details_popup.name_label')}</Label>
                    <div className="col-span-3 text-sm">{currentServerDetails.publisher.name}</div>
                  </div>
                  {currentServerDetails.publisher.url && (
                    <div className="grid grid-cols-4 items-start gap-4">
                      <Label className="text-right text-xs pt-1">{t('registry.details_popup.url_label')}</Label>
                      <div className="col-span-3">
                        <a 
                          href={currentServerDetails.publisher.url} 
                          target="_blank" 
                          rel="noopener noreferrer"
                          className="text-blue-500 hover:underline text-sm"
                        >
                          {currentServerDetails.publisher.url}
                        </a>
                      </div>
                    </div>
                  )}
                </div>
              </div>
            )}
            
            {/* Configuration */}
            {currentServerDetails.config && (
              <div className="space-y-2">
                <h3 className="text-sm font-medium">{t('registry.details_popup.config_title')}</h3>
                <div className="rounded-md border p-3 space-y-2">
                  {currentServerDetails.config.command && (
                    <div className="grid grid-cols-4 items-start gap-4">
                      <Label className="text-right text-xs pt-1">{t('registry.details_popup.command_label')}</Label>
                      <div className="col-span-3 text-sm font-mono">{currentServerDetails.config.command}</div>
                    </div>
                  )}
                  {currentServerDetails.config.args && currentServerDetails.config.args.length > 0 && (
                    <div className="grid grid-cols-4 items-start gap-4">
                      <Label className="text-right text-xs pt-1">{t('registry.details_popup.args_label')}</Label>
                      <div className="col-span-3 text-sm font-mono">
                        {currentServerDetails.config.args.map((arg, index) => (
                          <div key={index}>{arg}</div>
                        ))}
                      </div>
                    </div>
                  )}
                  {currentServerDetails.config.env && Object.keys(currentServerDetails.config.env).length > 0 && (
                    <div className="grid grid-cols-4 items-start gap-4">
                      <Label className="text-right text-xs pt-1">{t('registry.details_popup.env_vars_label')}</Label>
                      <div className="col-span-3 space-y-2">
                        {Object.entries(currentServerDetails.config.env).map(([key, value]) => (
                          <div key={key} className="text-sm">
                            <div className="font-medium">{key}</div>
                            {typeof value === 'object' && value.description ? (
                              <div className="text-muted-foreground text-xs">{value.description}</div>
                            ) : (
                              <div className="text-muted-foreground text-xs">{t('registry.details_popup.value_label', { value: String(value) })}</div>
                            )}
                          </div>
                        ))}
                      </div>
                    </div>
                  )}
                </div>
              </div>
            )}
            
            {/* Distribution */}
            {currentServerDetails.distribution && (
              <div className="space-y-2">
                <h3 className="text-sm font-medium">{t('registry.details_popup.distribution_title')}</h3>
                <div className="rounded-md border p-3 space-y-2">
                  <div className="grid grid-cols-4 items-start gap-4">
                    <Label className="text-right text-xs pt-1">{t('registry.details_popup.type_label')}</Label>
                    <div className="col-span-3 text-sm">{currentServerDetails.distribution.type}</div>
                  </div>
                  <div className="grid grid-cols-4 items-start gap-4">
                    <Label className="text-right text-xs pt-1">{t('registry.details_popup.package_label')}</Label>
                    <div className="col-span-3 text-sm font-mono">{currentServerDetails.distribution.package}</div>
                  </div>
                </div>
              </div>
            )}
          </div>
          <DialogFooter>
            <Button variant="outline" onClick={closeDetailsPopup}>
              {t('registry.details_popup.close_button')}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    );
  };

  // Set up the virtualizer
  const rowVirtualizer = useVirtualizer({
    count: filteredTools.length,
    getScrollElement: () => parentRef.current,
    estimateSize: () => 170, // Estimated height of each tool item in pixels
    overscan: 1, // Number of items to render beyond the visible area
    gap: 16,
  });

  return (
    <div className="mx-auto flex h-full w-full max-w-4xl flex-col gap-8 px-6 py-10 pb-4">
      <div className="flex flex-col space-y-1.5">
        <div className="flex justify-between items-center">
          <h1 className="text-2xl font-semibold tracking-tight">{t('registry.title')}</h1>
          <Button variant="outline" onClick={openGitHubImportModal}>
            {t('registry.import_button')}
          </Button>
        </div>
        <p className="text-muted-foreground text-sm">
          {t('registry.description')}
        </p>
      </div>

      <div className="flex flex-col gap-4">
        <div className="relative h-full max-h-12 min-h-12 flex-1 shrink-0">
          <div className="pointer-events-none absolute top-4 left-3 flex items-center">
            <Search size={18} className="text-gray-400" />
          </div>
          <input
            type="text"
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
            className="bg-background text-foreground placeholder:text-muted-foreground size-full rounded-lg border py-2 pr-4 pl-10 focus:ring-1 focus:ring-neutral-900/60 focus:outline-none"
            placeholder={t('registry.search_placeholder')}
            aria-label="Search for tools"
          />
        </div>

        {searchTerm && (
          <div className="text-muted-foreground text-sm">
            {t(filteredTools.length === 1 ? 'registry.search_results_found' : 'registry.search_results_found_plural', { count: filteredTools.length })}
          </div>
        )}

        <div className={`flex flex-wrap items-center gap-2 pb-2`}>
          <div
            className={`flex flex-wrap gap-2 ${showAllCategories ? "max-h-[300px] overflow-y-auto" : ""}`}
          >
            {visibleCategories.map(([category, count]) => (
              <Badge
                key={category}
                variant={selectedCategory === category ? "default" : "outline"}
                className="cursor-pointer whitespace-nowrap"
                onClick={() =>
                  setSelectedCategory(
                    selectedCategory === category ? null : category,
                  )
                }
              >
                {category} {count > 1 ? `(${count})` : ""}
              </Badge>
            ))}
          </div>
          {categories.length > 5 && (
            <Button
              variant="ghost"
              size="sm"
              className="whitespace-nowrap"
              onClick={() => setShowAllCategories(!showAllCategories)}
            >
              {showAllCategories ? t('registry.show_less') : t('registry.show_all_categories')}
              {showAllCategories ? (
                <ChevronLeft className="ml-1 h-4 w-4" />
              ) : (
                <ChevronRight className="ml-1 h-4 w-4" />
              )}
            </Button>
          )}
        </div>
      </div>

      {loading ? (
        <div className="flex flex-col items-center justify-center gap-3">
          {Array.from({ length: 3 }).map((_, index) => (
            <Skeleton key={index} className="bg-muted h-40 w-full rounded-md" />
          ))}
        </div>
      ) : (
        <>
          {filteredTools.length === 0 ? (
            <div className="text-muted-foreground py-10 text-center text-sm">
              <p>{t('registry.no_results')}</p>
            </div>
          ) : (
            <div
              ref={parentRef}
              style={{ height: "100%", overflow: "auto", contain: "strict" }}
            >
              <div
                style={{
                  height: `${rowVirtualizer.getTotalSize()}px`,
                  width: "100%",
                  position: "relative",
                  overflow: "hidden",
                }}
              >
                {rowVirtualizer.getVirtualItems().map((virtualRow) => {
                  const tool = filteredTools[virtualRow.index];
                  return (
                    <div
                      key={tool.id}
                      style={{
                        position: "absolute",
                        width: "100%",
                        height: `${virtualRow.size}px`,
                        transform: `translateY(${virtualRow.start}px)`,
                        boxSizing: "border-box",
                      }}
                    >
                      <Card className="w-full gap-4 overflow-hidden border-slate-200 shadow-none">
                        <CardHeader className="pb-0">
                          <div className="flex items-start justify-between">
                            <div className="flex items-center gap-3">
                              <div className="bg-muted size-8 rounded-md p-1">
                                {getRunnerIcon(tool.runtime)}
                              </div>

                              <CardTitle className="text-lg">
                                {tool.name}
                                <a
                                  href={tool.publisher.url}
                                  target="_blank"
                                  rel="noopener noreferrer"
                                  className="text-foreground underline"
                                >
                                  <Link
                                    size={14}
                                    className="ml-1 inline-block"
                                  />
                                </a>
                                <button 
                                  className="ml-1 text-muted-foreground hover:text-blue-500 focus:outline-none cursor-pointer"
                                  onClick={(e) => openDetailsPopup(tool, e)}
                                  title={t('registry.view_details_tooltip')}
                                >
                                  <Info size={14} className="inline-block" />
                                </button>
                              </CardTitle>
                              {tool.installed && (
                                <Badge variant="outline" className="ml-auto">
                                  {t('registry.installed_badge')}
                                </Badge>
                              )}
                            </div>
                          </div>
                        </CardHeader>
                        <CardContent>
                          <CardDescription className="line-clamp-2">
                            {tool.short_description}
                          </CardDescription>
                        </CardContent>
                        <CardFooter className="flex items-center justify-between pt-0">
                          {tool.publisher && (
                            <div className="text-muted-foreground flex items-center gap-2 text-sm">
                              <span>{t('registry.by_publisher')} </span> {tool.publisher.name}
                              {tool.featured && (
                                <Badge variant="outline" className="ml-2">
                                  {t('registry.featured_category')}
                                </Badge>
                              )}
                            </div>
                          )}
                          {!tool.publisher && tool.featured && (
                            <Badge variant="outline">
                              {t('registry.featured_category')}
                            </Badge>
                          )}

                          {tool.installed ? (
                            <Button
                              variant="destructive"
                              onClick={() => uninstallServer(tool.id)}
                              disabled={uninstalling === tool.id}
                              type="button"
                            >
                              {uninstalling === tool.id
                                ? t('registry.uninstalling_button')
                                : t('registry.uninstall_button')}
                            </Button>
                          ) : (
                            <Button
                              variant="outline"
                              type="button"
                              onClick={() =>
                                !tool.installed && installServer(tool)
                              }
                              disabled={
                                tool.installed || installing === tool.id
                              }
                            >
                              {tool.installed
                                ? t('registry.installed_badge') // Should technically not happen if button is disabled
                                : installing === tool.id
                                  ? t('registry.installing_button')
                                  : t('registry.install_button')}
                            </Button>
                          )}
                        </CardFooter>
                      </Card>
                    </div>
                  );
                })}
              </div>
            </div>
          )}
        </>
      )}
      {renderDetailsPopup()}
      {renderGitHubImportModal()}
      {renderEnvVarsDialog()}
      <Dialog open={showConfirmDialog} onOpenChange={setShowConfirmDialog}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>{confirmDialogConfig?.title || t('registry.confirm_dialog.title')}</DialogTitle>
            <DialogDescription>
              {/* Use explanation if provided, otherwise default */} 
              {confirmDialogConfig?.explanation ? confirmDialogConfig.explanation : t('registry.confirm_dialog.default_description')}
              {/* Display explanation separately if it exists, to avoid duplication if title already contains it */} 
              {confirmDialogConfig?.explanation && confirmDialogConfig?.title !== confirmDialogConfig.explanation && (
                <p className="mt-2 text-sm text-muted-foreground">
                  {confirmDialogConfig.explanation}
                </p>
              )}
            </DialogDescription>
          </DialogHeader>
          <DialogFooter className="flex flex-wrap gap-2">
            <Button variant="outline" onClick={() => setShowConfirmDialog(false)}>
              {t('registry.confirm_dialog.manual_button')}
            </Button>
            {confirmDialogConfig?.showClaude && (
              <Button
                onClick={() => {
                  setShowConfirmDialog(false);
                  confirmDialogConfig?.onRestartClaude?.();
                }}
              >
                {t('registry.confirm_dialog.restart_claude_button')}
              </Button>
            )}
            {confirmDialogConfig?.showCursor && (
              <Button
                onClick={() => {
                  setShowConfirmDialog(false);
                  confirmDialogConfig?.onRestartCursor?.();
                }}
              >
                {t('registry.confirm_dialog.restart_cursor_button')}
              </Button>
            )}
            {confirmDialogConfig?.showClaude && confirmDialogConfig?.showCursor && (
              <Button
                onClick={() => {
                  setShowConfirmDialog(false);
                  confirmDialogConfig?.onRestartBoth?.();
                }}
              >
                {t('registry.confirm_dialog.restart_both_button')}
              </Button>
            )}
            {confirmDialogConfig?.onConfirm && (
              <Button
                onClick={() => {
                  setShowConfirmDialog(false);
                  confirmDialogConfig?.onConfirm?.();
                }}
              >
                {t('registry.confirm_dialog.confirm_button')}
              </Button>
            )}
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
};

export default Registry;
