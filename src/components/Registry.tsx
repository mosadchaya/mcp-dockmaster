import React, { useState } from 'react';
import ServerCard from './registry/ServerCard';
import './Registry.css';
import { registry } from '../lib/registry';

interface Server {
  id: string;
  name: string;
  author: string;
  downloads: number;
  description: string;
  repoUrl: string;
}

const ITEMS_PER_PAGE = 9;

const Registry: React.FC = () => {
  const [searchQuery, setSearchQuery] = useState('');
  const [currentPage, setCurrentPage] = useState(1);

  // Convert registry data to Server interface
  const servers: Server[] = registry.map(item => ({
    id: item.id,
    name: item.name,
    author: item.publisher?.name || item.publisher?.id || 'Unknown',
    downloads: Math.floor(Math.random() * 10000), // Random download count as it's not in the original data
    description: item.description,
    repoUrl: item.sourceUrl || ''
  }));

  // Filter servers based on search query
  const filteredServers = servers.filter(server => 
    server.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
    server.description.toLowerCase().includes(searchQuery.toLowerCase()) ||
    server.author.toLowerCase().includes(searchQuery.toLowerCase())
  );

  // Calculate pagination
  const totalPages = Math.ceil(filteredServers.length / ITEMS_PER_PAGE);
  const startIndex = (currentPage - 1) * ITEMS_PER_PAGE;
  const paginatedServers = filteredServers.slice(startIndex, startIndex + ITEMS_PER_PAGE);

  return (
    <div className="registry-container">
      <div className="registry-header">
        <h2>Shinkai AI App Store</h2>
        <p>Extend your language model with {servers.length} capabilities via Shinkai AI Apps.</p>
        <p>Shinkai AI Apps are based on the Model Context Protocol (MCP) which is a protocol for extending language models with custom capabilities.</p>
      </div>

      <div className="search-container">
        <input
          type="text"
          placeholder="Search or prompt for servers..."
          value={searchQuery}
          onChange={(e) => setSearchQuery(e.target.value)}
          className="search-input"
        />
      </div>

      <div className="servers-grid">
        {paginatedServers.map(server => (
          <ServerCard
            key={server.id}
            name={server.name}
            author={server.author}
            downloads={server.downloads}
            description={server.description}
            repoUrl={server.repoUrl}
          />
        ))}
      </div>

      {totalPages > 1 && (
        <div className="pagination">
          <button
            onClick={() => setCurrentPage(p => Math.max(1, p - 1))}
            disabled={currentPage === 1}
          >
            Previous
          </button>
          <span className="page-info">
            Page {currentPage} of {totalPages}
          </span>
          <button
            onClick={() => setCurrentPage(p => Math.min(totalPages, p + 1))}
            disabled={currentPage === totalPages}
          >
            Next
          </button>
        </div>
      )}
    </div>
  );
};

export default Registry; 