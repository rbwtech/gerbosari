<script lang="ts">
  import { onMount } from 'svelte';
  import { navigate } from '../../lib/router';
  import { login } from '../../lib/api/auth';
  import { setAuth, getAuth, isAuthenticated } from '../../lib/auth';
  import { isApiError } from '../../lib/api/client';
  import Button from '../../lib/components/ui/Button.svelte';
  import { Lock, AlertCircle } from '../../lib/components/icons';

  let username = '';
  let password = '';
  let submitting = false;
  let errorMessage: string | null = null;

  // Redirect already-authenticated users straight into the dashboard so the
  // login screen never feels like a dead end on a hard refresh.
  onMount(() => {
    if (isAuthenticated(getAuth())) {
      navigate('/admin');
    }
  });

  async function handleSubmit(event: Event): Promise<void> {
    event.preventDefault();
    if (submitting) return;
    errorMessage = null;

    const u = username.trim();
    const p = password;
    if (!u || !p) {
      errorMessage = 'Username dan password wajib diisi.';
      return;
    }

    submitting = true;
    try {
      const result = await login(u, p);
      setAuth(result.token, result.expires_at, result.user);
      navigate('/admin');
    } catch (err) {
      if (isApiError(err)) {
        errorMessage = err.detail ?? err.message;
      } else {
        errorMessage = 'Terjadi kesalahan tak terduga. Silakan coba lagi.';
      }
    } finally {
      submitting = false;
    }
  }
</script>

<svelte:head>
  <title>Masuk Admin - Desa Gerbosari</title>
</svelte:head>

<div class="min-h-screen bg-krem-50 flex items-center justify-center px-4 py-12">
  <div class="w-full max-w-md">
    <!-- Brand block: keeps the panel anchored to the village identity even on a
         standalone screen that's outside the public navbar. -->
    <div class="flex flex-col items-center gap-3 mb-8">
      <svg
        viewBox="0 0 32 32"
        class="w-10 h-10"
        fill="none"
        stroke="currentColor"
        stroke-width="1.75"
        stroke-linecap="round"
        stroke-linejoin="round"
        aria-hidden="true"
      >
        <circle cx="22" cy="9" r="2.25" class="text-terakota-500" fill="currentColor" stroke="none" />
        <path d="M3 25 L11 13 L17 20 L23 11 L29 25 Z" class="text-menoreh-700" />
        <path d="M3 25 L29 25" class="text-menoreh-700" />
      </svg>
      <div class="text-center">
        <h1 class="font-serif text-2xl font-semibold text-arang-900 leading-tight">
          Panel Admin Desa Gerbosari
        </h1>
        <p class="mt-1 text-sm text-arang-500">Masuk untuk mengelola konten desa.</p>
      </div>
    </div>

    <div class="bg-white border border-krem-200 rounded-lg p-6 sm:p-8">
      <form on:submit={handleSubmit} novalidate>
        <div class="flex flex-col gap-5">
          <div class="flex flex-col gap-1.5">
            <label for="username" class="text-sm font-medium text-arang-900">
              Username
            </label>
            <input
              id="username"
              name="username"
              type="text"
              autocomplete="username"
              required
              bind:value={username}
              disabled={submitting}
              class="h-10 px-3 rounded-md border border-krem-200 bg-white text-arang-900
                     placeholder:text-arang-300
                     focus:outline-none focus:border-menoreh-500 focus:ring-2 focus:ring-menoreh-500/20
                     disabled:bg-krem-50 disabled:cursor-not-allowed
                     transition-colors duration-200 ease-out"
              placeholder="admin"
            />
          </div>

          <div class="flex flex-col gap-1.5">
            <label for="password" class="text-sm font-medium text-arang-900">
              Password
            </label>
            <input
              id="password"
              name="password"
              type="password"
              autocomplete="current-password"
              required
              bind:value={password}
              disabled={submitting}
              class="h-10 px-3 rounded-md border border-krem-200 bg-white text-arang-900
                     placeholder:text-arang-300
                     focus:outline-none focus:border-menoreh-500 focus:ring-2 focus:ring-menoreh-500/20
                     disabled:bg-krem-50 disabled:cursor-not-allowed
                     transition-colors duration-200 ease-out"
              placeholder="••••••••"
            />
          </div>

          {#if errorMessage}
            <div
              role="alert"
              class="flex items-start gap-2 rounded-md border border-terakota-200 bg-terakota-50 px-3 py-2.5
                     text-sm text-terakota-800"
            >
              <AlertCircle class="w-4 h-4 mt-0.5 shrink-0" strokeWidth={2} aria-hidden="true" />
              <span>{errorMessage}</span>
            </div>
          {/if}

          <Button type="submit" variant="primary" size="md" loading={submitting} class="w-full">
            {#if !submitting}
              <Lock class="w-4 h-4" strokeWidth={2} aria-hidden="true" />
            {/if}
            <span>{submitting ? 'Memproses...' : 'Masuk'}</span>
          </Button>
        </div>
      </form>
    </div>

    <p class="mt-6 text-center text-xs text-arang-500">
      Akses terbatas untuk perangkat desa. Hubungi pengurus jika lupa kredensial.
    </p>
  </div>
</div>
