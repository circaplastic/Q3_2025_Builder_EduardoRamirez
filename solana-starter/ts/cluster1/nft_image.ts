import wallet from "../turbin3-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"
import { readFile } from "fs/promises"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        //1. Load image
        // const image = ???
        const image = await readFile("/home/lalo/Q3_2025_Builder_EduardoRamirez/solana-starter/ts/cluster1/thesolAcouncil.png");
        //2. Convert image to generic file.
        // const [myUri] = ??? 
        const genericFile = createGenericFile(image, "solAcouncil.png", { contentType: "image/png" });
        //3. Upload image
        const [myUri] = await umi.uploader.upload([genericFile]);
        // console.log("Your image URI: ", myUri);
        console.log("Your image URI: ", myUri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();

// Your image URI:  https://gateway.irys.xyz/UJDzU4PwfP6rEFhmVFVxRyvkH7zEcKocGwtiJySMTsf
// Your image(rug) URI:  https://gateway.irys.xyz/8qRR6N4WGJFZ9jgrEB3fRETT8eKwVVGMjivtpVdQGTXP