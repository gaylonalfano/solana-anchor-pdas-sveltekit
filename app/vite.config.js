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
		esbuildOptions: {
			target: 'esnext'
		}
	},
	build: {
		target: 'esnext'
	}
};

export default config;
