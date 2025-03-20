// Event types
export const SERVER_INSTALLED = 'server_installed';
export const SERVER_UNINSTALLED = 'server_uninstalled';
export const SERVER_STATUS_CHANGED = 'server_status_changed';
export const SERVER_COLOR_TAGS_CHANGED = 'server_color_tags_changed';

// Event dispatcher
export const dispatchServerEvent = (eventType: string, serverId: string) => {
  const event = new CustomEvent(eventType, { 
    detail: { toolId: serverId },
    bubbles: true 
  });
  document.dispatchEvent(event);
};

// Helper functions
export const dispatchServerInstalled = (serverId: string) => {
  dispatchServerEvent(SERVER_INSTALLED, serverId);
};

export const dispatchServerUninstalled = (serverId: string) => {
  dispatchServerEvent(SERVER_UNINSTALLED, serverId);
};

export const dispatchServerStatusChanged = (serverId: string) => {
  dispatchServerEvent(SERVER_STATUS_CHANGED, serverId);
}; 

export const dispatchServerColorTagsChanged = (serverId: string) => {
  dispatchServerEvent(SERVER_COLOR_TAGS_CHANGED, serverId);
};    