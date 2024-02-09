import { connect } from './utils/connect';

import * as web3 from '@solana/web3.js'
import * as token from '@solana/spl-token'
import { kp } from './utils/key-pair';

const createSplToken = async (payer: web3.Keypair) => {
    const connection = await connect()
    const lamports = await token.getMinimumBalanceForRentExemptMint(connection);
    const accountKeypair = web3.Keypair.generate();
    const programId = token.TOKEN_PROGRAM_ID
  
    const transaction = new web3.Transaction().add(
      web3.SystemProgram.createAccount({
        fromPubkey: payer.publicKey,
        newAccountPubkey: accountKeypair.publicKey,
        space: token.MINT_SIZE,
        lamports,
        programId,
      }),
      token.createInitializeMintInstruction(
        accountKeypair.publicKey,
        2,
        payer.publicKey,
        payer.publicKey,
        programId
      )
    );
    const th = await web3.sendAndConfirmTransaction(connection, transaction, [payer])
    return th
}

createSplToken(kp).then(res => console.log(res))