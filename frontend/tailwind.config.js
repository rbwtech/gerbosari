/** @type {import('tailwindcss').Config} */
export default {
  content: ['./index.html', './src/**/*.{svelte,ts}'],
  theme: {
    extend: {
      // Earthy palette tuned to the Menoreh hills, terracotta crafts, warm cream
      // limestone, and historical umber. Ramps are perceptually-smoothed so each
      // step is usable as borders / surfaces / type without manual overrides.
      colors: {
        menoreh: {
          50: '#f3f7f3',
          100: '#dde8de',
          200: '#bfd1c1',
          300: '#94b497',
          400: '#669368',
          500: '#3e7242',
          600: '#2f5233',
          700: '#244226',
          800: '#1a3019',
          900: '#102010',
          950: '#070f07'
        },
        terakota: {
          50: '#fdf5f0',
          100: '#fae3d3',
          200: '#f3c4a4',
          300: '#e89e72',
          400: '#d97849',
          500: '#b85c38',
          600: '#9a4a2c',
          700: '#7c3a22',
          800: '#5c2b19',
          900: '#3d1d11',
          950: '#220f08'
        },
        krem: {
          50: '#faf7f2',
          100: '#f5ebe0',
          200: '#ebdfd0',
          300: '#dccab4',
          400: '#c4ab8e'
        },
        tanah: {
          50: '#f7f2ec',
          100: '#ebdfcd',
          200: '#d7c0a1',
          300: '#bf9e76',
          400: '#a07d55',
          500: '#856642',
          600: '#6c5135',
          700: '#553f29',
          800: '#3d2c1d',
          900: '#241a11'
        },
        arang: {
          50: '#f4f3f2',
          100: '#e3e1de',
          200: '#c2bdb6',
          300: '#9b9489',
          400: '#736c61',
          500: '#52493f',
          600: '#3d352d',
          700: '#2d2a26',
          800: '#211f1c',
          900: '#1a1816',
          950: '#0d0c0b'
        }
      },

      // Semantic token aliases. Routes/components should prefer these over raw
      // ramp values so the palette can be retuned in one place.
      borderColor: {
        soft: '#ebdfd0',   // krem-200
        strong: '#c4ab8e'  // krem-400
      },
      backgroundColor: {
        surface: '#faf7f2',       // krem-50
        'surface-raised': '#ffffff'
      },
      textColor: {
        primary: '#1a1816',   // arang-900
        secondary: '#2d2a26', // arang-700
        muted: '#52493f'      // arang-500 — single stable muted token
      },

      fontFamily: {
        sans: ['Inter', 'system-ui', 'sans-serif'],
        serif: ['Lora', 'Georgia', 'serif']
      },

      // Display-first scale. Body sticks to Tailwind defaults; display + h1/h2
      // map to serif hero usage on Beranda / Sejarah.
      fontSize: {
        display: ['clamp(2.75rem, 5vw + 1rem, 4.5rem)', { lineHeight: '1.05', letterSpacing: '-0.03em' }],
        h1: ['clamp(2rem, 2.5vw + 1rem, 2.75rem)', { lineHeight: '1.15', letterSpacing: '-0.02em' }],
        h2: ['1.875rem', { lineHeight: '1.2', letterSpacing: '-0.015em' }],
        h3: ['1.5rem', { lineHeight: '1.3', letterSpacing: '-0.01em' }],
        body: ['1rem', { lineHeight: '1.65' }],
        caption: ['0.75rem', { lineHeight: '1.4', letterSpacing: '0.08em' }]
      },

      letterSpacing: {
        'tighter-1': '-0.04em',
        smallcaps: '0.12em'
      },

      lineHeight: {
        relaxed: '1.65'
      },

      maxWidth: {
        prose: '68ch'
      },

      transitionTimingFunction: {
        'out-soft': 'cubic-bezier(0.22, 0.61, 0.36, 1)'
      },

      ringColor: {
        focus: '#3e7242' // menoreh-500
      },

      // Village-themed section "biomes". Kept as data-URI SVGs so no extra
      // assets ship — each pattern stays under ~1KB after minification and
      // renders standalone in any modern browser.
      backgroundImage: {
        'menoreh-mist':
          'linear-gradient(180deg, #dde8de 0%, #f0f5f1 35%, #faf7f2 100%)',
        // Solid base colors for tanah-paper / menoreh-deep are applied via the
        // .bg-tanah-paper / .bg-menoreh-deep utility classes (background-color),
        // because CSS `background-image` does not accept a solid color value.
        'tanah-paper':
          'radial-gradient(ellipse 60% 50% at 18% 28%, #fdf5f0 0%, transparent 62%), radial-gradient(ellipse 55% 45% at 82% 72%, #f5ebe0 0%, transparent 60%)',
        'menoreh-deep':
          'radial-gradient(ellipse 70% 60% at 30% 20%, #1a3019 0%, transparent 60%), radial-gradient(ellipse 60% 55% at 75% 80%, #244226 0%, transparent 65%)',
        'paper-noise':
          "url(\"data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='180' height='180' viewBox='0 0 180 180'><filter id='n'><feTurbulence type='fractalNoise' baseFrequency='0.9' numOctaves='2' stitchTiles='stitch'/><feColorMatrix values='0 0 0 0 0.24  0 0 0 0 0.18  0 0 0 0 0.12  0 0 0 0.035 0'/></filter><rect width='100%25' height='100%25' filter='url(%23n)'/></svg>\")",
        'batik-dots':
          "url(\"data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='56' height='56' viewBox='0 0 56 56'><g fill='none' stroke='%23b85c38' stroke-width='0.6' stroke-opacity='0.18'><circle cx='10' cy='10' r='1.6'/><circle cx='46' cy='10' r='1.6'/><circle cx='28' cy='28' r='2.2'/><circle cx='10' cy='46' r='1.6'/><circle cx='46' cy='46' r='1.6'/><path d='M26 6 L30 10 L26 14 L22 10 Z'/><path d='M26 42 L30 46 L26 50 L22 46 Z'/><path d='M6 26 L10 30 L6 34 L2 30 Z' transform='translate(2 -2)'/></g><g fill='%233e7242' fill-opacity='0.10'><circle cx='28' cy='28' r='0.9'/></g></svg>\")"
      }
    }
  },
  plugins: []
};
