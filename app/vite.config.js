import { sveltekit } from '@sveltejs/kit/vite';

/** @type {import('vite').UserConfig} */
const config = {
	plugins: [sveltekit()],
	define: {
		'process.env.BROWSER': true
	},
	optimizeDeps: {
		include: ['@project-serum/anchor', '@solana/web3.js', 'buffer'],
		// NOTE esbuildOptions & build config referenced from Github issue:
		// https://github.com/svelte-on-solana/wallet-adapter/issues/42
		// NOTE This FIRST entailed sveltkit upgrade/migration to new router:
		// https://github.com/sveltejs/kit/discussions/5774
		esbuildOptions: {
			target: 'esnext'
		}
	},
	build: {
		target: 'esnext'
	}
};

export default config;
