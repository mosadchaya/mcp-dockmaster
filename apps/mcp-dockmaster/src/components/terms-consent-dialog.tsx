import { useState } from "react";
import { 
  Dialog, 
  DialogContent, 
  DialogDescription, 
  DialogFooter, 
  DialogHeader, 
  DialogTitle 
} from "./ui/dialog";
import { Button } from "./ui/button";
import { Switch } from "./ui/switch";
import { Label } from "./ui/label";
import { termsOfServiceMarkdown } from "../constants/terms-of-service";
import { saveUserConsent } from "../lib/localStorage";
import { updateAnalyticsConsent } from "../lib/analytics";
import { useTranslation } from "@mcp-dockmaster/i18n";

interface TermsConsentDialogProps {
  open: boolean;
  onAccept: () => void;
}

export const TermsConsentDialog = ({ open, onAccept }: TermsConsentDialogProps) => {
  const { t } = useTranslation();
  const [termsAccepted, setTermsAccepted] = useState(false);
  const [analyticsEnabled, setAnalyticsEnabled] = useState(true);

  const handleAccept = () => {
    saveUserConsent(termsAccepted, analyticsEnabled);
    updateAnalyticsConsent(analyticsEnabled);
    onAccept();
  };

  return (
    <Dialog open={open} onOpenChange={() => {}}>
      <DialogContent className="max-w-3xl max-h-[80vh] flex flex-col">
        <DialogHeader>
          <DialogTitle>{t('terms_consent.title')}</DialogTitle>
          <DialogDescription>
            {t('terms_consent.description')}
          </DialogDescription>
        </DialogHeader>
        
        <div className="flex-1 overflow-auto my-4">
          <div className="rounded-md bg-muted p-4 text-sm">
            <pre className="whitespace-pre-wrap">{termsOfServiceMarkdown}</pre>
          </div>
        </div>
        
        <div className="space-y-4 mb-4">
          <div className="flex items-center space-x-2">
            <Switch 
              id="terms" 
              checked={termsAccepted} 
              onCheckedChange={setTermsAccepted} 
            />
            <Label htmlFor="terms" className="font-medium">
              {t('terms_consent.accept_terms')}
            </Label>
          </div>
          
          <div className="flex items-center space-x-2">
            <Switch 
              id="analytics" 
              checked={analyticsEnabled} 
              onCheckedChange={setAnalyticsEnabled} 
            />
            <Label htmlFor="analytics" className="font-medium">
              {t('terms_consent.enable_analytics')}
            </Label>
          </div>
        </div>
        
        <DialogFooter>
          <Button 
            onClick={handleAccept} 
            disabled={!termsAccepted}
          >
            {t('common.continue')}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};
