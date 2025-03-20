import claudeIcon from "../assets/claude.svg";
import cursorIcon from "../assets/cursor.svg";
import { OsType, type as osType } from "@tauri-apps/plugin-os";
import {
  checkClaude,
  checkCursor,
  installClaude,
  installCursor,
  isProcessRunning,
} from "../lib/process";

export type McpClientAppId = "claude" | "cursor" | "cursor-after-0470";
export interface McpClientApp {
  id: McpClientAppId;
  name: string;
  icon: string;
  supportedOS: OsType[];
  processName: string;
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
    processName: "Claude",
    isInstalled: checkClaude,
    isRunning: () => isProcessRunning("Claude"),
    install: () => installClaude(),
  },
  {
    id: "cursor",
    name: "Cursor (v0.47+)",
    processName: "Cursor",
    icon: cursorIcon,
    supportedOS: ["macos", "windows", "linux"] as OsType[],
    isInstalled: () => checkCursor(),
    isRunning: () => isProcessRunning("Cursor"),
    install: () => installCursor(),
  },
];

const os = osType();
export const SUPPORTED_MCP_CLIENT_APPS: McpClientApp[] = MCP_CLIENT_APPS.filter(
  (app) => app.supportedOS.includes(os),
);
