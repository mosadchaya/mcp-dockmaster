import { useState } from "react";
import "./App.css";
import homeIcon from "./assets/home.svg";
import serversIcon from "./assets/servers.svg";
import registryIcon from "./assets/registry.svg";
import aboutIcon from "./assets/about.svg";

// Import components
import Home from "./components/Home";
import InstalledServers from "./components/InstalledServers";
import Registry from "./components/Registry";
import About from "./components/About";
import LoadingOverlay from "./components/LoadingOverlay";

function App() {
  const [activeMenu, setActiveMenu] = useState('home');
  const [isInitializing, setIsInitializing] = useState(true);

  const menuItems = [
    { id: 'home', label: 'Home', icon: homeIcon },
    { id: 'installed', label: 'My Applications', icon: serversIcon },
    { id: 'registry', label: 'AI App Store', icon: registryIcon },
    { id: 'about', label: 'About', icon: aboutIcon },
  ];

  const renderContent = () => {
    switch (activeMenu) {
      case 'home':
        return <Home />;
      case 'installed':
        return <InstalledServers />;
      case 'registry':
        return <Registry />;
      case 'about':
        return <About />;
      default:
        return <Home />;
    }
  };

  const handleInitializationComplete = () => {
    setIsInitializing(false);
    
    // Force a re-render of the active component
    setActiveMenu(prevMenu => {
      // This is a trick to force a re-render without changing the actual menu
      setTimeout(() => setActiveMenu(prevMenu), 100);
      return prevMenu;
    });
  };

  return (
    <div className="app-container">
        <nav>
          <ul className="sidebar-menu">
            {menuItems.map((item) => (
              <li
                key={item.id}
                className={`sidebar-menu-item ${activeMenu === item.id ? 'active' : ''}`}
                onClick={() => setActiveMenu(item.id)}
              >
                <img src={item.icon} alt={item.label} />
                <span>{item.label}</span>
              </li>
            ))}
          </ul>
        </nav>
      <main className="main-content">
        {renderContent()}
      </main>
      
      {isInitializing && (
        <LoadingOverlay onInitializationComplete={handleInitializationComplete} />
      )}
    </div>
  );
}

export default App;
