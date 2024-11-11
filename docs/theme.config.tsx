import React from 'react'
import { DocsThemeConfig } from 'nextra-theme-docs'

const config: DocsThemeConfig = {
	logo: <span>Kavita Import Tool</span>,
	project: {
		link: 'https://github.com/mackenly/kavita-import-tool',
	},
	docsRepositoryBase: 'https://github.com/mackenly/kavita-import-tool-docs',
	footer: {
		content: (
			<p>
				Kavita Import Tool is a project by{' '}
				<a href="https://github.com/mackenly" target="_blank">
					Mackenly Jones
				</a>
				<br />
				Sponsor or donate to this project via{' '}
				<a href="https://github.com/sponsors/mackenly/" target="_blank">
					GitHub Sponsors @mackenly
				</a>
			</p>
		),
	},
	// favicon
	head: (
		<>
		<meta name="theme-color" content="#000000" />
		<meta
			name="description"
			content="Kavita Import Tool is a tool to help you create folder structures to import into Kavita"
		/>
		<link rel="icon" href="https://fav.farm/📖" />
		</>
	),
};

export default config
