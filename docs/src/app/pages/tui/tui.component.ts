import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';

@Component({
  selector: 'app-tui',
  standalone: true,
  imports: [CommonModule],
  template: `
    <div class="max-w-4xl mx-auto px-6 py-12">
      <div class="mb-12">
        <nav class="flex items-center gap-2 text-sm text-[var(--color-neutral-400)] mb-4">
          <a routerLink="/" class="hover:text-[var(--color-bitbucket-blue)]">Docs</a>
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/>
          </svg>
          <span class="text-[var(--color-neutral-700)]">TUI Mode</span>
        </nav>
        <h1 class="text-3xl font-bold text-[var(--color-neutral-900)] mb-4">Interactive TUI Mode</h1>
        <p class="text-lg text-[var(--color-neutral-400)]">
          A beautiful terminal user interface for browsing and managing your Bitbucket resources.
        </p>
      </div>

      <!-- Launch -->
      <section class="mb-12">
        <h2 class="text-xl font-semibold text-[var(--color-neutral-800)] mb-4">Launching the TUI</h2>
        <div class="bg-white rounded-xl border border-[var(--color-neutral-30)] p-6">
          <div class="bg-[var(--color-neutral-900)] rounded-lg p-4 mb-4">
            <code class="text-[var(--color-bitbucket-blue-light)] font-mono">bitbucket tui</code>
          </div>
          <p class="text-sm text-[var(--color-neutral-400)]">
            Or specify a workspace:
          </p>
          <div class="bg-[var(--color-neutral-900)] rounded-lg p-4 mt-2">
            <code class="text-[var(--color-bitbucket-blue-light)] font-mono">bitbucket tui --workspace myworkspace</code>
          </div>
        </div>
      </section>

      <!-- Features -->
      <section class="mb-12">
        <h2 class="text-xl font-semibold text-[var(--color-neutral-800)] mb-4">Features</h2>
        <div class="grid md:grid-cols-2 gap-4">
          @for (feature of features; track feature.title) {
            <div class="bg-white rounded-xl border border-[var(--color-neutral-30)] p-6">
              <div class="text-2xl mb-3">{{ feature.icon }}</div>
              <h3 class="font-semibold text-[var(--color-neutral-800)] mb-2">{{ feature.title }}</h3>
              <p class="text-sm text-[var(--color-neutral-400)]">{{ feature.description }}</p>
            </div>
          }
        </div>
      </section>

      <!-- Keyboard Shortcuts -->
      <section>
        <h2 class="text-xl font-semibold text-[var(--color-neutral-800)] mb-4">Keyboard Shortcuts</h2>
        <div class="bg-white rounded-xl border border-[var(--color-neutral-30)] overflow-hidden">
          <table class="w-full">
            <thead class="bg-[var(--color-neutral-10)]">
              <tr>
                <th class="text-left px-4 py-3 text-sm font-medium text-[var(--color-neutral-600)]">Key</th>
                <th class="text-left px-4 py-3 text-sm font-medium text-[var(--color-neutral-600)]">Action</th>
              </tr>
            </thead>
            <tbody>
              @for (shortcut of shortcuts; track shortcut.key) {
                <tr class="border-t border-[var(--color-neutral-30)]">
                  <td class="px-4 py-3">
                    <kbd class="px-2 py-1 bg-[var(--color-neutral-20)] rounded text-sm font-mono">{{ shortcut.key }}</kbd>
                  </td>
                  <td class="px-4 py-3 text-sm text-[var(--color-neutral-600)]">{{ shortcut.action }}</td>
                </tr>
              }
            </tbody>
          </table>
        </div>
      </section>
    </div>
  `
})
export class TuiComponent {
  features = [
    { icon: '📊', title: 'Dashboard', description: 'Overview of your workspace with quick stats' },
    { icon: '📁', title: 'Repository Browser', description: 'Browse and search your repositories' },
    { icon: '🔀', title: 'Pull Request List', description: 'View and manage pull requests with status indicators' },
    { icon: '🐛', title: 'Issue Tracker', description: 'Browse issues with labels and priorities' },
  ];

  shortcuts = [
    { key: 'q', action: 'Quit the TUI' },
    { key: '1-5', action: 'Switch between views' },
    { key: 'j / ↓', action: 'Move selection down' },
    { key: 'k / ↑', action: 'Move selection up' },
    { key: 'Enter', action: 'Select / Open item' },
    { key: 'r', action: 'Refresh data' },
    { key: 'Esc', action: 'Clear errors / Go back' },
  ];
}
