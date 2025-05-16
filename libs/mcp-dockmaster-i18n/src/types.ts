import { TFunction } from 'i18next';
import { TransProps } from 'react-i18next';

export type Locale = 'en_US' | 'es_ES' | 'fr_FR' | 'de_DE' | 'ja_JP' | 'zh_CN';

export interface I18nConfig {
  defaultLanguage: Locale;
  fallbackLanguage: Locale;
  supportedLanguages: Locale[];
  loadPath: string;
}

export interface TranslationResources {
  [key: string]: {
    translation: Record<string, any>;
  };
}

export interface I18nContextValue {
  currentLanguage: Locale;
  setLanguage: (lang: Locale) => void;
  t: TFunction;
  Trans: React.ComponentType<TransProps<any>>;
}

export interface I18nProviderProps {
  config?: Partial<Pick<I18nConfig, 'defaultLanguage' | 'fallbackLanguage' | 'supportedLanguages'>>;
  children: React.ReactNode;
} 