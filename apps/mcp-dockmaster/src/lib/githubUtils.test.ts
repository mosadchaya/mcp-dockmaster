import { parseGitHubUrl, fetchRepoInfo, checkRepoContents, determineRuntimeType } from './githubUtils';

// Test GitHub URL parsing
async function testGitHubUrlParsing() {
  console.log('Testing GitHub URL parsing...');
  
  const testUrls = [
    'https://github.com/deepfates/mcp-replicate',
    'https://github.com/toolhouse-community/mcp-server-toolhouse',
    'https://github.com/invalid-url',
    'https://not-github.com/user/repo'
  ];
  
  for (const url of testUrls) {
    const result = parseGitHubUrl(url);
    console.log(`URL: ${url} => ${result ? `owner: ${result.owner}, repo: ${result.repo}` : 'Invalid'}`);
  }
}

// Test fetching repository info
async function testFetchRepoInfo() {
  console.log('\nTesting repository info fetching...');
  
  const repos = [
    { owner: 'deepfates', repo: 'mcp-replicate' },
    { owner: 'toolhouse-community', repo: 'mcp-server-toolhouse' }
  ];
  
  for (const { owner, repo } of repos) {
    const info = await fetchRepoInfo(owner, repo);
    console.log(`Repo: ${owner}/${repo} => ${info ? 'Success' : 'Failed'}`);
    if (info) {
      console.log(`  Name: ${info.name}`);
      console.log(`  Description: ${info.description}`);
      console.log(`  Owner: ${info.owner.login}`);
      console.log(`  URL: ${info.html_url}`);
    }
  }
}

// Test checking repository contents
async function testCheckRepoContents() {
  console.log('\nTesting repository contents checking...');
  
  const repos = [
    { owner: 'deepfates', repo: 'mcp-replicate', branch: 'main' },
    { owner: 'toolhouse-community', repo: 'mcp-server-toolhouse', branch: 'main' }
  ];
  
  for (const { owner, repo, branch } of repos) {
    const contents = await checkRepoContents(owner, repo, branch);
    console.log(`Repo: ${owner}/${repo} => ${JSON.stringify(contents, null, 2)}`);
    
    const runtime = determineRuntimeType(contents);
    console.log(`  Runtime: ${runtime}`);
  }
}

// Run all tests
async function runTests() {
  await testGitHubUrlParsing();
  await testFetchRepoInfo();
  await testCheckRepoContents();
}

runTests().catch(console.error);
