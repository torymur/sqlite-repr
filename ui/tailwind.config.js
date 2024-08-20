/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  safelist: [
    {
        pattern: /(bg|text|border)-(orange|green|fuchsia)-(700|800)/,
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
  ],
};
