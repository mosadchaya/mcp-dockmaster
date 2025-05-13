import i18n, { Resource, ResourceKey } from 'i18next';
import { initReactI18next } from 'react-i18next';
import LanguageDetector from 'i18next-browser-languagedetector';

// Locales constant might be removed or updated if it was specific to the old structure
// import { Locales } from './constants'; 

const supportedLocales = [
  'en', 'en_US', 
  'es', 'es_ES', 
  'fr', 'fr_FR', 
  'de', 'de_DE', 
  'ja', 'ja_JP', 
  'zh', 'zh_CN'
];

const initI18n = async () => {
  console.log('[i18n] Initializing...');

  // Use Vite's import.meta.glob to load locales
  const locales = 
    import.meta.glob('../../locales/*.json');
  const resources: Resource = {};

  for (const path in locales) {
    const lang = path.match(/\.\.\/locales\/(.*)\.json/)?.[1];
    if (lang) {
      console.log(`[i18n] Preparing resources for lang: ${lang}`);
      try {
        const module = await locales[path]() as { default: ResourceKey };
        resources[lang] = {
          translation: module.default,
        };
        console.log(`[i18n] Successfully prepared resources for lang: ${lang}`);
      } catch (error) {
        console.error(`[i18n] Failed to load resources for lang: ${lang} from path: ${path}`, error);
      }
    }
  }

  await i18n
    .use(LanguageDetector)
    .use(initReactI18next)
    .init({
      resources,
      fallbackLng: 'en_US',
      supportedLngs: supportedLocales,
      defaultNS: 'translation',
      fallbackNS: 'translation',
      ns: ['translation'],
      detection: {
        order: ['navigator', 'htmlTag', 'path', 'subdomain'],
        caches: ['localStorage'],
      },
      interpolation: {
        escapeValue: false,
      },
      debug: true,
    }, (err) => {
      if (err) {
        console.error('[i18n] Initialization failed:', err);
      } else {
        console.log('[i18n] Initialization successful.');
        console.log('[i18n] Detected language:', i18n.language);
        console.log('[i18n] Loaded languages:', Object.keys(i18n.store.data));
      }
    });

  i18n.on('languageChanged', (lng) => {
    console.log(`[i18n] Language changed to: ${lng}`);
  });

  return i18n;
};

export const i18NextInstancePromise = initI18n();

let i18nextInstance: typeof i18n | null = null;
i18NextInstancePromise.then(instance => { 
  i18nextInstance = instance;
  console.log('[i18n] Instance assigned after async init.');
});

export const t = (key: string, options?: any) => {
  if (!i18nextInstance) {
    console.warn('[i18n] t function called before instance is ready.');
    return key;
  }
  return i18nextInstance.t(key, options);
};

export * from './useTranslation';

export type LocaleMode = typeof supportedLocales[number] | 'auto';

export const switchLanguage = (locale: LocaleMode) => {
  const lang = locale === 'auto' ? navigator.language : locale;
  if (i18nextInstance) {
    i18nextInstance.changeLanguage(lang);
  } else {
    console.warn('[i18n] switchLanguage called before instance is ready.');
  }
};
