/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
  ],
  theme: {
    extend: {},
  },
  plugins: [
    require('daisyui'),
  ],
  daisyui: {
    themes: ['synthwave'],
  }
}

