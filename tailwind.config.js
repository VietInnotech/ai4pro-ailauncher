/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{svelte,ts}"],
  theme: {
    extend: {
      colors: {
        shell: {
          950: "#111827",
          900: "#1f2937",
          800: "#273449"
        },
        accent: {
          500: "#4f46e5",
          600: "#4338ca"
        },
        ok: "#15803d",
        warn: "#b45309",
        danger: "#b91c1c"
      },
      boxShadow: {
        card: "0 24px 60px rgba(15, 23, 42, 0.18)"
      }
    }
  },
  plugins: []
};
