import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';

@Component({
  selector: 'app-authentication',
  standalone: true,
  imports: [CommonModule],
  template: `
    <div class="max-w-4xl mx-auto px-6 py-12">
      <!-- Page Header -->
      <div class="mb-12">
        <nav class="flex items-center gap-2 text-sm text-[var(--color-neutral-400)] mb-4">
          <a routerLink="/" class="hover:text-[var(--color-bitbucket-blue)]">Docs</a>
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/>
          </svg>
          <span class="text-[var(--color-neutral-700)]">Authentication</span>
        </nav>
        <h1 class="text-3xl font-bold text-[var(--color-neutral-900)] mb-4">Authentication</h1>
        <p class="text-lg text-[var(--color-neutral-400)]">
          Learn how to authenticate with Bitbucket Cloud using the CLI.
        </p>
      </div>

      <!-- Auth Methods Overview -->
      <section class="mb-12">
        <h2 class="text-xl font-semibold text-[var(--color-neutral-800)] mb-4">Authentication Methods</h2>
        <div class="bg-white rounded-xl border border-[var(--color-neutral-30)] overflow-hidden">
          <div class="p-6 border-b border-[var(--color-neutral-30)]">
            <div class="flex items-start gap-4">
              <div class="w-12 h-12 bg-[var(--color-success-light)] rounded-lg flex items-center justify-center flex-shrink-0">
                <span class="text-2xl">🔑</span>
              </div>
              <div>
                <h3 class="font-semibold text-[var(--color-neutral-800)] flex items-center gap-2">
                  App Password
                  <span class="px-2 py-0.5 bg-[var(--color-success-light)] text-[var(--color-success)] text-xs rounded font-medium">Recommended</span>
                </h3>
                <p class="text-sm text-[var(--color-neutral-400)] mt-1">
                  Simple and secure. Create an app password in your Bitbucket settings and use it to authenticate.
                </p>
              </div>
            </div>
          </div>
          <div class="p-6">
            <div class="flex items-start gap-4">
              <div class="w-12 h-12 bg-[var(--color-bitbucket-blue-50)] rounded-lg flex items-center justify-center flex-shrink-0">
                <span class="text-2xl">🌐</span>
              </div>
              <div>
                <h3 class="font-semibold text-[var(--color-neutral-800)]">OAuth 2.0</h3>
                <p class="text-sm text-[var(--color-neutral-400)] mt-1">
                  Browser-based authentication flow. Opens your browser to authorize access.
                </p>
              </div>
            </div>
          </div>
        </div>
      </section>

      <!-- App Password Setup -->
      <section class="mb-12">
        <h2 class="text-xl font-semibold text-[var(--color-neutral-800)] mb-4">Setting up App Password</h2>

        <div class="space-y-4">
          <!-- Step 1 -->
          <div class="bg-white rounded-xl border border-[var(--color-neutral-30)] p-6">
            <h3 class="font-semibold text-[var(--color-neutral-800)] mb-3 flex items-center gap-2">
              <span class="w-6 h-6 bg-[var(--color-bitbucket-blue)] text-white rounded-full flex items-center justify-center text-xs">1</span>
              Create an App Password
            </h3>
            <ol class="space-y-2 text-sm text-[var(--color-neutral-600)] ml-8">
              <li>Go to <a href="https://bitbucket.org/account/settings/app-passwords/" target="_blank" class="text-[var(--color-bitbucket-blue)] hover:underline">Bitbucket App Passwords</a></li>
              <li>Click <strong>"Create app password"</strong></li>
              <li>Give it a label (e.g., "CLI Access")</li>
              <li>Select the required permissions:
                <ul class="mt-2 ml-4 space-y-1">
                  <li>• <code class="bg-[var(--color-neutral-20)] px-1.5 py-0.5 rounded text-xs">Account: Read</code></li>
                  <li>• <code class="bg-[var(--color-neutral-20)] px-1.5 py-0.5 rounded text-xs">Repositories: Read, Write, Admin</code></li>
                  <li>• <code class="bg-[var(--color-neutral-20)] px-1.5 py-0.5 rounded text-xs">Pull requests: Read, Write</code></li>
                  <li>• <code class="bg-[var(--color-neutral-20)] px-1.5 py-0.5 rounded text-xs">Issues: Read, Write</code></li>
                  <li>• <code class="bg-[var(--color-neutral-20)] px-1.5 py-0.5 rounded text-xs">Pipelines: Read, Write</code></li>
                </ul>
              </li>
              <li>Click <strong>"Create"</strong> and copy the generated password</li>
            </ol>
          </div>

          <!-- Step 2 -->
          <div class="bg-white rounded-xl border border-[var(--color-neutral-30)] p-6">
            <h3 class="font-semibold text-[var(--color-neutral-800)] mb-3 flex items-center gap-2">
              <span class="w-6 h-6 bg-[var(--color-bitbucket-blue)] text-white rounded-full flex items-center justify-center text-xs">2</span>
              Authenticate with the CLI
            </h3>
            <p class="text-sm text-[var(--color-neutral-600)] mb-4 ml-8">
              Run the login command and enter your credentials:
            </p>
            <div class="bg-[var(--color-neutral-900)] rounded-lg p-4 ml-8">
              <code class="text-[var(--color-bitbucket-blue-light)] font-mono">bitbucket auth login</code>
            </div>
            <p class="text-sm text-[var(--color-neutral-400)] mt-4 ml-8">
              You'll be prompted to enter your Bitbucket username and the app password you created.
            </p>
          </div>

          <!-- Step 3 -->
          <div class="bg-white rounded-xl border border-[var(--color-neutral-30)] p-6">
            <h3 class="font-semibold text-[var(--color-neutral-800)] mb-3 flex items-center gap-2">
              <span class="w-6 h-6 bg-[var(--color-bitbucket-blue)] text-white rounded-full flex items-center justify-center text-xs">3</span>
              Verify Authentication
            </h3>
            <p class="text-sm text-[var(--color-neutral-600)] mb-4 ml-8">
              Check your authentication status:
            </p>
            <div class="bg-[var(--color-neutral-900)] rounded-lg p-4 ml-8">
              <code class="text-[var(--color-bitbucket-blue-light)] font-mono">bitbucket auth status</code>
            </div>
          </div>
        </div>
      </section>

      <!-- Security Note -->
      <section class="mb-12">
        <div class="bg-[var(--color-bitbucket-blue-50)] border border-[var(--color-bitbucket-blue-100)] rounded-xl p-6">
          <div class="flex gap-4">
            <div class="flex-shrink-0">
              <svg class="w-6 h-6 text-[var(--color-bitbucket-blue)]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
              </svg>
            </div>
            <div>
              <h3 class="font-semibold text-[var(--color-bitbucket-blue-dark)]">Secure Storage</h3>
              <p class="text-sm text-[var(--color-bitbucket-blue-dark)] mt-1">
                Your credentials are stored securely in your system's keyring (Keychain on macOS,
                Secret Service on Linux, Credential Manager on Windows). They are never stored in plain text.
              </p>
            </div>
          </div>
        </div>
      </section>

      <!-- Logout -->
      <section>
        <h2 class="text-xl font-semibold text-[var(--color-neutral-800)] mb-4">Logging Out</h2>
        <div class="bg-white rounded-xl border border-[var(--color-neutral-30)] p-6">
          <p class="text-sm text-[var(--color-neutral-600)] mb-4">
            To remove your stored credentials:
          </p>
          <div class="bg-[var(--color-neutral-900)] rounded-lg p-4">
            <code class="text-[var(--color-bitbucket-blue-light)] font-mono">bitbucket auth logout</code>
          </div>
        </div>
      </section>
    </div>
  `
})
export class AuthenticationComponent {}
