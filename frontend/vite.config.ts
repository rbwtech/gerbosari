import { defineConfig, type Plugin } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import { fileURLToPath } from 'node:url';
import { createReadStream, statSync } from 'node:fs';
import { extname, join } from 'node:path';
import type { IncomingMessage, ServerResponse } from 'node:http';

const MIME: Record<string, string> = {
  '.jpg': 'image/jpeg',
  '.jpeg': 'image/jpeg',
  '.png': 'image/png',
  '.webp': 'image/webp',
  '.gif': 'image/gif',
  '.svg': 'image/svg+xml'
};

/**
 * Serves repo-root `content/images/` at `/images/` during dev & preview so the
 * URLs stored in the database (e.g. /images/gallery/foo.jpg) work uniformly in
 * dev and prod. In production, nginx handles the same alias.
 */
function serveContentImages(): Plugin {
  const dir = fileURLToPath(new URL('../content/images', import.meta.url));

  const handle = (
    req: IncomingMessage,
    res: ServerResponse,
    next: (err?: unknown) => void
  ) => {
    if (!req.url || !req.url.startsWith('/images/')) return next();
    const rel = decodeURIComponent(req.url.replace(/^\/images\//, '').split('?')[0]);
    if (rel.includes('..')) {
      res.statusCode = 403;
      res.end();
      return;
    }
    const filePath = join(dir, rel);
    try {
      statSync(filePath);
    } catch {
      res.statusCode = 404;
      res.end();
      return;
    }
    res.setHeader('Content-Type', MIME[extname(filePath).toLowerCase()] ?? 'application/octet-stream');
    res.setHeader('Cache-Control', 'public, max-age=3600');
    createReadStream(filePath).pipe(res);
  };

  return {
    name: 'serve-content-images',
    configureServer(server) {
      server.middlewares.use(handle);
    },
    configurePreviewServer(server) {
      server.middlewares.use(handle);
    }
  };
}

export default defineConfig({
  plugins: [svelte(), serveContentImages()],
  resolve: {
    alias: {
      $lib: fileURLToPath(new URL('./src/lib', import.meta.url))
    }
  },
  server: {
    port: 5173,
    host: true,
    fs: {
      // Allow Vite to read JSON content from the sibling `content/` folder.
      allow: ['..']
    }
  },
  build: {
    target: 'es2022',
    sourcemap: true
  }
});
