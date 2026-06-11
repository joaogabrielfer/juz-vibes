// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import { docsSidebar, siteDescription, siteTitle } from './src/generated/site-data.js';

export default defineConfig({
	site: 'https://joaogabrielfer.github.io',
	base: '/juz-vibes',
	integrations: [
		starlight({
			title: siteTitle,
			description: siteDescription,
			customCss: ['./src/styles/custom.css'],
			editLink: {
				baseUrl: 'https://github.com/joaogabrielfer/juz-vibes/edit/main/site/',
			},
			social: [{ icon: 'github', label: 'GitHub', href: 'https://github.com/joaogabrielfer/juz-vibes' }],
			sidebar: docsSidebar,
		}),
	],
});
