/* eslint-disable @typescript-eslint/no-var-requires */
import { existsSync } from 'node:fs';
import { resolve } from 'node:path';
import { consola } from 'consola';
import { colors } from 'consola/utils';
import { diff } from 'just-diff';
import unset from 'lodash/unset';

import { divider, readJSON, tagWhite, writeJSON } from '../src/lib/utils';

import { fileURLToPath } from 'node:url';
const scriptPath = fileURLToPath(import.meta.url);
const scriptDir = resolve(scriptPath, '..');
const i18nConfigPath = resolve(scriptDir, '../.i18nrc.cjs');
const i18nModule = await import(i18nConfigPath);
const i18nConfig = i18nModule.default;

const DEFAULT_LOCALE = 'en_US';

const defaultTranslationsPath = resolve(scriptDir, '../src/lib/defaultTranslations.ts');

export const entryLocaleJsonFilepath = () =>
  resolve(scriptDir, '../locales', `${DEFAULT_LOCALE}.json`);

export const outputLocaleJsonFilepath = (locale: string) =>
  resolve(scriptDir, '../locales', `${locale}.json`);

const genDiff = async () => {
  /* Compare dev and prod version to remove if any */

  divider('Initiating locale diff');
  // Import the named export from the correct TS file path
  const { defaultTranslations: devJSON } = await import(defaultTranslationsPath);
  const filepath = entryLocaleJsonFilepath();
  if (!existsSync(filepath)) {
    consola.warn(`genDiff - Entry file not found, skipping diff: ${filepath}`);
    return;
  }

  consola.info('genDiff - Checking entry filepath:', filepath);

  const prodJSON = readJSON(filepath);

  const diffResult = diff(prodJSON, devJSON as any);

  const remove = diffResult.filter((item) => item.op === 'remove');
  if (remove.length === 0) {
    consola.success(tagWhite(DEFAULT_LOCALE), colors.gray(filepath));
    return;
  }

  consola.info('genDiff - Items to remove:', remove);

  const clearLocals: string[] = [];

  for (const locale of [i18nConfig.entryLocale, ...i18nConfig.outputLocales]) {
    const localeFilepath = outputLocaleJsonFilepath(locale);
    console.log('localeFilepath', localeFilepath);
    consola.info('genDiff - Processing locale filepath:', localeFilepath);
    if (!existsSync(localeFilepath)) continue;
    const localeJSON = readJSON(localeFilepath);

    for (const item of remove) {
      unset(localeJSON, item.path);
    }

    writeJSON(localeFilepath, localeJSON);
    clearLocals.push(locale);
  }
  consola.info('clear', clearLocals);
  consola.success(tagWhite(DEFAULT_LOCALE), colors.gray(filepath), {
    remove: remove.length,
  });
};

/* Generate default locale */
const genTranslations = async () => {
  divider(`Generating default locale: ${DEFAULT_LOCALE}`);

  // Import the named export from the correct TS file path
  const { defaultTranslations: dataJSON } = await import(defaultTranslationsPath);
  const filepath = entryLocaleJsonFilepath();

  consola.info('genTranslations - Writing to filepath:', filepath);

  writeJSON(filepath, dataJSON);
  consola.success(tagWhite(DEFAULT_LOCALE), colors.gray(filepath));
};

const main = async () => {
  await genDiff();
  await genTranslations();
  consola.success('Script finished successfully.');
};

main();
