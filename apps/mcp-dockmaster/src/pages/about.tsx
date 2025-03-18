const About = () => {

  return (
    <div className="text-muted-foreground mx-auto flex h-full w-full max-w-4xl flex-col gap-8 px-6 py-10 text-sm">
      <h1 className="text-foreground text-2xl font-semibold tracking-tight">
        About MCP Dockmaster
      </h1>
      <section className="about-section">
        <h2 className="text-foreground mb-4 border-b border-gray-300 pb-2 text-lg font-semibold">
          Overview
        </h2>
        <p>
          MCP Dockmaster is a straightforward tool designed to help you easily install, manage, and monitor AI applications using MCP (Model Context Protocol). MCP is an open-source standard created by Anthropic that allows AI apps like Claude Desktop or Cursor to seamlessly access data from platforms such as Slack or Google Drive, interact with other applications, and connect to APIs.
        </p>
      </section>
      <section className="about-section">
        <h2 className="text-foreground mb-4 border-b border-gray-300 pb-2 text-lg font-semibold">
          Our Purpose
        </h2>
        <p>
          MCP Dockmaster simplifies the management of MCP-compatible applications, providing a user-friendly interface that removes complexity. We believe everyone should have easy access to powerful AI tools, regardless of their technical experience.
        </p>
      </section>
      <section className="about-section">
        <h2 className="text-foreground mb-4 border-b border-gray-300 pb-2 text-lg font-semibold">
          What You Can Do
        </h2>
        <ul className="ml-6 list-disc">
          <li className="mb-2">Quickly install and manage MCP-compatible apps</li>
          <li className="mb-2">View and control your apps from one central spot</li>
          <li className="mb-2">Discover new AI tools through our curated app selection</li>
          <li className="mb-2">Simplify integrations and streamline your workflow</li>
        </ul>
      </section>
      <section className="about-section">
        <h2 className="text-foreground mb-4 border-b border-gray-300 pb-2 text-lg font-semibold">
          GitHub Repository
        </h2>
        <p>
          Visit our <a href="https://github.com/dcSpark/mcp-dockmaster/" className="text-blue-500 hover:underline">GitHub repository</a> to learn more, contribute, or report issues.
        </p>
      </section>
    </div>
  );
};

export default About;
