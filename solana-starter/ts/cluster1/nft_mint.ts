import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createSignerFromKeypair, signerIdentity, generateSigner, percentAmount } from "@metaplex-foundation/umi"
import { createNft, mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";

import wallet from "../turbin3-wallet.json"
import base58 from "bs58";

const metadataURI = "https://gateway.irys.xyz/GAMfXZdpwtpuPn1yfACiruMRKbwJcJ69joU7FyqwSAdU";

const RPC_ENDPOINT = "https://api.devnet.solana.com";
const umi = createUmi(RPC_ENDPOINT);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const myKeypairSigner = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(myKeypairSigner));
umi.use(mplTokenMetadata())

const mint = generateSigner(umi);

(async () => {
    // let tx = ???
    let tx = createNft(umi, {
        mint,
        name: "The SOL A Council",
        symbol: "25Q3A",
        uri: metadataURI,
        sellerFeeBasisPoints: percentAmount(1),
        isMutable: true,
        collectionDetails: null,

    }

    )
    let result = await tx.sendAndConfirm(umi);
    const signature = base58.encode(result.signature);
    console.log(`Succesfully Minted! Check out your TX here:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`)

    console.log("Mint Address: ", mint.publicKey);
})();

//Succesfully Minted! Check out your TX here:
//https://explorer.solana.com/tx/3Vwc93F9smQQExa8h9nWkkVrb34WbXuDz45w7LpLGvbai7zNGoTXGtLjtEiYBHRcQjX2sJwDtqWrhtQW8jRwQccs?cluster=devnet
//Mint Address:  DXr2stURrrECTk8HFphV6ZU7kU1XEAuWhnEmqbhtVUfz