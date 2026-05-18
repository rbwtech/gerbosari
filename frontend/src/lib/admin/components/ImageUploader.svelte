<script lang="ts">
  /**
   * Image upload widget used by admin Edit pages. Wraps a hidden `<input
   * type="file">` so the visible affordance is a styled drop zone + button.
   *
   * Behaviour:
   *  - Picks a single image (jpg/png/webp). Drag-and-drop accepted on desktop.
   *  - Uploads immediately to POST /api/admin/upload, then writes the returned
   *    URL into the two-way bound `value`.
   *  - Shows a live preview either from a freshly picked File (object URL) or
   *    from the currently bound `value` (an already-saved /images/... path).
   *  - Surfaces a manual URL fallback for the rare case the operator wants to
   *    paste a remote image link instead of uploading.
   *
   * Error and required state are mirrored from the parent the same way
   * `FormField` does, so the page-level error banner can stay unchanged.
   */
  import { onDestroy } from 'svelte';
  import { Upload, ImageIcon, Trash2, Loader2 } from '../../components/icons';
  import { uploadImage, type UploadEntity } from '../../api/admin/upload';
  import { isApiError } from '../../api/client';

  export let value: string = '';
  export let entity: UploadEntity;
  export let label: string;
  export let name: string;
  export let required = false;
  export let error: string | undefined = undefined;
  export let hint: string | undefined = undefined;

  /** Maximum bytes the user may select; mirrors the backend cap (5 MiB). */
  const MAX_BYTES = 5 * 1024 * 1024;
  /** MIME prefixes accepted by the backend. */
  const ACCEPTED_TYPES = ['image/jpeg', 'image/png', 'image/webp'];

  let inputEl: HTMLInputElement;
  let isUploading = false;
  let dragOver = false;
  let uploadError: string | null = null;
  /** Object URL for the in-flight preview. Revoked on replacement / destroy. */
  let pendingPreviewUrl: string | null = null;
  let showManual = false;

  const uid = `iu-${Math.random().toString(36).slice(2, 9)}`;
  $: descId = error ? `${uid}-err` : uploadError ? `${uid}-up-err` : hint ? `${uid}-hint` : undefined;

  // The image to show in the preview pane: the freshly-picked file beats the
  // already-saved value, so the operator sees what they're about to commit.
  $: previewSrc = pendingPreviewUrl ?? (value.trim() ? value : '');

  function revokePending() {
    if (pendingPreviewUrl) {
      URL.revokeObjectURL(pendingPreviewUrl);
      pendingPreviewUrl = null;
    }
  }

  onDestroy(() => revokePending());

  async function handleFile(file: File) {
    if (!ACCEPTED_TYPES.includes(file.type)) {
      uploadError = 'Tipe file tidak didukung. Gunakan JPG, PNG, atau WebP.';
      return;
    }
    if (file.size > MAX_BYTES) {
      uploadError = `Ukuran file melebihi batas ${(MAX_BYTES / (1024 * 1024)).toFixed(0)} MiB.`;
      return;
    }

    uploadError = null;
    revokePending();
    pendingPreviewUrl = URL.createObjectURL(file);
    isUploading = true;
    try {
      const res = await uploadImage(file, entity);
      value = res.url;
    } catch (err) {
      uploadError = isApiError(err) ? err.detail ?? err.message : 'Gagal mengunggah gambar.';
      // Revert the preview - if the upload failed, the saved value is the
      // source of truth, not the file the user just picked.
      revokePending();
    } finally {
      isUploading = false;
    }
  }

  function onPick(e: Event) {
    const target = e.target as HTMLInputElement;
    const file = target.files?.[0];
    if (file) void handleFile(file);
    // Reset input so picking the same filename twice still fires `change`.
    target.value = '';
  }

  function onDrop(e: DragEvent) {
    e.preventDefault();
    dragOver = false;
    const file = e.dataTransfer?.files?.[0];
    if (file) void handleFile(file);
  }

  function onDragOver(e: DragEvent) {
    e.preventDefault();
    dragOver = true;
  }

  function onDragLeave() {
    dragOver = false;
  }

  function clearImage() {
    revokePending();
    value = '';
    uploadError = null;
  }
</script>

