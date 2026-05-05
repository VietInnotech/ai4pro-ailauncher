/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{svelte,ts}"],
  theme: {
    extend: {
      colors: {
        shell: {
          950: "#1b2430",
          900: "#263241",
          800: "#38485b"
        },
        accent: {
          500: "#355c7d",
          600: "#29455f"
        },
        ok: "#2f6b57",
        warn: "#8a6431",
        danger: "#9a3d3d"
      },
      boxShadow: {
        card: "0 16px 40px rgba(16, 24, 40, 0.08)",
        panel: "0 16px 40px rgba(16, 24, 40, 0.08)"
      }
    }
  },
  plugins: []
};
