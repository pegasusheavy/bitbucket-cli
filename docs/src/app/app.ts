import { Component } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { SidebarComponent } from './components/sidebar/sidebar.component';
import { HeaderComponent } from './components/header/header.component';

@Component({
  selector: 'app-root',
  imports: [RouterOutlet, SidebarComponent, HeaderComponent],
  template: `
    <div class="flex h-screen overflow-hidden">
      <!-- Sidebar -->
      <app-sidebar class="hidden lg:flex" />

      <!-- Main Content -->
      <div class="flex flex-col flex-1 overflow-hidden">
        <!-- Header -->
        <app-header />

        <!-- Page Content -->
        <main class="flex-1 overflow-y-auto bg-[var(--color-neutral-10)]">
          <router-outlet />
        </main>
      </div>
    </div>
  `,
  styles: ``
})
export class App {
  title = 'Bitbucket CLI Documentation';
}
