import { Locale } from "../types";

export const allowedLocales: Locale[] = ['en_US', 'es_ES', 'fr_FR', 'de_DE', 'ja_JP', 'zh_CN'] as const;


export const normalizeLocale = (locale?: string): string => {
  if (!locale) return 'en_US';

  for (const l of allowedLocales) {
    if (l?.startsWith(locale)) return l;
  }

  return 'en_US';
};

type LocaleOptions = {
  label: string;
  value: Locale;
}[];

export const localeOptions: LocaleOptions = [
  {
    label: 'English',
    value: 'en_US',
  },
  {
    label: '简体中文',
    value: 'zh_CN',
  },
  {
    label: '日本語',
    value: 'ja_JP',
  },
  {
    label: 'Bahasa Indonesia',
    value: 'id_ID',
  },
  {
    label: 'Español',
    value: 'es_ES',
  },
  {
    label: 'Türkiye',
    value: 'tr_TR',
  },
] as LocaleOptions;
