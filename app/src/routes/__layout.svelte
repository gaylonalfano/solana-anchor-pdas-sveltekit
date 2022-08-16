<script lang="ts">
	import { WalletProvider, WalletMultiButton } from '@svelte-on-solana/wallet-adapter-ui';
	import { AnchorConnectionProvider, workSpace } from '@svelte-on-solana/wallet-adapter-anchor';
	import { clusterApiUrl } from '@solana/web3.js';
	import { PhantomWalletAdapter, SolflareWalletAdapter } from '@solana/wallet-adapter-wallets';
	import idl from '../../../target/idl/solana_anchor_pdas_sveltekit.json';

	const localStorageKey = 'walletAdapter';
	// const network = clusterApiUrl('devnet'); // Can't swap for 'localnet'
	const network = 'http://localhost:8899';

	let wallets = [new PhantomWalletAdapter(), new SolflareWalletAdapter()];
</script>

<WalletProvider {localStorageKey} {wallets} autoConnect />
<AnchorConnectionProvider {network} {idl} />
<div>
	<slot />
</div>
