/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {
			colors: {
				'anttp-blue': '#3b82f6',
				'anttp-green': '#10b981',
				'anttp-purple': '#8b5cf6',
			}
		}
	},
	plugins: []
};
