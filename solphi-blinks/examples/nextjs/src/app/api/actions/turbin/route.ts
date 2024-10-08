import {
    createPostResponse,
    createActionHeaders,
    ActionPostResponse,
    ActionGetResponse,
    ActionPostRequest,
  } from '@solana/actions';
  import {
    clusterApiUrl,
    Connection,
    LAMPORTS_PER_SOL,
    PublicKey,
    Keypair,
    SystemProgram,
    Transaction,
    sendAndConfirmTransaction,
  } from '@solana/web3.js';
  
  import "dotenv/config";
  import { getKeypairFromEnvironment } from "@solana-developers/helpers";
   
  const keypair = getKeypairFromEnvironment("SECRET_KEY");

  const senderSecretKey_ = keypair.secretKey;
  const headers = createActionHeaders();
  let icon_ = 'https://i.ibb.co/S3tHzDy/turbin.png';
  
  export const GET = async (req: Request) => {
    try {
      const requestUrl = new URL(req.url);
  
      const baseHref = new URL(
        `/api/actions/solphi?`,
        requestUrl.origin,
      ).toString();
      const payload: ActionGetResponse = {
        type: 'action',
        title: 'Solφ Turbin3 Advertisement',
        icon: icon_,
        description:
          'Earn SOL by watching ads.',
        label: 'Transfer', // this value will be ignored since `links.actions` exists
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
      const {toPubkey } = validatedQueryParams(requestUrl);
      const body: ActionPostRequest = await req.json();
  
      // validate the client provided input
      let account: PublicKey;
      const senderSecretKey = Uint8Array.from(senderSecretKey_);
      const senderWallet = Keypair.fromSecretKey(senderSecretKey);
      try {
        account = new PublicKey(body.account);
      } catch (err) {
        return new Response('Invalid "account" provided', {
          status: 400,
          headers,
        });
      }
      const connection = new Connection(
        process.env.SOLANA_RPC! || clusterApiUrl('mainnet-beta'),
      );
      //const connection = new Connection(clusterApiUrl('devnet'), 'confirmed');
  
      // ensure the receiving account will be rent exempt
      const minimumBalance = await connection.getMinimumBalanceForRentExemption(
        0, // note: simple accounts that just store native SOL have `0` bytes of data
      );
      if (0.001 * LAMPORTS_PER_SOL < minimumBalance) {
        throw `account may not be rent exempt: ${toPubkey.toBase58()}`;
      }

      let solphi: PublicKey = new PublicKey(
        'a514vQv8WeriXr6JYwTMB9gurRJVJW7yqvXghnJFT9Q',
      );
      const transferSolInstruction = SystemProgram.transfer({
        fromPubkey: account,
        toPubkey: solphi,
        lamports: 0.0005 * LAMPORTS_PER_SOL, // reklam ücreti (kesinti, komisyon)
      });


      const transferSolInstruction2 = SystemProgram.transfer({
        fromPubkey: senderWallet.publicKey,
        toPubkey: toPubkey,
        lamports: 0.0031 * LAMPORTS_PER_SOL, // kullanıcının claim ettiği tutar
      });

      const { blockhash, lastValidBlockHeight } =
      await connection.getLatestBlockhash();

      const transaction = new Transaction({
        feePayer: senderWallet.publicKey,
        blockhash,
        lastValidBlockHeight,
      }).add(transferSolInstruction,transferSolInstruction2);

      (async () => {
        try {
          let signature = await sendAndConfirmTransaction(
            connection,
            transaction,
            [senderWallet]
          );
          console.log('Transaction confirmed with signature', signature);
        } catch (error) {
          console.error('Transaction failed', error);
        }
      })();

      await new Promise(resolve => setTimeout(resolve, 1000));

      const payload: ActionPostResponse = await createPostResponse({
        fields: {
          transaction,
          message: `Check your wallet for the transaction`,
        },
        // note: no additional signers are needed
        // signers: [],
      });
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

  
  function validatedQueryParams(requestUrl: URL) {
    let toPubkey: PublicKey = new PublicKey(
      'a514vQv8WeriXr6JYwTMB9gurRJVJW7yqvXghnJFT9Q', // reklam verenden alınan komisyonun gidecği cüzdan adresi
    );
    // icon_ = 'https://upload.wikimedia.org/wikipedia/commons/thumb/4/46/Bitcoin.svg/640px-Bitcoin.svg.png';
  
    try {
      if (requestUrl.searchParams.get('receiverWallet')) {
        toPubkey = new PublicKey(requestUrl.searchParams.get('receiverWallet')!);
      }
    } catch (err) {
      throw 'Invalid input query parameter: receiverWallet';
    }
  
    return {
      toPubkey,
    };
  }