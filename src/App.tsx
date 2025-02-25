import { useState } from "react";
import "./App.css";
import homeIcon from "./assets/home.svg";
import serversIcon from "./assets/servers.svg";
import registryIcon from "./assets/registry.svg";

// Import components
import Home from "./components/Home";
import InstalledServers from "./components/InstalledServers";
import Registry from "./components/Registry";

function App() {
  const [activeMenu, setActiveMenu] = useState('home');

  const menuItems = [
    { id: 'home', label: 'Home', icon: homeIcon },
    { id: 'installed', label: 'My Applications', icon: serversIcon },
    { id: 'registry', label: 'AI App Store', icon: registryIcon },
  ];

  const renderContent = () => {
    switch (activeMenu) {
      case 'home':
        return <Home />;
      case 'installed':
        return <InstalledServers />;
      case 'registry':
        return <Registry />;
      default:
        return <Home />;
    }
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
    </div>
  );
}

export default App;
