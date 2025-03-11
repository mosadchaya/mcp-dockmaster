import posthog from 'posthog-js';
import { getUserConsent } from './localStorage';

// Initialize PostHog with environment variables
const initPostHog = () => {
  const consent = getUserConsent();
  
  if (consent?.analyticsEnabled) {
    posthog.init(
      import.meta.env.VITE_POSTHOG_KEY,
      {
        api_host: import.meta.env.VITE_POSTHOG_HOST,
        capture_pageview: true,
        capture_pageleave: true,
        autocapture: true,
        loaded: () => {
          console.log('PostHog loaded successfully');
        },
      }
    );
    console.log('Analytics enabled');
    return true;
  }
  
  console.log('Analytics disabled - user has not consented');
  return false;
};

// Track an event only if user has consented
const trackEvent = (eventName: string, properties?: Record<string, any>) => {
  const consent = getUserConsent();
  
  if (consent?.analyticsEnabled) {
    posthog.capture(eventName, properties);
    return true;
  }
  
  return false;
};

// Update PostHog state when user consent changes
const updateAnalyticsConsent = (analyticsEnabled: boolean) => {
  if (analyticsEnabled) {
    initPostHog();
  } else {
    posthog.opt_out_capturing();
  }
};

export { initPostHog, trackEvent, updateAnalyticsConsent };
