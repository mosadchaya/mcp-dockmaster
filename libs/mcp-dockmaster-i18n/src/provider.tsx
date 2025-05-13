import React, { createContext, useContext, useEffect, useState } from 'react';
import { useTranslation, Trans as I18nTrans } from 'react-i18next';
import { I18nContextValue, I18nProviderProps, Language } from './types';
import { initializeI18n, switchLanguage } from './config';

const I18nContext = createContext<I18nContextValue | null>(null);

export const I18nProvider: React.FC<I18nProviderProps> = ({ children, config }) => {
  const [currentLanguage, setCurrentLanguage] = useState<Language>('en');
  const { t } = useTranslation();

  useEffect(() => {
    initializeI18n(config);
  }, [config]);

  const handleLanguageChange = (lang: Language) => {
    switchLanguage(lang);
    setCurrentLanguage(lang);
  };

  const value: I18nContextValue = {
    currentLanguage,
    setLanguage: handleLanguageChange,
    t,
    Trans: I18nTrans,
  };

  return <I18nContext.Provider value={value}>{children}</I18nContext.Provider>;
};

export const useI18n = () => {
  const context = useContext(I18nContext);
  if (!context) {
    throw new Error('useI18n must be used within an I18nProvider');
  }
  return context;
}; 