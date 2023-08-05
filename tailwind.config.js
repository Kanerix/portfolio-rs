/** @type {import('tailwindcss').Config} */

module.exports = {
	darkMode: 'class',
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