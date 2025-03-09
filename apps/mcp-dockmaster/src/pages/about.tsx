import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";

const About = () => {
  const [showForm, setShowForm] = useState(false);
  const [formSuccess, setFormSuccess] = useState(false);
  
  const handleShowForm = () => {
    setShowForm(true);
  };

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
          MCP Dockmaster is a powerful application management platform designed
          to simplify the deployment, management, and monitoring of AI
          applications and services. Our platform provides a seamless experience
          for developers and users alike.
        </p>
      </section>
      <section className="about-section">
        <h2 className="text-foreground mb-4 border-b border-gray-300 pb-2 text-lg font-semibold">
          Mission
        </h2>
        <p>
          Our mission is to democratize access to advanced AI tools and
          applications by providing an intuitive interface for managing complex
          software ecosystems. We believe in making technology accessible to
          everyone, regardless of their technical expertise.
        </p>
      </section>
      <section className="about-section">
        <h2 className="text-foreground mb-4 border-b border-gray-300 pb-2 text-lg font-semibold">
          Features
        </h2>
        <ul className="ml-6 list-disc">
          <li className="mb-2">Easy application installation and management</li>
          <li className="mb-2">
            Centralized control panel for all your AI applications
          </li>
          <li className="mb-2">Access to a curated store of AI applications</li>
          <li className="mb-2">Simplified configuration and deployment</li>
          <li className="mb-2">Real-time monitoring and logging</li>
        </ul>
      </section>
      <section className="about-section">
        <h2 className="text-foreground mb-4 border-b border-gray-300 pb-2 text-lg font-semibold">
          Contact
        </h2>
        <p>
          For more information about MCP Dockmaster, please visit our website or
          contact our support team.
        </p>
      </section>
      <section className="about-section">
        <h2 className="text-foreground mb-4 border-b border-gray-300 pb-2 text-lg font-semibold">
          Feedback
        </h2>
        <p className="mb-4">
          We value your feedback! Please let us know your thoughts about MCP Dockmaster
          and how we can improve your experience.
        </p>
        
        {showForm ? (
          formSuccess ? (
            <div className="bg-green-50 border border-green-200 text-green-800 rounded-md p-4 mb-4">
              Thank you for your feedback! We'll get back to you soon.
            </div>
          ) : (
            <form
              // Using the provided Formspree form ID
              action="https://formspree.io/f/mgvawbkv"
              method="POST"
              className="space-y-4"
              onSubmit={() => setFormSuccess(true)}
            >
              <div>
                <label htmlFor="feedback" className="block text-sm font-medium mb-1">Your Feedback</label>
                <Textarea 
                  id="feedback" 
                  name="feedback" 
                  placeholder="Please share your thoughts, suggestions, or questions..." 
                  required 
                  className="w-full"
                />
              </div>
              
              <div>
                <label htmlFor="contact" className="block text-sm font-medium mb-1">Contact Information</label>
                <Input 
                  id="contact" 
                  name="contact" 
                  type="text" 
                  placeholder="Email or phone number" 
                  required 
                  className="w-full"
                />
                <p className="text-xs text-gray-500 mt-1">How can we reach you if we have questions?</p>
              </div>
              
              <Button type="submit">
                Send Feedback
              </Button>
            </form>
          )
        ) : (
          <div className="flex flex-col items-start">
            <p className="mb-4">Click the button below to open our feedback form.</p>
            <Button onClick={handleShowForm}>
              Open Feedback Form
            </Button>
          </div>
        )}
      </section>
    </div>
  );
};

export default About;
