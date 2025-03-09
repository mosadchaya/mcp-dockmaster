import { useState } from "react";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";

const feedbackSchema = z.object({
  feedback: z.string().min(5, {
    message: "Feedback must be at least 5 characters.",
  }),
  contact: z.string().min(1, {
    message: "Please provide a way to contact you.",
  }),
});

type FeedbackFormValues = z.infer<typeof feedbackSchema>;

const About = () => {
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [formSuccess, setFormSuccess] = useState(false);
  const [formError, setFormError] = useState("");

  const form = useForm<FeedbackFormValues>({
    resolver: zodResolver(feedbackSchema),
    defaultValues: {
      feedback: "",
      contact: "",
    },
  });

  const onSubmit = async (data: FeedbackFormValues) => {
    setIsSubmitting(true);
    setFormError("");
    
    try {
      // Web3Forms access key
      const formData = new FormData();
      formData.append("access_key", "1234567890abcdef1234567890abcdef");
      formData.append("feedback", data.feedback);
      formData.append("contact", data.contact);
      formData.append("subject", "New Feedback from MCP Dockmaster");
      
      const response = await fetch("https://api.web3forms.com/submit", {
        method: "POST",
        body: formData,
      });
      
      const result = await response.json();
      
      if (result.success) {
        setFormSuccess(true);
        form.reset();
      } else {
        setFormError("Something went wrong. Please try again.");
      }
    } catch (error) {
      setFormError("An error occurred. Please try again later.");
    } finally {
      setIsSubmitting(false);
    }
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
        
        {formSuccess ? (
          <div className="bg-green-50 border border-green-200 text-green-800 rounded-md p-4 mb-4">
            Thank you for your feedback! We'll get back to you soon.
          </div>
        ) : (
          <Form {...form}>
            <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
              <FormField
                control={form.control}
                name="feedback"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Your Feedback</FormLabel>
                    <FormControl>
                      <Textarea placeholder="Please share your thoughts, suggestions, or questions..." {...field} />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
              
              <FormField
                control={form.control}
                name="contact"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Contact Information</FormLabel>
                    <FormControl>
                      <Input placeholder="Email or phone number" {...field} />
                    </FormControl>
                    <FormDescription>
                      How can we reach you if we have questions?
                    </FormDescription>
                    <FormMessage />
                  </FormItem>
                )}
              />
              
              {formError && (
                <div className="bg-red-50 border border-red-200 text-red-800 rounded-md p-4">
                  {formError}
                </div>
              )}
              
              <Button type="submit" disabled={isSubmitting}>
                {isSubmitting ? "Sending..." : "Send Feedback"}
              </Button>
            </form>
          </Form>
        )}
      </section>
    </div>
  );
};

export default About;
