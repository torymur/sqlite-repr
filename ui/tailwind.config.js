/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  safelist: [
    "pattern-vertical-lines",
    "pattern-opacity-60",
    "pattern-white",
    "pattern-size-1",
    "pattern-bg-slate-200",
    "locked",
    {
        pattern: /(bg|text|border)-(orange|green|slate)-(600|700|800)/,
    },
    {
        pattern: /bg-slate-([1-4][0-9]0)/,
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
    extend: {
      colors: {
        slate: {
          330: "#b8c2d0",
          360: "#aab4c4",
          390: "#9ea8bb",
        }
      },
    },
  },
  plugins: [
    require("@tailwindcss/typography"),
    // daisyui requirement should go after typography
    require("daisyui"),
    require('tailwindcss-bg-patterns'),
  ],
};
