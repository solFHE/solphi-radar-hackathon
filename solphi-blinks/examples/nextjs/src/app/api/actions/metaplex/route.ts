import {
    createActionHeaders,
    ActionPostResponse,
    ActionGetResponse,
    ActionPostRequest,
  } from '@solana/actions';
  import {
    PublicKey,
  } from '@solana/web3.js';

  import { createUmi } from '@metaplex-foundation/umi-bundle-defaults';
  import { createSignerFromKeypair, signerIdentity, publicKey, generateSigner, createNoopSigner } from '@metaplex-foundation/umi';
  import { mplCore, create, fetchAssetV1, transferV1 } from '@metaplex-foundation/mpl-core';

  import "dotenv/config";
  import { getKeypairFromEnvironment } from "@solana-developers/helpers";
   
  const keypair = getKeypairFromEnvironment("SECRET_KEY");

  const senderSecretKey_ = keypair.secretKey;
  const headers = createActionHeaders();
  let icon_ = 'https://media4.giphy.com/media/v1.Y2lkPTc5MGI3NjExMW43bzM2aTZnMm1zaXUwYWhqeHgzanZydHhlZmc5N2Z6cWJkaWZ5cSZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/gpUHcunoaG8tdXzJ6A/giphy.gif';
  
  export const GET = async (req: Request) => {
    try {
      const requestUrl = new URL(req.url);
  
      const baseHref = new URL(
        `/api/actions/metaplex?`,
        requestUrl.origin,
      ).toString();
      const payload: ActionGetResponse = {
        type: 'action',
        title: 'Solφ Advertisement',
        icon: icon_,
        description:
          'Mint Metaplex Core NFTs by watching ads.',
        label: 'Transfer', // this value will be ignored since `links.actions` exists
        disabled: false,
        links: {
          actions: [
            {
              label: 'Send', // button text
              href: `${baseHref}receiverWallet={receiverWallet}`, // this href will have a text input
              parameters: [
                {
                  name: 'receiverWallet', // parameter name in the `href` above
                  label: 'Receiver Wallet', // placeholder of the text input
                  required: true,
                },
              ],
            },
          ],
        },
      };
  
      return Response.json(payload, {
        headers,
      });
    } catch (err) {
      console.log(err);
      let message = 'An unknown error occurred';
      if (typeof err == 'string') message = err;
      return new Response(message, {
        status: 400,
        headers,
      });
    }
  };
  
  // DO NOT FORGET TO INCLUDE THE `OPTIONS` HTTP METHOD
  // THIS WILL ENSURE CORS WORKS FOR BLINKS
  export const OPTIONS = async (req: Request) => {
    return new Response(null, { headers });
  };
  
  export const POST = async (req: Request) => {
    try {
      const requestUrl = new URL(req.url);
      const body: ActionPostRequest = await req.json();
  
      // validate the client provided input
      let account: PublicKey;

      try {
        account = new PublicKey(body.account);
      } catch (err) {
        return new Response('Invalid "account" provided', {
          status: 400,
          headers,
        });
      }

      const transaction = await prepareTransaction(account);

      await new Promise(resolve => setTimeout(resolve, 1000));
      
      const response: ActionPostResponse = {
        transaction: Buffer.from(transaction).toString("base64"),
      };

      return Response.json(response, {
        headers,
      });
    } catch (err) {
      console.log(err);
      let message = 'An unknown error occurred';
      if (typeof err == 'string') message = err;
      return new Response(message, {
        status: 400,
        headers,
      });
    }

    async function prepareTransaction(user: PublicKey) {
      const umi = createUmi('https://api.mainnet-beta.solana.com');

      const senderSecretKey = Uint8Array.from(senderSecretKey_);
      let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(senderSecretKey));
      const adminSigner = createSignerFromKeypair(umi, keypair);
      umi.use(signerIdentity(createNoopSigner(publicKey(user))));
    
      const asset = generateSigner(umi);
      console.log("Asset: ", asset.publicKey.toString());
    

      const assetAddress = generateSigner(umi);

      const tx = await create(umi, {
        name: "Metaplex",
        uri: "https://example.com/asset.json",
        asset: assetAddress,
        authority: adminSigner,
      }).buildAndSign(umi);
    
      return umi.transactions.serialize(tx);
    }
  };