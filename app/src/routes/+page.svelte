<script lang="ts">
	import { WalletMultiButton } from '@svelte-on-solana/wallet-adapter-ui';
	import { walletStore } from '@svelte-on-solana/wallet-adapter-core';
	import { workSpace as workspaceStore } from '@svelte-on-solana/wallet-adapter-anchor';
	import { fly } from 'svelte/transition';
	import BN from 'bn.js';
	import * as anchor from '@project-serum/anchor';

	function shortKey(key: anchor.web3.PublicKey) {
		return key.toString().substring(0, 8);
	}

	// Configure the client to use the local cluster.
	// NOTE Grab provider as const so we have access to its methods
	// WITHOUT Store
	/* const provider = anchor.AnchorProvider.env(); */
	/* anchor.setProvider(provider); */
	// WITH Store
	/* const program = $workSpace.program; */
	/* const provider = $workspaceStore.provider; */
	/* anchor.setProvider(provider); */

	// Q: How do I get my Program<T> i.e., as Program<SolanaAnchorPdasSveltekit>??
	// A: NOT SURE IT'S NECESSARY as there's a lot of sync to the IDL in the AnchorConnectionProvider Svelte Component
	// https://github.com/svelte-on-solana/wallet-adapter/blob/master/packages/anchor/src/lib/AnchorConnectionProvider.svelte
	/* const program = workspaceStore.program.idl. */
	/* 	.SolanaAnchorPdasSveltekit as Program<SolanaAnchorPdasSveltekit>; */
	const DEVNET_LEDGERS = [
		{
			wallet: '2BScwdytqa6BnjW6SUqKt8uaKYn6M4gLbWBdn3JuJWjE',
			color: 'red',
			pda: '5ZKfW2vMyKShV1MoiekEASwVQBGkcZbYThFCUaAZxLzM'
		},
		{
			wallet: '2BScwdytqa6BnjW6SUqKt8uaKYn6M4gLbWBdn3JuJWjE',
			color: 'white',
			pda: 'imaENQ8o46KvzkVpx6DT1SXpxbgd9wXMPV1kMgShtfJ'
		},
		{
			wallet: '2BScwdytqa6BnjW6SUqKt8uaKYn6M4gLbWBdn3JuJWjE',
			color: 'yellow',
			pda: 'AYWUYuchSSskAKYNAVzV4zSkjh45RNg8vNwGe6PGMdFD'
		},
		{
			wallet: 'SSyUdM98Z6Fa5faGyo5qrBmxFuB6koZt7cUt4i9JyXt',
			color: 'red',
			pda: 'Ezu4mwWzm4KSJ9xaGAbvofAVxAsTKLNjJorhR7P2oKkk'
		},
		{
			wallet: 'SSyUdM98Z6Fa5faGyo5qrBmxFuB6koZt7cUt4i9JyXt',
			color: 'blue',
			pda: '2eQc39fkTyRHproyQR9X6cq62cNhnhehqzgqprxBryMn'
		}
	];

	const LOCALNET_LEDGERS = [
		{
			wallet: '2BScwdytqa6BnjW6SUqKt8uaKYn6M4gLbWBdn3JuJWjE',
			color: 'yellow',
			pda: ''
		},
		{
			wallet: '2BScwdytqa6BnjW6SUqKt8uaKYn6M4gLbWBdn3JuJWjE',
			color: 'green',
			pda: ''
		}
	];

	$: {
		console.log('workspaceStore', $workspaceStore);
		console.log('walletStore', $walletStore);
		console.log('wallet', wallet);
		console.log('color', color);
		console.log('fetchedLedgerAccount', fetchedLedgerAccount);
	}

	// NOTE Ran my DEVNET tests to generate these for now...
	const testWallet1 = DEVNET_LEDGERS[3].wallet;
	const testPda1 = DEVNET_LEDGERS[3].pda;
	const testPda1Color = DEVNET_LEDGERS[3].color;
	const testPda2 = DEVNET_LEDGERS[4].pda;
	const testPda2Color = DEVNET_LEDGERS[4].color;
	let wallet = testWallet1;
	let pda = testPda1;
	let color = testPda1Color;
	let newBalance = '0';
	let fetchedLedgerAccount;

	async function generateKeypair() {
		// Ensure that new wallet keypair has enough SOL
		let keypair = anchor.web3.Keypair.generate();
		// WITHOUT Store
		// await provider.connection.requestAirdrop(keypair.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL);
		// WITH Store
		await $workspaceStore.provider?.connection.requestAirdrop(
			keypair.publicKey,
			2 * anchor.web3.LAMPORTS_PER_SOL
		);
		// Sleep for devnet
		await new Promise((resolve) => setTimeout(resolve, 3 * 1000));
		return keypair;
	}

	async function derivePda(color: string, pubkey: anchor.web3.PublicKey) {
		// NOTE This is key! We can derive PDA WITHOUT hitting our program!
		// Then we can use this PDA address in our functions as a check to see
		// whether there is a ledger account at this PDA address.
		// Then, MOST IMPORTANTLY, we can fetch the account's data from the CLIENT
		// and use its data.
		// NOTE pubkey is actually provider.wallet.publicKey
		let [pda, _] = await anchor.web3.PublicKey.findProgramAddress(
			[pubkey.toBuffer(), Buffer.from('_'), Buffer.from(color)],
			// WITH Store
			$workspaceStore.program?.programId as anchor.web3.PublicKey
			// WITHOUT Store
			// program.programId // The program that we want to OWN the PDA
		);

		return pda;
	}

	async function handleGetLedgerAccount(color: string, wallet: string) {
		// NOTE For testing purposes only. Taking input text and converting to correct types.
		// NOTE Must convert string to type Publickey
		let pda = await derivePda(color, new anchor.web3.PublicKey(wallet));
		let data = await $workspaceStore.program?.account.ledger.fetch(pda);
		fetchedLedgerAccount = data;
		return data;
	}

	async function createLedgerAccount(
		color: string,
		pda: anchor.web3.PublicKey,
		wallet: anchor.web3.Keypair
	) {
		// Calls the program's on-chain create_ledger instruction function
		// to create a ledger account LOCATED at our generated PDA address!
		// NOTE This requires same args i.e., Context, color, system
		// NOTE We're technically creating a ledger account LOCATED at
		// this PDA address!
		await $workspaceStore.program?.methods
			.createLedger(color)
			.accounts({
				ledgerAccount: pda,
				wallet: $workspaceStore.provider.wallet.publicKey // OR: $walletStore.publicKey
				// NOTE Anchor automatically adds System Program (and other programs if required)
			})
			// NOTE FRONTEND: Don't need to pass signers() I guess....
			// .signers([wallet]) // Q: Need this? A: NO!
			.rpc();
	}

	async function handleCreateLedgerAccount() {
		let pda = await derivePda(color, new anchor.web3.PublicKey($walletStore.publicKey));

		try {
			// Q: How to pass a Keypair from walletStore? I have the signers([wallet]) for the ix
			// REF: https://solana.stackexchange.com/questions/1984/anchor-signing-and-paying-for-transactions-to-interact-with-program
			// REF: https://stackoverflow.com/questions/72549145/how-to-sign-and-call-anchor-solana-smart-contract-from-web-app
			// REF: https://www.youtube.com/watch?v=vt8GUw_PDqM
			// UPDATE: Looks like I can pass the $walletStore OR $workspaceStore.provider.wallet
			// UPDATE: Looks like you DON'T pass signers([wallet]) call from frontend,
			// since it fails if I pass it inside the program.methods.createLedger() call
			await createLedgerAccount(color, pda, $walletStore); // WORKS
			// await createLedgerAccount(color, pda, $workspaceStore.provider.wallet); // WORKS

			const data = await $workspaceStore.program?.account.ledger.fetch(pda);
			fetchedLedgerAccount = data;
		} catch (e) {
			console.error('handleCreateLedgerAccount::Error: ', e);
		}
	}

	async function modifyLedgerAccount(
		color: string,
		newBalance: number,
		wallet: anchor.web3.Keypair // Q: How to pass this with $walletStore????
	) {
		console.log('------------------------------------');
		// 1. Retrieve the PDA using helper
		// NOTE Don't pass pda address. Just pass color
		let data; // Is type Ledger

		/* let pda = await derivePda(color, wallet.publicKey); */
		let pda = await derivePda(color, new anchor.web3.PublicKey($walletStore.publicKey));

		// 2. Try to retreive PDA account data if it exists
		console.log(`Checking if account ${shortKey(pda)} exists for color: ${color}...`);
		try {
			// NOTE We're technically seeing if our PDA address has a
			// ledger account at its location (address)
			data = await $workspaceStore.program?.account.ledger.fetch(pda);
			fetchedLedgerAccount = data;
			console.log(`Account already exists!`);
		} catch (e) {
			// console.log(e);
			console.log(`Account ${shortKey(pda)} does NOT exist!`);
			console.log('Creating account...');
			// 1. Create account using helper that calls program instruction
			/* await createLedgerAccount(color, pda, wallet); */
			await createLedgerAccount(color, pda, $walletStore);
			// 2. Retrieve account data
			data = await $workspaceStore.program?.account.ledger.fetch(pda);
			fetchedLedgerAccount = data;
		}

		console.log(`SUCCESS! Wallet: ${shortKey(wallet.publicKey)} -- PDA: ${shortKey(pda)} `);
		console.log('Our PDA has a ledger account with data:\n');
		console.log(`    Color: ${data?.color}   Balance: ${data?.balance}`);
		console.log(`Modifying balance of ${data?.color} from ${data?.balance} to ${newBalance}`);

		// 3. Make our modifications to the account using on-chain program function
		// NOTE This is another program function instruction
		// Q: Going to TEST whether other wallets can modify if they have the PDA...
		// A: NOPE! Error: Signature verification failed
		// let otherWalletInfo = DEVNET_LEDGERS[4];
		// let pdaFromOtherWallet = await derivePda(color, new anchor.web3.PublicKey(otherWalletInfo.wallet));

		await $workspaceStore.program?.methods
			.modifyLedger(newBalance)
			.accounts({
				ledgerAccount: pda,
				wallet: $walletStore.publicKey // OR: $workspaceStore.provider.publicKey
				// ledgerAccount: pdaFromOtherWallet, // CANNOT modify using a different wallet!
				// wallet: new anchor.web3.PublicKey(otherWalletInfo.wallet), // CANNOT modify using a different wallet!
			})
			// .signers([wallet]) // NOT needed on FRONTEND I THINK...
			.rpc();

		// 4. Retrieve the updated data one last time
		data = await $workspaceStore.program?.account.ledger.fetch(pda);
		// data = await $workspaceStore.program?.account.ledger.fetch(pdaFromOtherWallet); // CANNOT modify using a different wallet!
		fetchedLedgerAccount = data;
		// console.log(`Updated data for account located at:`);
		console.log(`UPDATED! Wallet: ${shortKey(wallet.publicKey)} -- PDA: ${shortKey(pda)} `);
		console.log(`    Color: ${data?.color}   Balance: ${data?.balance}`);
		console.log('Successfully modified ledger account!');
	}

	/* it("An example of PDAs in action", async () => { */
	/* // Q: Is this new keypair essentially representing another */
	/* // wallet???? Which is then used to create/modify ledger accounts? */
	/* // A: YES! We need a Keypair (Wallet) to sign these transactions, */
	/* // so this is a quick/easy way to simulate multiple users. */
	/* const testKeypair1 = await generateKeypair(); */
	/* await modifyLedgerAccount("red", 2, testKeypair1); */
	/* await modifyLedgerAccount("red", 4, testKeypair1); */
	/* await modifyLedgerAccount("blue", 3, testKeypair1); */

	/* const testKeypair2 = await generateKeypair(); */
	/* await modifyLedgerAccount("red", 3, testKeypair2); */
	/* await modifyLedgerAccount("green", 5, testKeypair2); */
	/* }); */

	async function handleModifyLedgerAccount() {
		try {
			// Q: How should I pass in type number? Use new BN() or new Number()?
			// A: Works using BN() and/or Number()!
			await modifyLedgerAccount(color, new anchor.BN(newBalance), $workspaceStore.provider.wallet);
		} catch (e) {
			console.error(e);
		}
	}
