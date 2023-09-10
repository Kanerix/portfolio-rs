/** @type {import('tailwindcss').Config} */

export default {
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