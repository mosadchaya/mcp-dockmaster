// Event types
export const TOOL_INSTALLED = 'tool_installed';
export const TOOL_UNINSTALLED = 'tool_uninstalled';
export const TOOL_STATUS_CHANGED = 'tool_status_changed';

// Event dispatcher
export const dispatchToolEvent = (eventType: string, toolId: string) => {
  const event = new CustomEvent(eventType, { 
    detail: { toolId },
    bubbles: true 
  });
  document.dispatchEvent(event);
};

// Helper functions
export const dispatchToolInstalled = (toolId: string) => {
  dispatchToolEvent(TOOL_INSTALLED, toolId);
};

export const dispatchToolUninstalled = (toolId: string) => {
  dispatchToolEvent(TOOL_UNINSTALLED, toolId);
};

export const dispatchToolStatusChanged = (toolId: string) => {
  dispatchToolEvent(TOOL_STATUS_CHANGED, toolId);
}; 