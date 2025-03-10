import React, { useState } from "react";
import { 
  Dialog, 
  DialogContent, 
  DialogDescription, 
  DialogFooter, 
  DialogHeader, 
  DialogTitle, 
  DialogTrigger 
} from "./ui/dialog";
import { Button } from "./ui/button";
import { Input } from "./ui/input";
import { importServerFromGitHub } from "../lib/githubUtils";
import { RegistryServer } from "../lib/mcpClient";

interface ImportServerDialogProps {
  onImport: (server: RegistryServer) => Promise<void>;
  open?: boolean;
  onOpenChange?: (open: boolean) => void;
}

const ImportServerDialog: React.FC<ImportServerDialogProps> = ({ 
  onImport, 
  open: externalOpen, 
  onOpenChange: externalOnOpenChange 
}) => {
  const [url, setUrl] = useState("");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [open, setOpen] = useState(false);

  const handleImport = async () => {
    setLoading(true);
    setError(null);
    
    try {
      const serverInfo = await importServerFromGitHub(url);
      if (!serverInfo) {
        throw new Error("Failed to import server from GitHub URL");
      }
      
      // Convert to RegistryServer format
      const server: RegistryServer = {
        ...serverInfo,
        fullDescription: serverInfo.description,
        installed: false,
        categories: []
      };
      
      // Call the onImport callback
      await onImport(server);
      
      // Close the dialog
      setOpen(false);
    } catch (error) {
      console.error("Error importing server:", error);
      setError(error instanceof Error ? error.message : "Failed to import server");
    } finally {
      setLoading(false);
    }
  };

  return (
    <Dialog 
      open={externalOpen !== undefined ? externalOpen : open} 
      onOpenChange={(newOpen) => {
        if (externalOnOpenChange) {
          externalOnOpenChange(newOpen);
        } else {
          setOpen(newOpen);
        }
      }}
    >
      {externalOpen === undefined && (
        <DialogTrigger asChild>
          <Button variant="outline">Import from URL</Button>
        </DialogTrigger>
      )}
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Import MCP Server from GitHub</DialogTitle>
          <DialogDescription>
            Enter a GitHub repository URL to import as an MCP Server.
            Supports both Node.js and Python/UV projects.
          </DialogDescription>
        </DialogHeader>
        
        <div className="grid gap-4 py-4">
          <div className="grid gap-2">
            <Input
              placeholder="https://github.com/username/repository"
              value={url}
              onChange={(e) => setUrl(e.target.value)}
            />
            {error && (
              <p className="text-destructive text-sm">{error}</p>
            )}
          </div>
        </div>
        
        <DialogFooter>
          <Button 
            onClick={handleImport} 
            disabled={!url || loading}
          >
            {loading ? "Importing..." : "Import"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};

export default ImportServerDialog;
