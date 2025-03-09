import fetch from 'node-fetch';

interface GitHubRepoInfo {
  name: string;
  description: string;
  owner: {
    login: string;
    html_url: string;
  };
  html_url: string;
  default_branch: string;
}

interface RepoContents {
  hasPackageJson: boolean;
  hasPyprojectToml: boolean;
  packageJson?: any;
  pyprojectToml?: any;
}

export interface ParsedRepoInfo {
  id: string;
  name: string;
  description: string;
  publisher: {
    id: string;
    name: string;
    url: string;
  };
  sourceUrl: string;
  runtime: string; // "node", "python", or "docker"
  distribution?: {
    type: string;
    package: string;
  };
}

/**
 * Parse a GitHub repository URL
 * @param url GitHub repository URL
 * @returns Repository owner and name
 */
export const parseGitHubUrl = (url: string): { owner: string; repo: string } | null => {
  try {
    const urlObj = new URL(url);
    if (urlObj.hostname !== 'github.com') {
      return null;
    }
    
    const pathParts = urlObj.pathname.split('/').filter(Boolean);
    if (pathParts.length < 2) {
      return null;
    }
    
    return {
      owner: pathParts[0],
      repo: pathParts[1]
    };
  } catch (error) {
    console.error('Failed to parse GitHub URL:', error);
    return null;
  }
};

/**
 * Fetch GitHub repository metadata
 * @param owner Repository owner
 * @param repo Repository name
 * @returns Repository metadata
 */
export const fetchRepoInfo = async (owner: string, repo: string): Promise<GitHubRepoInfo | null> => {
  try {
    const response = await fetch(`https://api.github.com/repos/${owner}/${repo}`);
    if (!response.ok) {
      throw new Error(`Failed to fetch repository info: ${response.statusText}`);
    }
    return await response.json() as GitHubRepoInfo;
  } catch (error) {
    console.error('Failed to fetch repository info:', error);
    return null;
  }
};

/**
 * Check repository contents to determine runtime type
 * @param owner Repository owner
 * @param repo Repository name
 * @param branch Repository branch
 * @returns Repository contents information
 */
export const checkRepoContents = async (
  owner: string, 
  repo: string, 
  branch: string
): Promise<RepoContents> => {
  const result: RepoContents = {
    hasPackageJson: false,
    hasPyprojectToml: false
  };
  
  try {
    // Check for package.json
    const packageJsonResponse = await fetch(
      `https://raw.githubusercontent.com/${owner}/${repo}/${branch}/package.json`
    );
    
    if (packageJsonResponse.ok) {
      result.hasPackageJson = true;
      result.packageJson = await packageJsonResponse.json();
    }
    
    // Check for pyproject.toml
    const pyprojectTomlResponse = await fetch(
      `https://raw.githubusercontent.com/${owner}/${repo}/${branch}/pyproject.toml`
    );
    
    if (pyprojectTomlResponse.ok) {
      result.hasPyprojectToml = true;
      result.pyprojectToml = await pyprojectTomlResponse.text();
    }
    
    return result;
  } catch (error) {
    console.error('Failed to check repository contents:', error);
    return result;
  }
};

/**
 * Determine runtime type based on repository contents
 * @param contents Repository contents
 * @returns Runtime type ("node", "python", or "docker")
 */
export const determineRuntimeType = (contents: RepoContents): string => {
  if (contents.hasPackageJson) {
    return "node";
  } else if (contents.hasPyprojectToml) {
    return "python";
  } else {
    // Default to node if we can't determine
    return "node";
  }
};

/**
 * Parse GitHub repository information into a format compatible with MCP Server Registry
 * @param repoInfo GitHub repository metadata
 * @param contents Repository contents
 * @returns Parsed repository information
 */
export const parseRepoInfo = (
  repoInfo: GitHubRepoInfo, 
  contents: RepoContents
): ParsedRepoInfo => {
  const runtime = determineRuntimeType(contents);
  
  // Generate a unique ID based on owner and repo name
  const id = `${repoInfo.owner.login}-${repoInfo.name}`.toLowerCase().replace(/[^a-z0-9-]/g, '-');
  
  // Create distribution info based on runtime type
  let distribution;
  if (runtime === "node" && contents.packageJson?.name) {
    distribution = {
      type: "npm",
      package: contents.packageJson.name
    };
  }
  
  return {
    id,
    name: repoInfo.name,
    description: repoInfo.description || `${repoInfo.name} MCP Server`,
    publisher: {
      id: repoInfo.owner.login,
      name: repoInfo.owner.login,
      url: repoInfo.owner.html_url
    },
    sourceUrl: repoInfo.html_url,
    runtime,
    distribution
  };
};

/**
 * Import a server from a GitHub repository URL
 * @param url GitHub repository URL
 * @returns Parsed repository information or null if failed
 */
export const importServerFromGitHub = async (url: string): Promise<ParsedRepoInfo | null> => {
  // Parse the GitHub URL
  const urlInfo = parseGitHubUrl(url);
  if (!urlInfo) {
    console.error('Invalid GitHub URL:', url);
    return null;
  }
  
  // Fetch repository information
  const repoInfo = await fetchRepoInfo(urlInfo.owner, urlInfo.repo);
  if (!repoInfo) {
    console.error('Failed to fetch repository information');
    return null;
  }
  
  // Check repository contents
  const contents = await checkRepoContents(
    urlInfo.owner, 
    urlInfo.repo, 
    repoInfo.default_branch
  );
  
  // Parse repository information
  return parseRepoInfo(repoInfo, contents);
};
