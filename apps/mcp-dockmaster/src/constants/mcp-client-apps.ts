import claudeIcon from "../assets/claude.svg";
import cursorIcon from "../assets/cursor.svg";
import { OsType, type as osType } from '@tauri-apps/plugin-os';
import { checkClaude, checkCursor, isProcessRunning } from "../lib/process";

export type McpClientAppId = 'claude' | 'cursor';
export interface McpClientApp {
    id: McpClientAppId,
    name: string;
    icon: string;
    supportedOS: OsType[];
    isInstalled: () => Promise<boolean>;
    isRunning: () => Promise<boolean>;
}

const MCP_CLIENT_APPS: McpClientApp[] = [{
    id: 'claude',
    name: 'Claude',
    icon: claudeIcon,
    supportedOS: ['macos', 'windows'] as OsType[],
    isInstalled: checkClaude,
    isRunning: () => isProcessRunning('Claude'),
}, {
    id: 'cursor',
    name: 'Cursor',
    icon: cursorIcon,
    supportedOS: ['macos', 'windows', 'linux'] as OsType[],
    isInstalled: checkCursor,
    isRunning: () => isProcessRunning('Cursor'),
}];

const os = osType();
export const SUPPORTED_MCP_CLIENT_APPS: McpClientApp[] = MCP_CLIENT_APPS.filter(app => app.supportedOS.includes(os));
