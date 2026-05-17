<script lang="ts">
  import SectionShell from '../lib/components/ui/SectionShell.svelte';
  import Button from '../lib/components/ui/Button.svelte';
  import { ChevronLeft, Compass } from '../lib/components/icons';
</script>

<!--
  Mist biome on 404 keeps the tone calm; the Compass icon nods to the
  geography motif without trivialising the error state.
-->
<SectionShell variant="mist" padding="lg" contained={true}>
  <div class="text-center max-w-md mx-auto">
    <Compass
      class="mx-auto h-5 w-5 sm:h-6 sm:w-6 text-menoreh-700/80"
      strokeWidth={1.5}
      aria-hidden="true"
    />
    <p class="mt-6 font-serif text-6xl sm:text-8xl md:text-9xl font-semibold text-arang-900 tnum leading-none">
      404
    </p>
    <h1 class="mt-4 font-serif text-2xl md:text-3xl font-semibold text-arang-900 break-words">
      Halaman tidak ditemukan
    </h1>
    <p class="mt-3 text-arang-700 break-words">
      Tautan yang Anda buka mungkin sudah dipindahkan atau tidak pernah ada.
    </p>
    <div class="mt-8 flex justify-center">
      <Button href="#/" variant="primary" class="min-h-12 w-full sm:w-auto">
        <ChevronLeft class="h-4 w-4" strokeWidth={2} aria-hidden="true" />
        Kembali ke Beranda
      </Button>
    </div>
  </div>
</SectionShell>
