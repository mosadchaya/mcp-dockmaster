import "./index.css";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { I18nProvider, useTranslation } from '@mcp-dockmaster/i18n';

import Home from "./pages/home";
import InstalledServers from "./components/InstalledServers";
import Registry from "./components/Registry";
import CustomServerRegistry from "./components/CustomServerRegistry";
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
  CustomServerIcon,
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
  const { t } = useTranslation();
  const [appVersion, setAppVersion] = useState<string | null>(null);
  useEffect(() => {
    getVersion().then((version) => {
      console.log("app version", version);
      setAppVersion(version);
    });
  }, []);
  const items = [
    { id: "home", label: t('navigation.home'), icon: HomeIcon, to: "/" },
    {
      id: "installed",
      label: t('navigation.servers_installed'),
      icon: ServersIcon,
      to: "/installed",
    },
    {
      id: "registry",
      label: t('navigation.mcp_server_registry'),
      icon: RegistryIcon,
      to: "/registry",
    },
    {
      id: "custom-registry",
      label: "Custom Server Registry",
      icon: CustomServerIcon,
      to: "/custom-registry",
    },
    { id: "about", label: t('navigation.about'), icon: AboutIcon, to: "/about" },
    { id: "feedback", label: t('navigation.feedback'), icon: MessageSquare, to: "/feedback" },
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
          {t('navigation.app_version')} {appVersion}
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
                  <Route path="/custom-registry" element={<CustomServerRegistry />} />
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
