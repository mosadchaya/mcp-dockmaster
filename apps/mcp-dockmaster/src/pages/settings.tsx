import { useState, useEffect } from "react";
import { useTranslation } from "@mcp-dockmaster/i18n";
import { Button } from "@/components/ui/button";
import { Label } from "@/components/ui/label";
import { 
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { toast } from "sonner";
import { ChevronDown } from "lucide-react";

// Define locale type locally since it's not exported from the main index
type Locale = 'en_US' | 'es_ES' | 'fr_FR' | 'de_DE' | 'ja_JP' | 'zh_CN';

// Language options with native names for better UX
const languageOptions = [
  { value: 'en_US' as Locale, label: 'English', nativeName: 'English' },
  { value: 'es_ES' as Locale, label: 'Spanish', nativeName: 'Español' },
  { value: 'fr_FR' as Locale, label: 'French', nativeName: 'Français' },
  { value: 'de_DE' as Locale, label: 'German', nativeName: 'Deutsch' },
  { value: 'ja_JP' as Locale, label: 'Japanese', nativeName: '日本語' },
  { value: 'zh_CN' as Locale, label: 'Chinese (Simplified)', nativeName: '简体中文' },
];

const Settings = () => {
  const { t, i18n } = useTranslation();
  const [selectedLanguage, setSelectedLanguage] = useState<Locale>('en_US');
  const [isChangingLanguage, setIsChangingLanguage] = useState(false);

  useEffect(() => {
    // Get the current language from i18next
    const currentLang = i18n.language as Locale;
    if (currentLang && languageOptions.find(opt => opt.value === currentLang)) {
      setSelectedLanguage(currentLang);
    }
  }, [i18n.language]);

  const handleLanguageChange = async (newLanguage: Locale) => {
    if (newLanguage === selectedLanguage) return;
    
    setIsChangingLanguage(true);
    try {
      // Use the i18n instance to change language directly
      await i18n.changeLanguage(newLanguage);
      setSelectedLanguage(newLanguage);
      
      // Note: localStorage is now handled automatically by the i18n system
      // Note: Full UI re-render is handled by the key prop in App.tsx
      
      toast.success(t('settings.language.changed_success'));
      
    } catch (error) {
      console.error('Failed to change language:', error);
      toast.error(t('settings.language.changed_error'));
    } finally {
      setIsChangingLanguage(false);
    }
  };

  const getCurrentLanguageLabel = () => {
    const currentOption = languageOptions.find(opt => opt.value === selectedLanguage);
    return currentOption ? `${currentOption.nativeName} (${currentOption.label})` : selectedLanguage;
  };

  return (
    <div className="mx-auto flex h-full w-full max-w-4xl flex-col gap-8 px-6 py-10">
      <div className="flex flex-col space-y-1.5">
        <h1 className="text-2xl font-semibold tracking-tight">
          {t('settings.title')}
        </h1>
        <p className="text-muted-foreground text-sm">
          {t('settings.description')}
        </p>
      </div>

      <div className="space-y-6">
        {/* Language Settings Card */}
        <Card>
          <CardHeader>
            <CardTitle className="text-lg">{t('settings.language.title')}</CardTitle>
            <CardDescription>
              {t('settings.language.description')}
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="flex flex-col space-y-2">
              <Label htmlFor="language-select">{t('settings.language.select_label')}</Label>
              <DropdownMenu>
                <DropdownMenuTrigger asChild>
                  <Button 
                    variant="outline" 
                    className="w-full max-w-xs justify-between"
                    disabled={isChangingLanguage}
                  >
                    {getCurrentLanguageLabel()}
                    <ChevronDown className="h-4 w-4" />
                  </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent className="w-full max-w-xs">
                  {languageOptions.map((option) => (
                    <DropdownMenuItem 
                      key={option.value} 
                      onClick={() => handleLanguageChange(option.value)}
                      className="cursor-pointer"
                    >
                      <div className="flex flex-col">
                        <span className="font-medium">{option.nativeName}</span>
                        <span className="text-sm text-muted-foreground">{option.label}</span>
                      </div>
                    </DropdownMenuItem>
                  ))}
                </DropdownMenuContent>
              </DropdownMenu>
            </div>
            
            <div className="flex items-center justify-between">
              <div className="text-sm text-muted-foreground">
                {t('settings.language.current_language')}: <span className="font-medium">{getCurrentLanguageLabel()}</span>
              </div>
              {isChangingLanguage && (
                <div className="text-sm text-muted-foreground">
                  {t('settings.language.changing')}
                </div>
              )}
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  );
};

export default Settings; 