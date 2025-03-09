import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";

const Feedback = () => {
  const [showForm, setShowForm] = useState(false);
  const [formSuccess, setFormSuccess] = useState(false);
  
  const handleShowForm = () => {
    setShowForm(true);
  };

  return (
    <div className="text-muted-foreground mx-auto flex h-full w-full max-w-4xl flex-col gap-8 px-6 py-10 text-sm">
      <h1 className="text-foreground text-2xl font-semibold tracking-tight">
        Feedback
      </h1>
      <section className="feedback-section">
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

export default Feedback;
