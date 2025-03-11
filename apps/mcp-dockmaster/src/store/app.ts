// create a store for the app

import { create } from "zustand";
import { getUserConsent } from "../lib/localStorage";

interface AppStore {
  appState: "pending" | "ready" | "error";
  setAppState: (state: "pending" | "ready" | "error") => void;
  userConsented: boolean;
  setUserConsented: (consented: boolean) => void;
}

export const useAppStore = create<AppStore>((set) => ({
  appState: "pending",
  setAppState: (state: "pending" | "ready" | "error") => set({ appState: state }),
  userConsented: !!getUserConsent()?.termsAccepted,
  setUserConsented: (consented: boolean) => set({ userConsented: consented }),
}));
