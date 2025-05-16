import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";
import { useTranslation } from "@mcp-dockmaster/i18n";

const Feedback = () => {
  const { t } = useTranslation();
  const [formSuccess, setFormSuccess] = useState(false);

  return (
    <div className="text-muted-foreground mx-auto flex h-full w-full max-w-4xl flex-col gap-8 px-6 py-10 text-sm">
      <h1 className="text-foreground text-2xl font-semibold tracking-tight">
        {t('feedback.title')}
      </h1>
      <section className="feedback-section">
        <p className="mb-4">
          {t('feedback.description')}
        </p>
        
        {formSuccess ? (
          <div className="bg-green-50 border border-green-200 text-green-800 rounded-md p-4 mb-4">
            {t('feedback.success_message')}
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
                    alert(t('feedback.error_message'));
                  }
                })
                .catch((error) => {
                  console.error("Form submission error:", error);
                  alert(t('feedback.error_message'));
                });
            }}
          >
            <div>
              <label htmlFor="feedback" className="block text-sm font-medium mb-1">{t('feedback.feedback_label')}</label>
              <Textarea 
                id="feedback" 
                name="feedback" 
                placeholder={t('feedback.feedback_placeholder')} 
                required 
                className="w-full"
              />
            </div>
            
            <div>
              <label htmlFor="contact" className="block text-sm font-medium mb-1">{t('feedback.contact_label')}</label>
              <Input 
                id="contact" 
                name="contact" 
                type="text" 
                placeholder={t('feedback.contact_placeholder')} 
                required 
                className="w-full"
              />
              <p className="text-xs text-gray-500 mt-1">{t('feedback.contact_description')}</p>
            </div>
            
            <Button type="submit">
              {t('feedback.send_button')}
            </Button>
          </form>
        )}
      </section>
    </div>
  );
};

export default Feedback;
