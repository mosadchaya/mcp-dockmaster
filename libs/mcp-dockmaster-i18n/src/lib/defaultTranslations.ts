export const defaultTranslations = {
  "common": {
    "welcome": "Welcome",
    "loading": "Loading...",
    "error": "An error occurred",
    "retry": "Retry",
    "cancel": "Cancel",
    "save": "Save",
    "delete": "Delete"
  },
  "navigation": {
    "home": "Home",
    "servers_installed": "Servers Installed",
    "mcp_server_registry": "MCP Server Registry",
    "about": "About",
    "feedback": "Feedback",
    "settings": "Settings",
    "profile": "Profile",
    "logout": "Logout",
    "app_version": "App Version:"
  },
  "errors": {
    "required": "This field is required",
    "invalid_email": "Please enter a valid email address",
    "network_error": "Network error. Please check your connection",
    "server_error": "Server error. Please try again later",
    "restart": "Restart"
  },
  "validation": {
    "min_length": "Must be at least {{count}} characters",
    "max_length": "Must be at most {{count}} characters",
    "password_mismatch": "Passwords do not match"
  },
  "home": {
    "title": "Welcome to MCP Dockmaster",
    "refresh": "Refresh",
    "checking": "Checking...",
    "what_is_mcp": "What is MCP?",
    "install_error": "Failed to open installation page for {{toolName}}",
    "install_success": "{{mcpClientApp}} installed successfully! Please restart {{mcpClientApp}} to apply the changes.",
    "restart_success": "{{mcpClientApp}} restarted successfully!",
    "restart_error": "Failed to restart {{mcpClientApp}}",
    "mcp_description": "MCP is an open-source standard from Anthropic that helps AI apps like Claude Desktop or Cursor easily access data from platforms such as Slack and Google Drive, interact with other applications, and connect to APIs.",
    "getting_started": "Getting Started",
    "environment": {
      "title": "Environment Details",
      "description": "Make sure that you have Node.js, Python, and Docker installed so you can run MCPs.",
      "installed": "Installed and running",
      "not_installed": "Not installed or not running",
      "active": "Active",
      "inactive": "Inactive",
      "install": "Install"
    },
    "integration": {
      "title": "Integration Details",
      "description": "Add Dockmaster to Cursor, Claude Desktop, or any other MCP client.",
      "add_to_cursor": "Add to Cursor",
      "add_to_claude": "Add to Claude Desktop",
      "add_to_other": "Add to other MCP client",
      "other": "Other Apps",
      "inactive": "Inactive",
      "active": "Active",
      "other_description": "Add MCP Dockmaster to any other app that supports MCP Servers of type command:"
    },
    "registry": {
      "title": "Registry Details",
      "description": "Browse available MCPs from the registry to extend your AI applications with various capabilities.",
      "view_registry": "View Registry",
    },
    "restart": {
      "title": "Restart Options",
      "description": "Restart your MCP clients to apply the changes and start using your MCPs.",
      "running": "Running",
      "not_running": "Not Running",
      "restart": "Restart"
    }
  },
  "about": {
    "title": "About MCP Dockmaster",
    "overview": {
      "title": "Overview",
      "description": "MCP Dockmaster is a straightforward tool designed to help you easily install, manage, and monitor AI applications using MCP (Model Context Protocol). MCP is an open-source standard created by Anthropic that allows AI apps like Claude Desktop or Cursor to seamlessly access data from platforms such as Slack or Google Drive, interact with other applications, and connect to APIs."
    },
    "purpose": {
      "title": "Our Purpose",
      "description": "MCP Dockmaster simplifies the management of MCP-compatible applications, providing a user-friendly interface that removes complexity. We believe everyone should have easy access to powerful AI tools, regardless of their technical experience."
    },
    "what_you_can_do": {
      "title": "What You Can Do",
      "item1": "Quickly install and manage MCP-compatible apps",
      "item2": "View and control your apps from one central spot",
      "item3": "Discover new AI tools through our curated app selection",
      "item4": "Simplify integrations and streamline your workflow"
    },
    "github": {
      "title": "GitHub Repository",
      "visit": "Visit our ",
      "link_text": "GitHub repository",
      "learn_more": " to learn more, contribute, or report issues."
    }
  },
  "feedback": {
    "title": "Feedback",
    "description": "We value your feedback! Please let us know your thoughts about MCP Dockmaster and how we can improve your experience.",
    "success_message": "Thank you for your feedback! We'll get back to you soon.",
    "error_message": "Form submission failed. Please try again.",
    "feedback_label": "Your Feedback",
    "feedback_placeholder": "Please share your thoughts, suggestions, or questions...",
    "contact_label": "Contact Information",
    "contact_placeholder": "Email or phone number",
    "contact_description": "How can we reach you if we have questions?",
    "send_button": "Send Feedback"
  },
  "registry": {
    "title": "MCP Server Registry",
    "import_button": "Import From Github",
    "description": "Discover and install AI applications and MCP tools.",
    "search_placeholder": "Search for tools...",
    "search_results_found": "Found {{count}} result",
    "search_results_found_plural": "Found {{count}} results",
    "no_results": "No tools found matching your search criteria.",
    "show_less": "Show Less",
    "show_all_categories": "Show All Categories",
    "featured_category": "Featured",
    "installed_badge": "Installed",
    "by_publisher": "By",
    "install_button": "Install",
    "installing_button": "Installing...",
    "uninstall_button": "Uninstall",
    "uninstalling_button": "Uninstalling...",
    "view_details_tooltip": "View server details",
    "import_modal": {
      "title": "Import MCP Server from GitHub",
      "description": "Enter a GitHub repository URL to import a new MCP server. The repository should contain a package.json (for Node.js) or pyproject.toml (for Python) file.",
      "note_title": "Note:",
      "note_description": "We will attempt to extract required environment variables from the repository's README.md file. Please note that this process may not identify all required variables correctly.",
      "url_label": "GitHub Repository URL",
      "url_placeholder": "https://github.com/owner/repo",
      "error_empty_url": "Please enter a GitHub URL",
      "error_invalid_url": "Please enter a valid GitHub repository URL",
      "import_error": "Failed to import server: {{message}}",
      "import_generic_error": "Failed to import server",
      "cancel_button": "Cancel",
      "importing_button": "Importing...",
      "import_button": "Import"
    },
    "env_vars_dialog": {
      "title": "Configure Environment Variables",
      "description_required": "{{serverName}} requires {{count}} environment variable(s) to be configured before installation.",
      "description_optional": "Configure optional environment variables for {{serverName}}.",
      "required_title": "Required Environment Variables",
      "optional_title": "Optional Environment Variables",
      "required_marker": "*",
      "cancel_button": "Cancel",
      "install_button": "Install"
    },
    "details_popup": {
      "title": "{{serverName}}",
      "description": "Server details and information",
      "basic_info_title": "Basic Information",
      "description_label": "Description",
      "id_label": "ID",
      "runtime_label": "Runtime",
      "license_label": "License",
      "categories_label": "Categories",
      "tools_title": "Tools",
      "value_label": "Value: {{value}}",
      "publisher_title": "Publisher",
      "name_label": "Name",
      "url_label": "URL",
      "config_title": "Configuration",
      "command_label": "Command",
      "args_label": "Arguments",
      "env_vars_label": "Environment Variables",
      "distribution_title": "Distribution",
      "type_label": "Type",
      "package_label": "Package",
      "close_button": "Close"
    },
    "confirm_dialog": {
      "title": "Confirm Action",
      "default_description": "Are you sure you want to proceed with this action?",
      "restart_claude_title": "Restart Claude",
      "restart_cursor_title": "Restart Cursor",
      "restart_both_title": "Restart Claude and Cursor",
      "restart_explanation": "Claude and Cursor need to restart so their interfaces can reload the updated list of tools provided by the Model Context Protocol (MCP).",
      "restart_claude_success": "Claude restarted successfully!",
      "restart_cursor_success": "Cursor restarted successfully!",
      "restart_both_success": "Claude and Cursor restarted successfully!",
      "manual_button": "I'll do it manually",
      "restart_claude_button": "Restart Claude",
      "restart_cursor_button": "Restart Cursor",
      "restart_both_button": "Restart Both",
      "confirm_button": "Confirm"
    }
  }
};

export default defaultTranslations; 