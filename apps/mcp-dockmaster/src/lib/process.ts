import { invoke } from "@tauri-apps/api/core";

export const checkClaude = async () => {
  try {
    return await invoke<boolean>("check_claude_installed");
  } catch (error) {
    console.error("Failed to check Claude:", error);
    return false;
  }
};

export const installClaude = async () => {
  try {
    return await invoke<void>("install_claude");
  } catch (error) {
    console.error("Failed to install Claude:", error);
  }
};

export const checkCursor = async (after0470: boolean) => {
  try {
    return await invoke<boolean>("check_cursor_installed", { after0470 });
  } catch (error) {
    console.error("Failed to check Cursor:", error);
    throw error;
  }
};

export const installCursor = async (after0470: boolean) => {
  try {
    return await invoke<void>("install_cursor", { after0470 });
  } catch (error) {
    console.error("Failed to install Cursor:", error);
    throw error;
  }
};


export const isProcessRunning = async (
  process_name: "Claude" | "Cursor" | "Generic",
): Promise<boolean> => {
  try {
    return await invoke<boolean>("is_process_running", {
      process: { process_name },
    });
  } catch (error) {
    console.error(`Failed to check if ${process_name} is running:`, error);
    return false;
  }
};

export const getMCPProxyServerBinaryPath = async (): Promise<string> => {
  try {
    return await invoke<string>("get_mcp_proxy_server_binary_path");
  } catch (error) {
    console.error(`failed to get mcp proxy server binary path:`, error);
    return "";
  }
};
