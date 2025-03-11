interface UserConsent {
  termsAccepted: boolean;
  analyticsEnabled: boolean;
  timestamp: number;
}

const USER_CONSENT_KEY = 'mcp-dockmaster-user-consent';

export const getUserConsent = (): UserConsent | null => {
  try {
    const storedConsent = localStorage.getItem(USER_CONSENT_KEY);
    if (!storedConsent) return null;
    return JSON.parse(storedConsent) as UserConsent;
  } catch (error) {
    console.error('Error retrieving user consent from localStorage:', error);
    return null;
  }
};

export const saveUserConsent = (termsAccepted: boolean, analyticsEnabled: boolean): void => {
  try {
    const consent: UserConsent = {
      termsAccepted,
      analyticsEnabled,
      timestamp: Date.now(),
    };
    localStorage.setItem(USER_CONSENT_KEY, JSON.stringify(consent));
  } catch (error) {
    console.error('Error saving user consent to localStorage:', error);
  }
};
