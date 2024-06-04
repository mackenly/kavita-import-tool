import { describe, it, expect } from 'vitest';
import config from './theme.config';
import React from 'react';
import ReactDOMServer from 'react-dom/server';


describe('theme.config.tsx', () => {
	it('should export a config object with the expected properties', () => {
		expect(config).toHaveProperty('logo');
		expect(config).toHaveProperty('project');
		expect(config).toHaveProperty('docsRepositoryBase');
		expect(config).toHaveProperty('footer');
		expect(config).toHaveProperty('head');

		// Kavita Import Tool should be in the logo
		const logoString = ReactDOMServer.renderToStaticMarkup(config.logo);
        expect(logoString).toMatch("<span>Kavita Import Tool</span>");
		expect(config.project.link).toBe('https://github.com/mackenly/kavita-import-tool');
		expect(config.docsRepositoryBase).toBe('https://github.com/mackenly/kavita-import-tool-docs');
        const footerString = ReactDOMServer.renderToStaticMarkup(config.footer.text);
		expect(footerString).toMatch(/Kavita Import Tool is a project by/);
		expect(footerString).toMatch(/Mackenly Jones/);
		expect(footerString).toMatch(/Sponsor or donate to this project via/);
		expect(footerString).toMatch(/GitHub Sponsors @mackenly/);
        const headString = ReactDOMServer.renderToStaticMarkup(config.head);
		expect(headString).toMatch(
			'<meta name="theme-color" content="#000000"/><meta name="description" content="Kavita Import Tool is a tool to help you create folder structures to import into Kavita"/><link rel="icon" href="https://fav.farm/📖"/>'
		);
	});
});
