import { useTranslation } from "@mcp-dockmaster/i18n";

const About = () => {
  const { t } = useTranslation();

  return (
    <div className="text-muted-foreground mx-auto flex h-full w-full max-w-4xl flex-col gap-8 px-6 py-10 text-sm">
      <h1 className="text-foreground text-2xl font-semibold tracking-tight">
        {t('about.title')}
      </h1>
      <section className="about-section">
        <h2 className="text-foreground mb-4 border-b border-gray-300 pb-2 text-lg font-semibold">
          {t('about.overview.title')}
        </h2>
        <p>
          {t('about.overview.description')}
        </p>
      </section>
      <section className="about-section">
        <h2 className="text-foreground mb-4 border-b border-gray-300 pb-2 text-lg font-semibold">
          {t('about.purpose.title')}
        </h2>
        <p>
          {t('about.purpose.description')}
        </p>
      </section>
      <section className="about-section">
        <h2 className="text-foreground mb-4 border-b border-gray-300 pb-2 text-lg font-semibold">
          {t('about.what_you_can_do.title')}
        </h2>
        <ul className="ml-6 list-disc">
          <li className="mb-2">{t('about.what_you_can_do.item1')}</li>
          <li className="mb-2">{t('about.what_you_can_do.item2')}</li>
          <li className="mb-2">{t('about.what_you_can_do.item3')}</li>
          <li className="mb-2">{t('about.what_you_can_do.item4')}</li>
        </ul>
      </section>
      <section className="about-section">
        <h2 className="text-foreground mb-4 border-b border-gray-300 pb-2 text-lg font-semibold">
          {t('about.github.title')}
        </h2>
        <p>
          {t('about.github.visit')}<a href="https://github.com/dcSpark/mcp-dockmaster/" className="text-blue-500 hover:underline">{t('about.github.link_text')}</a>{t('about.github.learn_more')}
        </p>
      </section>
    </div>
  );
};

export default About;
