import "./index.css";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { I18nProvider } from '@mcp-dockmaster/i18n';

import Home from "./pages/home";
import InstalledServers from "./components/InstalledServers";
import Registry from "./components/Registry";
import About from "./pages/about";
import Feedback from "./pages/feedback";
import InitMcpOverlay from "./components/init-mcp-overlay";
import { getVersion } from "@tauri-apps/api/app";

import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarHeader,
  SidebarInset,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarProvider,
  SidebarRail,
  SidebarSeparator,
  SidebarTrigger,
} from "./components/ui/sidebar";
import { Navigate, NavLink, Route, Routes, useMatch } from "react-router";
import {
  AboutIcon,
  HomeIcon,
  RegistryIcon,
  ServersIcon,
} from "./components/icons";
import { MessageSquare } from "lucide-react";
import { Toaster } from "./components/ui/sonner";
import { cn } from "./lib/utils";
import { TooltipProvider } from "./components/ui/tooltip";
import { useEffect, useState } from "react";

function NavItem({
  icon: Icon,
  label,
  to,
}: {
  icon: React.ElementType;
  label: string;
  to: string;
}) {
  const match = useMatch(to);
  return (
    <NavLink
      to={to}
      className={cn(
        "flex cursor-pointer items-center gap-3 rounded-md px-3 py-2 text-sm font-medium transition-colors",
        match ? "bg-white" : "text-slate-600 opacity-80 hover:opacity-100",
      )}
    >
      <Icon className="size-5" />
      <span>{label}</span>
    </NavLink>
  );
}

function AppSidebar() {
  const [appVersion, setAppVersion] = useState<string | null>(null);
  useEffect(() => {
    getVersion().then((version) => {
      console.log("app version", version);
      setAppVersion(version);
    });
  }, []);
  const items = [
    { id: "home", label: "Home", icon: HomeIcon, to: "/" },
    {
      id: "installed",
      label: "Servers Installed",
      icon: ServersIcon,
      to: "/installed",
    },
    {
      id: "registry",
      label: "MCP Server Registry",
      icon: RegistryIcon,
      to: "/registry",
    },
    { id: "about", label: "About", icon: AboutIcon, to: "/about" },
    { id: "feedback", label: "Feedback", icon: MessageSquare, to: "/feedback" },
  ];

  return (
    <Sidebar>
      <SidebarHeader>
        {/* <img src="/logo.png" alt="MCP Dockmaster" className="h-8 w-8" /> */}
      </SidebarHeader>
      <SidebarContent>
        <SidebarGroup>
          <SidebarMenu>
            {items.map((item) => (
              <SidebarMenuItem key={item.id}>
                <SidebarMenuButton asChild>
                  <NavItem icon={item.icon} label={item.label} to={item.to} />
                </SidebarMenuButton>
              </SidebarMenuItem>
            ))}
          </SidebarMenu>
        </SidebarGroup>
      </SidebarContent>
      <SidebarSeparator />
      <SidebarFooter>
        <div className="text-sidebar-foreground/70 text-center text-xs">
          App Version: {appVersion}
        </div>
      </SidebarFooter>
      <SidebarRail />
    </Sidebar>
  );
}

export const queryClient = new QueryClient();

const AppRoutes = () => {
  return (
    <I18nProvider>
      <QueryClientProvider client={queryClient}>
        <TooltipProvider delayDuration={0}>
          <InitMcpOverlay>
            <SidebarProvider>
              <AppSidebar />
              <SidebarInset>
                <SidebarTrigger className="absolute top-2 left-2" />
                <Routes>
                  <Route path="/" element={<Home />} />
                  <Route path="/installed" element={<InstalledServers />} />
                  <Route path="/registry" element={<Registry />} />
                  <Route path="/about" element={<About />} />
                  <Route path="/feedback" element={<Feedback />} />
                  <Route element={<Navigate replace to={"/"} />} path="*" />
                </Routes>
              </SidebarInset>
            </SidebarProvider>
          </InitMcpOverlay>
        </TooltipProvider>
        <Toaster position="top-right" theme="light" />
      </QueryClientProvider>
    </I18nProvider>
  );
};

export default AppRoutes;
