# Implementing a Robust i18n System Similar to Shinkai

## Overview

I need help implementing a robust internationalization (i18n) system for my application that follows these key patterns:

1. Uses i18next with React integration
2. Maintains translations in a structured way with a default language as source of truth
3. Supports automatic language detection and switching
4. Includes a CLI workflow for managing translations
5. Handles components with HTML/JSX elements inside translations

## Core Architecture

Create a library structure with:

- A default translations source (in English) that serves as the source of truth
- A system to generate JSON translation files for all supported languages
- A dynamic loading mechanism to fetch translations on demand
- React components and hooks for easy integration
- Utility functions for standalone usage outside of React components

## Required Features

### 1. Translation Hooks/Components

Implement:
- A custom `useTranslation` hook that returns:
  - `t` function for simple text translations
  - `Trans` component for translations with HTML/JSX elements

### 2. Language Management

- Support a defined list of languages with proper typings
- Function to normalize/detect the user's language
- A method to switch between languages dynamically
- Language detection using browser preferences with fallback to default

### 3. Translation File Structure

- Keep default translations in TypeScript/JavaScript source files
- Generate and maintain JSON files for all supported languages
- Support nested translation keys (e.g., "desktop.welcome", "chat.create")

### 4. Translation CLI Workflow

Set up:
- A script to detect differences between source translations and generated files
- A process to add new translation keys and update existing ones
- Automated translation generation for new languages via AI (optional)

### 5. Provider Component

Create an `I18nProvider` component that:
- Wraps the application
- Initializes the i18n system
- Makes translations available throughout the component tree

## Implementation Details

The system should follow these technical patterns:

1. Use i18next with these extensions:
   - i18next-browser-languagedetector for auto-detection
   - react-i18next for React integration
   - i18next-resources-to-backend for dynamic loading

2. Support translation interpolation:
   - Variable substitution (e.g., `{{count}}`)
   - Pluralization rules (e.g., `_one`, `_other` suffixes)
   - JSX component insertion

3. Include TypeScript support:
   - Type-safe translation keys
   - Type-checking for language codes
   - Type definitions for all exported functions and components

4. Use a scalable file structure:
   - Default translations in source code
   - Generated locale files in a dedicated directory
   - Clean separation between i18n logic and translation content

## Example Usage Patterns

The final implementation should support these usage patterns:

### In React Components

```tsx
import { useTranslation } from '@my-app/i18n';

const MyComponent: React.FC = () => {
  const { t, Trans } = useTranslation();
  
  // Simple text
  return (
    <div>
      <h1>{t('component.title')}</h1>
      <p>{t('component.description')}</p>
      
      {/* With variable replacement */}
      <span>{t('component.count', { count: 5 })}</span>
      
      {/* With HTML/JSX elements */}
      <Trans 
        i18nKey="component.richText" 
        components={{ 
          b: <strong />,
          link: <a href="https://example.com" />
        }} 
      />
    </div>
  );
};
```

### Outside React Components

```ts
import { t } from '@my-app/i18n';

// For use in utility functions, etc.
const getErrorMessage = (code: string) => t(`errors.${code}`);
```

### Language Switching

```tsx
import { switchLanguage } from '@my-app/i18n';

// Switch to a specific language
const handleLanguageChange = (locale: string) => {
  switchLanguage(locale);
};

// Or auto-detect from browser
const useAutoDetect = () => {
  switchLanguage('auto');
};
```

## Development Workflow

The implementation should support this workflow:

1. Add new text in the default language source file
2. Run a command to update all translation JSON files
3. Update translations for other languages (manually or via AI service)
4. Access translations in components via the `useTranslation` hook

## CLI Commands

Implement these commands for managing translations:

1. `generate-i18n`: Update JSON files based on default source
2. `diff-i18n`: Show differences between source and generated files
3. `sync-i18n`: Generate translations for all supported languages

## Additional Requirements

- Support for development mode with debugging enabled
- Production optimization with minimal bundle size
- Clear documentation on how to add new languages
- Support for date/time/number formatting based on locale