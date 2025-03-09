import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";

const Feedback = () => {
  const [formSuccess, setFormSuccess] = useState(false);

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
        
        {formSuccess ? (
          <div className="bg-green-50 border border-green-200 text-green-800 rounded-md p-4 mb-4">
            Thank you for your feedback! We'll get back to you soon.
          </div>
        ) : (
          <form
            // Using the provided Formspree form ID
            action="https://formspree.io/f/mgvawbkv"
            method="POST"
            className="space-y-4"
            onSubmit={(e) => {
              e.preventDefault();
              
              // Get form data
              const formData = new FormData(e.target as HTMLFormElement);
              
              // Submit form data to Formspree
              fetch("https://formspree.io/f/mgvawbkv", {
                method: "POST",
                body: formData,
                headers: {
                  Accept: "application/json",
                },
              })
                .then((response) => {
                  if (response.ok) {
                    setFormSuccess(true);
                  } else {
                    // Handle error
                    console.error("Form submission failed");
                    alert("Form submission failed. Please try again.");
                  }
                })
                .catch((error) => {
                  console.error("Form submission error:", error);
                  alert("Form submission failed. Please try again.");
                });
            }}
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
        )}
      </section>
    </div>
  );
};

export default Feedback;
