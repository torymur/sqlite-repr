/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  safelist: [
    "pattern-vertical-lines",
    "pattern-opacity-60",
    "pattern-white",
    "pattern-size-1",
    "pattern-bg-zinc-200",
    {
        pattern: /(bg|text|border)-(orange|green|slate)-(400|700|800|900)/,
    },
  ],
  daisyui: {
    themes: [
      {
        "custom": {
          ...require("daisyui/src/theming/themes")["corporate"],
        "primary": "D9DDE0", 
        }
      }
    ],
  },
  theme: {
    extend: {},
  },
  plugins: [
    require("@tailwindcss/typography"),
    // daisyui requirement should go after typography
    require("daisyui"),
    require('tailwindcss-bg-patterns'),
  ],
};