</script>

<div class="wrapper-app">
	<div class="title">
		<h1>Solana Svelte PDAs Demo</h1>
	</div>

	<div class="address">
		<WalletMultiButton />
	</div>

	{#if $walletStore?.connected}
		<div class="wrapper-content">
			<div class="connection">
				<h2>workspace.provider.connection established!</h2>
				<h3>Connected wallet address:</h3>
				<p>Wallet: {$walletStore.wallet?.publicKey}</p>
				<p>Program: {$workspaceStore.program?.programId}</p>
				<p>Color: {color}</p>
			</div>
			<div class="create-account">
				<input type="text" name="color" bind:value={color} placeholder="color" />
				<button on:click={handleCreateLedgerAccount}>Create Ledger</button>
			</div>
			<div class="modify-account">
				<input type="text" name="color" bind:value={color} placeholder="color" />
				<input type="text" name="newBalance" bind:value={newBalance} placeholder="new balance" />
				<button on:click={handleModifyLedgerAccount}>Modify Ledger</button>
			</div>
			<div class="get-account">
				<input type="text" name="color" bind:value={color} placeholder="color" />
				<input type="text" name="wallet" bind:value={wallet} placeholder="wallet" />
				<input type="text" name="pda" bind:value={pda} placeholder="pda" />
				<button on:click={() => handleGetLedgerAccount(color, wallet)}>Get Ledger</button>
				{#if fetchedLedgerAccount}
					<div class="account">
						<p><strong>Ledger Account</strong></p>
						<p>Color: {fetchedLedgerAccount.color}</p>
						<p>Balance: {fetchedLedgerAccount.balance}</p>
					</div>
				{/if}
			</div>
		</div>
		<p class="warning">You are connected to: <strong>{$workspaceStore.network}</strong>!</p>
	{:else}
		<p class="warning">You are not connected...</p>
	{/if}
</div>

<style>
	:global(body) {
		padding: 100px;
		margin: 0;
		background-color: #333333;
	}
	.wrapper-app {
		height: 100vh;
		font-family: 'Gill Sans', 'Gill Sans MT', Calibri, 'Trebuchet MS', sans-serif;
	}
	.title {
		text-align: center;
		color: white;
		font-size: 20px;
		margin-bottom: 40px;
	}

	a {
		color: #676796;
	}

	.address {
		position: absolute;
		right: 30px;
		top: 30px;
		border-radius: 5px;
		padding: 10px;
	}

	.wrapper-content {
		border-radius: 5px;
		padding: 0 50px;
		width: 400px;
		margin: 0 auto;
		text-align: center;
		margin-bottom: 30px;
	}

	.connection > h2,
	h3 {
		color: teal;
	}

	.connection > p {
		color: orange;
	}

	.get-account {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		margin: 10px;
	}

	.create-account {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		margin: 20px;
	}

	.modify-account {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		margin: 20px;
	}

	button {
		border: none;
		padding: 16px;
		border-radius: 5px;
		font-size: 16px;
		cursor: pointer;
		color: white;
		background-color: #4e44ce;
	}

	.value {
		font-size: 40px;
		padding: 25px;
		color: white;
	}

	.warning {
		color: #ca4b4b;
		text-align: center;
		padding: 40px;
		font-size: 20px;
	}
</style>
