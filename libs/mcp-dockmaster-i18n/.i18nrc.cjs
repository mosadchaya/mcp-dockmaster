const { defineConfig } = require('@lobehub/i18n-cli');

module.exports = defineConfig({
  entry: 'locales/en_US.json',
  entryLocale: 'en_US',
  output: 'locales',
  outputLocales: ['es_ES', 'fr_FR', 'de_DE', 'ja_JP', 'zh_CN'],
  temperature: 0,
  modelName: 'gpt-4o-mini',
  splitToken: 2048,
  experimental: {
    jsonMode: true,
  },
});