<div class="flex flex-col">
  <span class="block text-sm font-medium text-arang-800 mb-1.5">
    {label}
    {#if required}
      <span class="text-terakota-500" aria-hidden="true">*</span>
    {/if}
  </span>

  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div
    class="relative rounded-lg border-2 border-dashed transition-colors duration-150
           {dragOver ? 'border-menoreh-500 bg-menoreh-50' : 'border-krem-300 bg-krem-50'}
           {error || uploadError ? 'border-terakota-500' : ''}"
    on:dragover={onDragOver}
    on:dragleave={onDragLeave}
    on:drop={onDrop}
  >
    {#if previewSrc}
      <div class="flex flex-col sm:flex-row gap-4 p-4">
        <div class="w-full sm:w-48 shrink-0">
          <div class="aspect-video w-full overflow-hidden rounded border border-krem-200 bg-white">
            <img
              src={previewSrc}
              alt="Pratinjau gambar"
              class="h-full w-full object-cover"
              loading="lazy"
            />
          </div>
        </div>
        <div class="flex flex-col gap-2 min-w-0 flex-1">
          <p class="text-xs uppercase tracking-widest text-arang-500">
            {isUploading ? 'Mengunggah...' : pendingPreviewUrl ? 'Baru dipilih' : 'Tersimpan'}
          </p>
          {#if value}
            <p class="text-xs font-mono text-arang-700 break-all">{value}</p>
          {/if}
          <div class="mt-2 flex flex-wrap gap-2">
            <button
              type="button"
              on:click={() => inputEl?.click()}
              disabled={isUploading}
              class="inline-flex items-center gap-1.5 min-h-9 px-3 rounded border border-krem-300 bg-white text-xs font-medium text-arang-800 hover:bg-krem-100 disabled:opacity-60 disabled:cursor-not-allowed transition-colors duration-150"
            >
              <Upload class="h-3.5 w-3.5" strokeWidth={2} aria-hidden="true" />
              Ganti gambar
            </button>
            <button
              type="button"
              on:click={clearImage}
              disabled={isUploading}
              class="inline-flex items-center gap-1.5 min-h-9 px-3 rounded border border-krem-300 bg-white text-xs font-medium text-terakota-700 hover:bg-terakota-50 disabled:opacity-60 disabled:cursor-not-allowed transition-colors duration-150"
            >
              <Trash2 class="h-3.5 w-3.5" strokeWidth={2} aria-hidden="true" />
              Hapus
            </button>
          </div>
        </div>
      </div>
    {:else}
      <button
        type="button"
        on:click={() => inputEl?.click()}
        disabled={isUploading}
        class="w-full flex flex-col items-center justify-center gap-2 px-4 py-8 text-center text-sm text-arang-700 disabled:opacity-60 disabled:cursor-not-allowed"
      >
        {#if isUploading}
          <Loader2 class="h-7 w-7 text-menoreh-600 animate-spin" strokeWidth={1.75} aria-hidden="true" />
          <span class="font-medium">Mengunggah gambar...</span>
        {:else}
          <ImageIcon class="h-7 w-7 text-arang-400" strokeWidth={1.5} aria-hidden="true" />
          <span class="font-medium text-arang-800">Klik untuk pilih, atau jatuhkan gambar di sini</span>
          <span class="text-xs text-arang-500">JPG, PNG, atau WebP. Maksimal 5 MiB.</span>
        {/if}
      </button>
    {/if}

    <input
      bind:this={inputEl}
      id={uid}
      type="file"
      accept="image/jpeg,image/png,image/webp"
      class="sr-only"
      on:change={onPick}
      aria-describedby={descId}
      {name}
    />
  </div>

  {#if uploadError}
    <p id="{uid}-up-err" class="mt-1.5 text-xs text-terakota-700" role="alert">
      {uploadError}
    </p>
  {:else if error}
    <p id="{uid}-err" class="mt-1.5 text-xs text-terakota-700" role="alert">{error}</p>
  {:else if hint}
    <p id="{uid}-hint" class="mt-1.5 text-xs text-arang-500">{hint}</p>
  {/if}

  <!-- Manual URL escape hatch. Hidden by default so the upload flow reads as
       the primary path, but kept available for ops who paste a CDN link. -->
  <div class="mt-2">
    <button
      type="button"
      on:click={() => (showManual = !showManual)}
      class="text-xs text-menoreh-700 hover:text-menoreh-800 underline-offset-2 hover:underline"
    >
      {showManual ? 'Sembunyikan input URL manual' : 'Atau masukkan URL secara manual'}
    </button>
    {#if showManual}
      <input
        type="text"
        bind:value
        placeholder="/images/gallery/contoh.jpg atau https://..."
        class="mt-2 w-full rounded-md border border-krem-300 bg-white px-3 py-2 min-h-11 text-sm font-mono text-arang-900 placeholder:text-arang-400 focus:outline-none focus:border-menoreh-600 focus:ring-2 focus:ring-menoreh-500"
      />
    {/if}
  </div>
</div>
