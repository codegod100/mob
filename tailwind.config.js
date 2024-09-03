const path = require('path')

/** @type {import('tailwindcss').Config} */
export default {
  content: [path.join(__dirname, './src/**/*.{html,js,svelte,ts}')],
  theme: {
    extend: {},
  },
  plugins: [],
}

//'./src/**/*.{html,js,svelte,ts}'