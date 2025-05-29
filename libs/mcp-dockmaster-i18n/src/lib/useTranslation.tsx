import {
  Trans as I18NextTrans,
  useTranslation as useI18NextTranslation,
} from 'react-i18next';

export const useTranslation = () => {
  const { t, i18n } = useI18NextTranslation();
  const Trans = I18NextTrans;

  return {
    t,
    i18n,
    Trans,
  };
};
