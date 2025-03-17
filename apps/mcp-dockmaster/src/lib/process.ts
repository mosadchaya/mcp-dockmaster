import { invoke } from "@tauri-apps/api/core";

export const checkClaude = async () => {
  try {
    return await invoke<boolean>("check_claude_installed");
  } catch (error) {
    console.error("Failed to check Claude:", error);
    return false;
  }
};

export const checkCursor = async () => {
  try {
    return await invoke<boolean>("check_cursor_installed");
  } catch (error) {
    console.error("Failed to check Cursor:", error);
    return false;
  }
};

export const isProcessRunning = async (process_name: 'Claude' | 'Cursor' | 'Generic'): Promise<boolean> => {
  try {
    return await invoke<boolean>('is_process_running', { process: { process_name } });
  } catch (error) {
    console.error(`Failed to check if ${process_name} is running:`, error);
    return false;
  }
};
