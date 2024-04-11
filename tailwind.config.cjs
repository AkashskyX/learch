/** @type {import('tailwindcss').Config}*/
const config = {

  darkMode: false,


  variants: {
    extend: {
      backdropFilter: ['responsive'], // enable backdrop-filter utilities
    },
  },



  content: ["./src/**/*.{html,js,svelte,ts}"],

  theme: {
    extend: {},
  },

  plugins: [
    require('@tailwindcss/forms'),
    require('tailwindcss-filters'), // make sure to install this plugin
  ],
};

module.exports = config;
