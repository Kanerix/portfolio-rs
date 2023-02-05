/** @type {import('tailwindcss').Config} */

module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"]
  },
  theme: {
	fontFamily: {
		'sans': ['Poppins'],
	}
  },
  plugins: [],
}