interface CacheEntry {
  timestamp: number;
  data: RegistryTool[];
  categories: [string, number][];
}

let toolsCache: CacheEntry | null = null;
const CACHE_DURATION = 24 * 60 * 60 * 1000; // 24 hours in milliseconds

interface RegistryTool {
  id: string;
  name: string;
  description: string;
  fullDescription: string;
  publisher: {
    id: string;
    name: string;
    url: string;
  };
  isOfficial: boolean;
  sourceUrl: string;
  distribution: {
    type: string;
    package: string;
  };
  license: string;
  runtime: string;
  config: {
    command: string;
    args: string[];
    env: Record<string, any>;
  };
  categories: string[];
}

/**
* Get all available tools from the registry
* @param force If true, bypasses cache and forces a new download
*/
export const getAvailableTools = async (force: boolean = false): Promise<RegistryTool[]> => {
  // Return cached data if available and not expired and force is false
  const cacheValid = toolsCache && (Date.now() - toolsCache.timestamp) < CACHE_DURATION;
  if (!force && cacheValid) {
    return toolsCache!.data;
  }

  try {
    const response = await fetch('https://pub-790f7c5dc69a482998b623212fa27446.r2.dev/db.v0.json');
    if (!response.ok) {
      throw new Error(`Failed to fetch tools: ${response.statusText}`);
    }
    const tools: RegistryTool[] = await response.json();
    
    const categoryCounts: Record<string, number> = {};
    tools.forEach((tool) => {
      tool.categories.forEach((category) => {
        categoryCounts[category] = (categoryCounts[category] || 0) + 1;
      });
    });
    const uniqueCategoriesOrdered = Object.keys(categoryCounts)
      .sort((a, b) => categoryCounts[b] - categoryCounts[a])
      .map((category) => [category, categoryCounts[category]] as [string, number]);
    
    // Update cache
    toolsCache = {
      timestamp: Date.now(),
      data: tools,
      categories: uniqueCategoriesOrdered
    };
    
    return tools;
  } catch (error) {
    console.error('Error fetching available tools:', error);
    // If cache exists, return cached data even if expired
    if (toolsCache?.data) {
      console.warn('Returning expired cached data due to fetch error');
      return toolsCache.data;
    }
    return [];
  }
};

const wait = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));

export const getCategories = async (): Promise<[string, number][]> => {
  while (!toolsCache) await wait(50);
  return toolsCache.categories;
};

export const getCategoryTools = async (category: string): Promise<RegistryTool[]> => {
  while (!toolsCache) await wait(50);
  return toolsCache.data.filter((tool: RegistryTool) => tool.categories.includes(category));  
};

/**
* Get a specific tool by ID
*/
export const getToolById = async (id: string): Promise<RegistryTool | null> => {
  if (toolsCache) {
    return toolsCache.data.find((tool: RegistryTool) => tool.id === id) || null;
  }
  return null;
};

export default {
  getAvailableTools,
  getToolById
};