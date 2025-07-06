import wallet from "../turbin3-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { 
    createMetadataAccountV3, 
    CreateMetadataAccountV3InstructionAccounts, 
    CreateMetadataAccountV3InstructionArgs,
    DataV2Args
} from "@metaplex-foundation/mpl-token-metadata";
import { createSignerFromKeypair, signerIdentity, publicKey } from "@metaplex-foundation/umi";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { option } from "@metaplex-foundation/umi/serializers";

// Define our Mint address
const mint = publicKey("8syMZfWHVapuDkSQqXRXrmo2n3y99oiECzt2vEsb1paY")

// Create a UMI connection
const umi = createUmi('https://api.devnet.solana.com');
const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(createSignerFromKeypair(umi, keypair)));

(async () => {
    try {
        // Start here
        // let accounts: CreateMetadataAccountV3InstructionAccounts = {
        //     ???
        // }
        let accounts: CreateMetadataAccountV3InstructionAccounts = {
            mint,
            mintAuthority: signer,
        }
        // let data: DataV2Args = {
        //     ???
        // }
        let data: DataV2Args = {
            name: "THE SOL A COUNCIL",
            symbol: "25Q3A",
            uri: "https://arweave.net/123456",
            sellerFeeBasisPoints: 5,
            creators: null,
            collection: null,
            uses: null,
        }
        // let args: CreateMetadataAccountV3InstructionArgs = {
        //     ???
        // }
        let args: CreateMetadataAccountV3InstructionArgs = {
            data,
            isMutable: true,
            collectionDetails: null,
        }
        // let tx = createMetadataAccountV3(
        //     umi,
        //     {
        //         ...accounts,
        //         ...args
        //     }
        // )
        let tx = createMetadataAccountV3(
            umi,
            {
             ...accounts,
             ...args
            }
        )
        let result = await tx.sendAndConfirm(umi);
        console.log(bs58.encode(result.signature));
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();
// Returned: Signature: HRiEKXsEuKKP3MLCBJEhmmWcC8ndDpPPGKZpUqkvu3kNB8uQBSkQxJQeoSbJpQio9cHjCv426CDLKc1KLpEibkr