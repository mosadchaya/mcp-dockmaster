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

  // Load tools and categories on initial mount
  useEffect(() => {
    loadAvailableServers();
    loadCategories();

    // Add event listener for visibility change to reload tools when component becomes visible
    const handleVisibilityChange = () => {
      if (document.visibilityState === "visible") {
        loadAvailableServers();
        loadCategories();
      }
    };

    document.addEventListener("visibilitychange", handleVisibilityChange);
    window.addEventListener("focus", loadAvailableServers);

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
      setCategories(categoriesData);
    } catch (error) {
      console.error("Failed to load categories:", error);
    }
  };

  const installServer = async (server: RegistryServer) => {
    // Double-check if the tool is already installed before proceeding
    const installedServers = await MCPClient.listServers();
    console.log("installedServers: ", installedServers);

    const isAlreadyInstalled = installedServers.some(
      (installedServer) =>
        installedServer.id === server.id ||
        installedServer.name.toLowerCase() === server.name.toLowerCase(),
    );

    if (isAlreadyInstalled) {
      // Tool is already installed, update UI and don't try to install again
      setAvailableServers((prev: RegistryServer[]) =>
        prev.map((item: RegistryServer) =>
          item.id === server.id ? { ...item, installed: true } : item,
        ),
      );
      return;
    }

    setInstalling(server.id);
    try {
      // For now, use a default entry point based on the tool type
      const entryPoint = getDefaultEntryPoint(server.name);

      // Prepare authentication if needed
      let authentication = null;
      if (server.config && server.config.env) {
        // For now, we don't have a way to collect env vars from the user
        // In a real implementation, you would prompt the user for these values
        authentication = { env: server.config.env };
      }

      console.log(
        "Registering server:",
        JSON.stringify(server, null, 2),
        server.runtime,
        entryPoint,
        authentication,
      );

      const response = await MCPClient.registerServer({
        server_id: server.id,
        server_name: server.name,
        description: server.description,
        tools_type: server.runtime,
        configuration: server.config,
        distribution: server.distribution,
        authentication: authentication,
      });

      if (response.success) {
        // Update tool as installed
        setAvailableServers((prev: RegistryServer[]) =>
          prev.map((item: RegistryServer) =>
            item.id === server.id ? { ...item, installed: true } : item,
          ),
        );

        // Dispatch event that a tool was installed
        dispatchServerInstalled(server.id);
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
      ? tool.categories?.includes(selectedCategory)
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
    if (!githubUrl.trim()) {
      setImportError("Please enter a GitHub URL");
      return;
    }
    
    // Simple validation for GitHub URL
    if (!githubUrl.startsWith("https://github.com/")) {
      setImportError("Please enter a valid GitHub repository URL");
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
        setImportError(response.message || "Failed to import server");
      }
    } catch (error) {
      console.error("Error importing server:", error);
      setImportError("Failed to import server: " + (error instanceof Error ? error.message : String(error)));
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
            <DialogTitle>Import MCP Server from GitHub</DialogTitle>
            <DialogDescription>
              Enter a GitHub repository URL to import a new MCP server.
              The repository should contain a package.json (for Node.js) or pyproject.toml (for Python) file.
            </DialogDescription>
          </DialogHeader>
          <div className="bg-amber-50 border border-amber-200 rounded-md p-3 mb-3">
            <p className="text-amber-800 text-sm">
              <strong>Note:</strong> We will attempt to extract required environment variables from the repository's README.md file.
              Please note that this process may not identify all required variables correctly.
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
    );
  };
  
  // Function to render the details popup
  const renderDetailsPopup = () => {
    if (!detailsPopupVisible || !currentServerDetails) return null;
    
    return (
      <Dialog open={detailsPopupVisible} onOpenChange={closeDetailsPopup}>
        <DialogContent className="sm:max-w-[600px]">
          <DialogHeader>
            <DialogTitle>{currentServerDetails.name}</DialogTitle>
            <DialogDescription>
              Server details and information
            </DialogDescription>
          </DialogHeader>
          <div className="grid gap-4 py-4 max-h-[60vh] overflow-y-auto">
            {/* Basic Information */}
            <div className="space-y-2">
              <h3 className="text-sm font-medium">Basic Information</h3>
              <div className="rounded-md border p-3 space-y-2">
                <div className="grid grid-cols-4 items-start gap-4">
                  <Label className="text-right text-xs pt-1">Description</Label>
                  <div className="col-span-3 text-sm">
                    {currentServerDetails.fullDescription || currentServerDetails.description}
                  </div>
                </div>
                <div className="grid grid-cols-4 items-start gap-4">
                  <Label className="text-right text-xs pt-1">ID</Label>
                  <div className="col-span-3 text-sm font-mono">{currentServerDetails.id}</div>
                </div>
                <div className="grid grid-cols-4 items-start gap-4">
                  <Label className="text-right text-xs pt-1">Runtime</Label>
                  <div className="col-span-3 text-sm">{currentServerDetails.runtime}</div>
                </div>
                {currentServerDetails.license && (
                  <div className="grid grid-cols-4 items-start gap-4">
                    <Label className="text-right text-xs pt-1">License</Label>
                    <div className="col-span-3 text-sm">{currentServerDetails.license}</div>
                  </div>
                )}
                {currentServerDetails.categories && currentServerDetails.categories.length > 0 && (
                  <div className="grid grid-cols-4 items-start gap-4">
                    <Label className="text-right text-xs pt-1">Categories</Label>
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
            
            {/* Publisher Information */}
            {currentServerDetails.publisher && (
              <div className="space-y-2">
                <h3 className="text-sm font-medium">Publisher</h3>
                <div className="rounded-md border p-3 space-y-2">
                  <div className="grid grid-cols-4 items-start gap-4">
                    <Label className="text-right text-xs pt-1">Name</Label>
                    <div className="col-span-3 text-sm">{currentServerDetails.publisher.name}</div>
                  </div>
                  {currentServerDetails.publisher.url && (
                    <div className="grid grid-cols-4 items-start gap-4">
                      <Label className="text-right text-xs pt-1">URL</Label>
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
                <h3 className="text-sm font-medium">Configuration</h3>
                <div className="rounded-md border p-3 space-y-2">
                  {currentServerDetails.config.command && (
                    <div className="grid grid-cols-4 items-start gap-4">
                      <Label className="text-right text-xs pt-1">Command</Label>
                      <div className="col-span-3 text-sm font-mono">{currentServerDetails.config.command}</div>
                    </div>
                  )}
                  {currentServerDetails.config.args && currentServerDetails.config.args.length > 0 && (
                    <div className="grid grid-cols-4 items-start gap-4">
                      <Label className="text-right text-xs pt-1">Arguments</Label>
                      <div className="col-span-3 text-sm font-mono">
                        {currentServerDetails.config.args.map((arg, index) => (
                          <div key={index}>{arg}</div>
                        ))}
                      </div>
                    </div>
                  )}
                  {currentServerDetails.config.env && Object.keys(currentServerDetails.config.env).length > 0 && (
                    <div className="grid grid-cols-4 items-start gap-4">
                      <Label className="text-right text-xs pt-1">Environment Variables</Label>
                      <div className="col-span-3 space-y-2">
                        {Object.entries(currentServerDetails.config.env).map(([key, value]) => (
                          <div key={key} className="text-sm">
                            <div className="font-medium">{key}</div>
                            {typeof value === 'object' && value.description ? (
                              <div className="text-muted-foreground text-xs">{value.description}</div>
                            ) : (
                              <div className="text-muted-foreground text-xs">Value: {String(value)}</div>
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
                <h3 className="text-sm font-medium">Distribution</h3>
                <div className="rounded-md border p-3 space-y-2">
                  <div className="grid grid-cols-4 items-start gap-4">
                    <Label className="text-right text-xs pt-1">Type</Label>
                    <div className="col-span-3 text-sm">{currentServerDetails.distribution.type}</div>
                  </div>
                  <div className="grid grid-cols-4 items-start gap-4">
                    <Label className="text-right text-xs pt-1">Package</Label>
                    <div className="col-span-3 text-sm font-mono">{currentServerDetails.distribution.package}</div>
                  </div>
                </div>
              </div>
            )}
          </div>
          <DialogFooter>
            <Button variant="outline" onClick={closeDetailsPopup}>
              Close
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
          <h1 className="text-2xl font-semibold tracking-tight">MCP Server Registry</h1>
          <Button variant="outline" onClick={openGitHubImportModal}>
            Import From Github
          </Button>
        </div>
        <p className="text-muted-foreground text-sm">
          Discover and install AI applications and MCP tools.
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
            placeholder="Search for tools..."
            aria-label="Search for tools"
          />
        </div>

        {searchTerm && (
          <div className="text-muted-foreground text-sm">
            Found {filteredTools.length} result
            {filteredTools.length !== 1 ? "s" : ""}
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
              {showAllCategories ? "Show Less" : "Show All Categories"}
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
              <p>No tools found matching your search criteria.</p>
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
                                  title="View server details"
                                >
                                  <Info size={14} className="inline-block" />
                                </button>
                              </CardTitle>
                              {tool.installed && (
                                <Badge variant="outline" className="ml-auto">
                                  Installed
                                </Badge>
                              )}
                            </div>
                          </div>
                        </CardHeader>
                        <CardContent>
                          <CardDescription className="line-clamp-2">
                            {tool.description}
                          </CardDescription>
                        </CardContent>
                        <CardFooter className="flex items-center justify-between pt-0">
                          {tool.publisher && (
                            <div className="text-muted-foreground flex items-center gap-1 text-sm">
                              <span>By </span> {tool.publisher.name}
                            </div>
                          )}
                          {tool.installed ? (
                            <Button
                              variant="destructive"
                              onClick={() => uninstallServer(tool.id)}
                              disabled={uninstalling === tool.id}
                              type="button"
                            >
                              {uninstalling === tool.id
                                ? "Uninstalling..."
                                : "Uninstall"}
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
                                ? "Installed"
                                : installing === tool.id
                                  ? "Installing..."
                                  : "Install"}
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
    </div>
  );
};

export default Registry;
