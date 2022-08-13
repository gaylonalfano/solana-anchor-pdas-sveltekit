<script lang="ts">
	import { walletStore } from '@svelte-on-solana/wallet-adapter-core';
	import { WalletProvider, WalletMultiButton } from '@svelte-on-solana/wallet-adapter-ui';
	import { AnchorConnectionProvider, workSpace } from '@svelte-on-solana/wallet-adapter-anchor';
	import { clusterApiUrl } from '@solana/web3.js';
	import { PhantomWalletAdapter, SolflareWalletAdapter } from '@solana/wallet-adapter-wallets';
	import idl from '../../../target/idl/solana_anchor_pdas_sveltekit.json';

	const localStorageKey = 'walletAdapter';
	const network = clusterApiUrl('devnet');

	let wallets = [new PhantomWalletAdapter(), new SolflareWalletAdapter()];
</script>

<WalletProvider {localStorageKey} {wallets} autoConnect />
<AnchorConnectionProvider {network} {idl} />
<div>
	<slot />
</div>
<WalletMultiButton />
