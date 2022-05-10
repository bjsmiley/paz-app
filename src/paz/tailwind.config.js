const production = !process.env.ROLLUP_WATCH;

module.exports = {
  content: [
    "./src/**/*.svelte"
  ],
  // purge: {
  //   content: [
  //     "./src/**/*.svelte",
  //   ],
  //   enabled: production
  // },
  // darkMode: false, // or 'media' or 'class'
  theme: {
    extend: {},
  },
  plugins: [],
  future: {
    purgeLayersByDefault: true,
    removeDeprecatedGapUtilities: true,
  },
}
