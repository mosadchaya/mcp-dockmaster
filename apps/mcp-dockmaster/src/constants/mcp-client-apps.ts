import claudeIcon from "../assets/claude.svg";
import cursorIcon from "../assets/cursor.svg";
import { OsType, type as osType } from "@tauri-apps/plugin-os";
import { checkClaude, checkCursor, installClaude, installCursor, isProcessRunning } from "../lib/process";

export type McpClientAppId = "claude" | "cursor" | "cursor-after-0470";
export interface McpClientApp {
  id: McpClientAppId;
  name: string;
  icon: string;
  supportedOS: OsType[];
  isInstalled: () => Promise<boolean>;
  isRunning: () => Promise<boolean>;
  install: () => Promise<void>;
}

const MCP_CLIENT_APPS: McpClientApp[] = [
  {
    id: "claude",
    name: "Claude",
    icon: claudeIcon,
    supportedOS: ["macos", "windows"] as OsType[],
    isInstalled: checkClaude,
    isRunning: () => isProcessRunning("Claude"),
    install: () => installClaude(),
  },
  {
    id: "cursor",
    name: "Cursor before v0.47",
    icon: cursorIcon,
    supportedOS: ["macos", "windows", "linux"] as OsType[],
    isInstalled: () => checkCursor(false),
    isRunning: () => isProcessRunning("Cursor"),
    install: () => installCursor(false),
  },
  {
    id: "cursor-after-0470",
    name: "Cursor since v0.47+",
    icon: cursorIcon,
    supportedOS: ["macos", "windows", "linux"] as OsType[],
    isInstalled: () => checkCursor(true),
    isRunning: () => isProcessRunning("Cursor"),
    install: () => installCursor(true),
  },
];

const os = osType();
export const SUPPORTED_MCP_CLIENT_APPS: McpClientApp[] = MCP_CLIENT_APPS.filter(
  (app) => app.supportedOS.includes(os),
);
