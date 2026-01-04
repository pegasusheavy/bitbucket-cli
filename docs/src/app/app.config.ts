import { ApplicationConfig, provideBrowserGlobalErrorListeners } from '@angular/core';
import { provideRouter } from '@angular/router';

import { routes } from './app.routes';

// FontAwesome configuration
import { FaConfig, FaIconLibrary } from '@fortawesome/angular-fontawesome';
import {
  faHome, faDownload, faLock, faCog, faKey, faFolder, faCodeBranch,
  faBug, faBolt, faTerminal, faScroll, faWrench, faSearch, faBars,
  faChevronRight, faArrowRight, faArrowLeft, faCopy, faCheck, faXmark,
  faExternalLinkAlt, faBook, faRocket, faShieldAlt, faCode, faPlay,
  faStop, faEye, faPlus, faComment, faTimes, faInfoCircle, faCheckCircle,
  faExclamationTriangle, faQuestionCircle, faList, faLayerGroup
} from '@fortawesome/free-solid-svg-icons';
import { faGithub, faBitbucket } from '@fortawesome/free-brands-svg-icons';

export const appConfig: ApplicationConfig = {
  providers: [
    provideBrowserGlobalErrorListeners(),
    provideRouter(routes)
  ]
};

// Icon library setup - call this in main.ts
export function setupFontAwesome(library: FaIconLibrary, config: FaConfig) {
  // Add icons to library
  library.addIcons(
    // Solid icons
    faHome, faDownload, faLock, faCog, faKey, faFolder, faCodeBranch,
    faBug, faBolt, faTerminal, faScroll, faWrench, faSearch, faBars,
    faChevronRight, faArrowRight, faArrowLeft, faCopy, faCheck, faXmark,
    faExternalLinkAlt, faBook, faRocket, faShieldAlt, faCode, faPlay,
    faStop, faEye, faPlus, faComment, faTimes, faInfoCircle, faCheckCircle,
    faExclamationTriangle, faQuestionCircle, faList, faLayerGroup,
    // Brand icons
    faGithub, faBitbucket
  );

  // Optional: Set default prefix
  config.defaultPrefix = 'fas';
}
