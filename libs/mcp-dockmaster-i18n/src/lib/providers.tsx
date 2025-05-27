import React, { useState, useEffect } from 'react';
import { I18nextProvider } from 'react-i18next';
import type { i18n as I18nType } from 'i18next'; // Import the type

// Import the promise that resolves with the instance
import { i18NextInstancePromise } from './init';

export const I18nProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [instance, setInstance] = useState<I18nType | null>(null);

  useEffect(() => {
    i18NextInstancePromise.then((resolvedInstance) => {
      setInstance(resolvedInstance);
    });
  }, []); // Run only once on mount

  // Render children only when the instance is ready
  if (!instance) {
    // Optional: Render a loading component here
    return null; 
  }

  return <I18nextProvider i18n={instance}>{children}</I18nextProvider>;
};
