import wallet from "../turbin3-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        // Follow this JSON structure
        // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

        // const image = ???
        const imagePath = "https://gateway.irys.xyz/UJDzU4PwfP6rEFhmVFVxRyvkH7zEcKocGwtiJySMTsf";
        
        const metadata = {
            name: "The SOL A Council",
            symbol: "25Q3A",
            description: "NFT for the Turbin3 25Q3 Rug Day",
            image: imagePath,
            attributes: [
                { trait_type: "LLM", value: "generated"}
            ],
            properties: {
                files: [
                    {
                        type: "image/png",
                        uri: imagePath,
                    },
                ],
                category: "image"
            },    
            creators: [
                {
                address: signer.publicKey.toString(),
                share: 100
                }
            ]    
        };
        const myUri = await umi.uploader.uploadJson(metadata);
        console.log("Your metadata URI: ", myUri);
    }
    catch (error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
//Your metadata URI:  https://gateway.irys.xyz/GAMfXZdpwtpuPn1yfACiruMRKbwJcJ69joU7FyqwSAdU