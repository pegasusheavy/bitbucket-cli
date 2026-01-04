import { Component } from '@angular/core';
import { RouterLink } from '@angular/router';

@Component({
  selector: 'app-not-found',
  standalone: true,
  imports: [RouterLink],
  template: `
    <div class="max-w-4xl mx-auto px-6 py-24 text-center">
      <div class="text-6xl mb-6">🔍</div>
      <h1 class="text-3xl font-bold text-[var(--color-neutral-900)] mb-4">Page Not Found</h1>
      <p class="text-lg text-[var(--color-neutral-400)] mb-8">
        The page you're looking for doesn't exist or has been moved.
      </p>
      <a
        routerLink="/"
        class="inline-flex items-center gap-2 px-6 py-3 bg-[var(--color-bitbucket-blue)] text-white font-medium rounded-lg hover:bg-[var(--color-bitbucket-blue-dark)] transition-colors"
      >
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18"/>
        </svg>
        Back to Home
      </a>
    </div>
  `
})
export class NotFoundComponent {}
