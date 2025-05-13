import i18n from 'i18next';
import { initReactI18next } from 'react-i18next';
import LanguageDetector from 'i18next-browser-languagedetector';
import resourcesToBackend from 'i18next-resources-to-backend';
import { I18nConfig, Language } from './types';

export const defaultConfig: I18nConfig = {
  defaultLanguage: 'en_US',
  fallbackLanguage: 'en_US',
  supportedLanguages: ['en_US', 'es_ES', 'fr_FR', 'de_DE', 'ja_JP', 'zh_CN'],
  loadPath: '/locales/{{lng}}/{{ns}}.json',
};

export const initializeI18n = (config: Partial<I18nConfig> = {}) => {
  const finalConfig = { ...defaultConfig, ...config };

  i18n
    .use(LanguageDetector)
    .use(initReactI18next)
    .use(
      resourcesToBackend((language: string, namespace: string) =>
        import(`../locales/${language}/${namespace}.json`)
      )
    )
    .init({
      fallbackLng: finalConfig.fallbackLanguage,
      supportedLngs: finalConfig.supportedLanguages,
      defaultNS: 'translation',
      fallbackNS: 'translation',
      ns: ['translation'],
      load: 'languageOnly',
      detection: {
        order: ['navigator', 'htmlTag', 'path', 'subdomain'],
        caches: ['localStorage'],
      },
      interpolation: {
        escapeValue: false,
      },
    });

  return i18n;
};

export const switchLanguage = (language: Language) => {
  i18n.changeLanguage(language);
};

export const getCurrentLanguage = (): Language => {
  return (i18n.language as Language) || defaultConfig.defaultLanguage;
}; 