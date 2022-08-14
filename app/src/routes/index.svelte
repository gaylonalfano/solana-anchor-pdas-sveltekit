<script lang="ts">
	import { WalletMultiButton } from '@svelte-on-solana/wallet-adapter-ui';
	import { walletStore } from '@svelte-on-solana/wallet-adapter-core';
	import { workSpace as workspaceStore } from '@svelte-on-solana/wallet-adapter-anchor';
	import { fly } from 'svelte/transition';
	import * as anchor from '@project-serum/anchor';

	/* let value; */

	/* $: console.log('value: ', value); */

	/* async function createCounter() { */
	/* 	try { */
	/* 		/1* interact with the program via rpc *1/ */
	/* 		await $workSpace.program.rpc.create({ */
	/* 			accounts: { */
	/* 				baseAccount: $workSpace.baseAccount.publicKey, */
	/* 				user: $walletStore.publicKey, */
	/* 				systemProgram: $workSpace.systemProgram.programId */
	/* 			}, */
	/* 			signers: [$workSpace.baseAccount] */
	/* 		}); */

	/* 		const account = await $workSpace.program.account.baseAccount.fetch( */
	/* 			$workSpace.baseAccount.publicKey */
	/* 		); */
	/* 		value = account.count.toString(); */
	/* 	} catch (err) { */
	/* 		console.log('Transaction error: ', err); */
	/* 	} */
	/* } */

	/* async function increment() { */
	/* 	await $workSpace.program.rpc.increment({ */
	/* 		accounts: { */
	/* 			baseAccount: $workSpace.baseAccount.publicKey */
	/* 		} */
	/* 	}); */

	/* 	const account = await $workSpace.program.account.baseAccount.fetch( */
	/* 		$workSpace.baseAccount.publicKey */
	/* 	); */
	/* 	value = account.count.toString(); */
	/* } */

	// let testKeypair1 = generateKeypair();
	// let testKeypair2 = generateKeypair();

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
	/* const provider = workspaceStore.provider; */
	/* anchor.setProvider(provider); */

	// Q: How do I get my Program<T> i.e., as Program<SolanaAnchorPdasSveltekit>??
	// A: NOT SURE IT'S NECESSARY as there's a lot of sync to the IDL in the AnchorConnectionProvider Svelte Component
	// https://github.com/svelte-on-solana/wallet-adapter/blob/master/packages/anchor/src/lib/AnchorConnectionProvider.svelte
	/* const program = workspaceStore.program.idl. */
	/* 	.SolanaAnchorPdasSveltekit as Program<SolanaAnchorPdasSveltekit>; */

	$: {
		console.log('workspaceStore', $workspaceStore);
		console.log('walletStore', $walletStore);
		console.log('wallet', wallet);
		console.log('color', color);
		console.log('ledgerAccount', ledgerAccount);
	}

	// NOTE Ran my DEVNET tests to generate these for now...
	const testWallet1 = "SSyUdM98Z6Fa5faGyo5qrBmxFuB6koZt7cUt4i9JyXt";
	const testPda1 = "Ezu4mwWzm4KSJ9xaGAbvofAVxAsTKLNjJorhR7P2oKkk" // expect: red 4
	const testPda1Color = "red";
	const testPda2 = "2eQc39fkTyRHproyQR9X6cq62cNhnhehqzgqprxBryMn"; // expect: blue 3
	const testPda2Color = "blue";
	let wallet = testWallet1;
	let pda = testPda1;
	let color = testPda1Color;
	let ledgerAccount;
	let newLedgerAccount;


	let colorSeed: string = '';

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

	async function createLedgerAccount(
		color: string,
		pda: anchor.web3.PublicKey,
		wallet: anchor.web3.Keypair
	) {
		// Calls the program's on-chain create_ledger instruction function
		// to create a ledger account LOCATED at our generated PDA address!
		// NOTE This requires same args i.e., Context, color, system
		// NOTE We're technically creating a ledger account located at
		// this PDA address!
		await $workspaceStore.program?.methods
			.createLedger(color)
			.accounts({
				// Q: Do I use snake_case or camelCase?
				// NOTE Tutorial used camelCase even though it's snake_case in program
				// A: Looks like I use camelCase...
				ledgerAccount: pda,
				wallet: wallet.publicKey
				// NOTE Anchor automatically adds System Program (and other programs if required)
			})
			.signers([wallet])
			.rpc();
	}

		// TODO Try to add a button and an input to trigger creating a new ledger

	async function getLedgerAccount(color: string, wallet: string) {
		/* let pda = await derivePda(color, $walletStore.publicKey); */
		let walletPubkey = new anchor.web3.PublicKey(wallet);
		let pda = await derivePda(color, walletPubkey);
		let data = await $workspaceStore.program?.account.ledger.fetch(pda);
		// Update global ledgerAccount var
		ledgerAccount = data;
		return data;
	}

	// async function getPdaAccount(seeds: string[]) {
	// 	/* let pda = await derivePda(color, $walletStore.publicKey); */
	// 	let walletPubkey = new anchor.web3.PublicKey(seeds[1]);
	// 	let pda = await derivePda(seeds[0], walletPubkey);
	// 	let data = await $workspaceStore.program?.account.ledger.fetch(pda);
	// 	// Update global ledgerAccount var
	// 	ledgerAccount = data;
	// 	return data;
	// }

	async function modifyLedgerAccount(
		color: string,
		newBalance: number,
		wallet: anchor.web3.Keypair
	) {
		console.log('------------------------------------');
		// 1. Retrieve the PDA using helper
		// NOTE Don't pass pda address. Just pass color
		let data; // Is type Ledger
		let pda = await derivePda(color, wallet.publicKey);

		// 2. Try to retreive PDA account data if it exists
		console.log(`Checking if account ${shortKey(pda)} exists for color: ${color}...`);
		try {
			// NOTE We're technically seeing if our PDA address has a
			// ledger account at its location (address)
			data = await $workspaceStore.program?.account.ledger.fetch(pda);
			console.log(`Account already exists!`);
		} catch (e) {
			// console.log(e);
			console.log(`Account ${shortKey(pda)} does NOT exist!`);
			console.log('Creating account...');
			// 1. Create account using helper that calls program instruction
			await createLedgerAccount(color, pda, wallet);
			// 2. Retrieve account data
			data = await $workspaceStore.program?.account.ledger.fetch(pda);
		}

		console.log(`SUCCESS! Wallet: ${shortKey(wallet.publicKey)} -- PDA: ${shortKey(pda)} `);
		console.log('Our PDA has a ledger account with data:\n');
		console.log(`    Color: ${data?.color}   Balance: ${data?.balance}`);
		console.log(`Modifying balance of ${data?.color} from ${data?.balance} to ${newBalance}`);

		// 3. Make our modifications to the account using on-chain program function
		// NOTE This is another program function instruction
		await $workspaceStore.program?.methods
			.modifyLedger(newBalance)
			.accounts({
				ledgerAccount: pda,
				wallet: wallet.publicKey
			})
			.signers([wallet])
			.rpc();

		// 4. Retrieve the updated data one last time
		data = await $workspaceStore.program?.account.ledger.fetch(pda);
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
			<div class="top">
				<h2>workspace.provider.connection established!</h2>
				<h3>Connected wallet address:</h3>
				<p>Wallet: {$walletStore.wallet?.publicKey}</p>
				<p>Program: {$workspaceStore.program?.programId}</p>
				<p>Color: {colorSeed}</p>
			</div>
			<div class="create-account">
				<input type="text" name="color" bind:value="{color}" placeholder="color">
				<button on:click="{() => createLedgerAccount(color)}">Create Ledger</button>
			</div>
			<div class="get-account">
				<input type="text" name="color" bind:value="{color}" placeholder="color">
				<input type="text" name="wallet" bind:value="{wallet}" placeholder="wallet">
				<input type="text" name="pda" bind:value="{pda}" placeholder="pda">
				<button on:click="{() => getLedgerAccount(color, wallet)}">Get Ledger</button>
				{#if ledgerAccount}
					<div class="account">
						<p><strong>Ledger Account</strong></p>
						<p>Color: {ledgerAccount.color}</p>
						<p>Balance: {ledgerAccount.balance}</p>
					</div>
				{/if}
			</div>
		</div>
		<p class="warning">You are connected to:  <strong>{$workspaceStore.network}</strong>!</p>
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
		padding: 50px;
		width: 400px;
		margin: 0 auto;
		text-align: center;
		margin-bottom: 30px;
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
