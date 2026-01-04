import { bootstrapApplication } from '@angular/platform-browser';
import { appConfig, setupFontAwesome } from './app/app.config';
import { App } from './app/app';
import { FaConfig, FaIconLibrary } from '@fortawesome/angular-fontawesome';

bootstrapApplication(App, appConfig)
  .then((appRef) => {
    // Setup FontAwesome after bootstrap
    const library = appRef.injector.get(FaIconLibrary);
    const config = appRef.injector.get(FaConfig);
    setupFontAwesome(library, config);
  })
  .catch((err) => console.error(err));
