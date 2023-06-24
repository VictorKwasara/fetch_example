import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Starter } from '../target/types/starter';

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = anchor.workspace.Starter as Program<Starter>;
const [one] = anchor.web3.PublicKey.findProgramAddressSync(
	[Buffer.from('hello')],
	program.programId
);
const [two] = anchor.web3.PublicKey.findProgramAddressSync(
	[Buffer.from('helloWorld'), provider.publicKey.toBuffer()],
	program.programId
);
describe('starter', () => {
	
	it('Is initialized!', async () => {
    //Initializing the account and passing in the two pda's
		const tx = await program.methods.initialize().accounts({ one, two }).rpc();
		console.log('Your transaction signature', tx);
	});
});

describe('search for acounts', () => {
	

	it('Fetch accounts', async () => {
		// fetch the first account using the authority key
		let acc: any = await program.account.one.all([
			{
				memcmp: {
					offset: 8,
					bytes: provider.publicKey.toBase58().toString(),
				},
			},
		]);

		console.log('Account 1 message => ', acc[0].account);
		let acc_two: any = await program.account.two.fetch(two);
		console.log('Account two message is => ', acc_two);
	});
});
