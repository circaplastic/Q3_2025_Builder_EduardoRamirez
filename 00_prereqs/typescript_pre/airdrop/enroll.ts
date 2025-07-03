import {Connection, Keypair, PublicKey} from "@solana/web3.js";
import {Program, Wallet, AnchorProvider} from "@coral-xyz/anchor";
import {IDL, Turbin3Prereq} from "../programs/Turbin3_prereq";
import wallet from "./Turbin3-wallet.json";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";

const MPL_CORE_PROGRAM_ID = new PublicKey("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d");
//Import the keypair from wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
//Create a devnet collection
const connection = new Connection("https://api.devnet.solana.com");
//Create anchor provider
const provider = new AnchorProvider(connection, new Wallet(keypair), {commitment: 'confirmed'});
//Create Anchor program to interact with Turbin3 devnet program
const program: Program<Turbin3Prereq> = new Program(IDL, provider);
const account_seeds = [
    Buffer.from("prereqs"),
    keypair.publicKey.toBuffer(),
];
const [account_key, _account_bump] = PublicKey.findProgramAddressSync(account_seeds, program.programId);
const mintCollection = new PublicKey("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2");
const mintTs = Keypair.generate();

//Generate PDA for authority
const authority_seeds = [
    Buffer.from("collection"),
    mintCollection.toBuffer(),
];
const [authority_key, _authority_bump] = PublicKey.findProgramAddressSync(authority_seeds, program.programId);



/*
//Execute the initialize transaction
(async () => {
    try {
        const txhash = await program.methods
        .initialize("circaplastic")
        .accountsPartial({
            user: keypair.publicKey,
            account: account_key,
            system_program: SYSTEM_PROGRAM_ID,
        })
        .signers([keypair])
        .rpc();
        console.log(`Success! Check out your TX here: https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
    } catch (e) {
        console.error(`Oops, something went wrong: ${e}`);
    }
})();

*/

//Execute the submit Ts transaction
(async () => {
    try {
        const txhash = await program.methods
        .submitTs()
        .accountsPartial({
            user: keypair.publicKey,
            account: account_key,
            mint: mintTs.publicKey,
            collection: mintCollection,
            authority: authority_key,
            mpl_core_program: MPL_CORE_PROGRAM_ID,
            system_program: SYSTEM_PROGRAM_ID,
        })
        .signers([keypair, mintTs])
        .rpc();
        console.log(`Success! Check out your TX here: https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
    } catch (e) {
        console.error(`Oops, something went wrong: ${e}`);
    }
})();
