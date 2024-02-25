import spanishMessages from 'ra-language-spanish';
import englishMessages from 'ra-language-english';
import es from './locales/spanish.json';
import en from './locales/english.json';

spanishMessages.ra.auth = {...spanishMessages.ra.auth, ...es.patch.auth }
spanishMessages.ra.action = {...spanishMessages.ra.action, ...es.patch.action }
spanishMessages.ra.navigation = {...spanishMessages.ra.navigation, ...es.patch.navigation }

englishMessages.ra.auth = {...englishMessages.ra.auth, ...en.patch.auth }
englishMessages.ra.action = {...englishMessages.ra.action, ...en.patch.action }
englishMessages.ra.navigation = {...englishMessages.ra.navigation, ...en.patch.navigation }

export const messages = {
  es: { ...spanishMessages, ...es },
  en: { ...englishMessages, ...en },
};

export const LOCALES = ["en","es"];

let fromBrowser = navigator.language.length > 2 ? navigator.language.substring(1,3) : navigator.language;

if (LOCALES.indexOf(fromBrowser) < 0) {
  fromBrowser = "en";
}

export const browserLocale = fromBrowser;
//export const browserLocale = 'es';
