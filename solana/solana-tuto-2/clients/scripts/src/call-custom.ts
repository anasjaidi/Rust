import {
  Keypair,
  PublicKey,
  Transaction,
  sendAndConfirmTransaction,
} from '@solana/web3.js';
import { connect } from './utils/connect';
import { kp } from './utils/key-pair';

const call = async (signer: Keypair, programId: PublicKey) => {
  const connection = await connect();
  const tx = new Transaction().add({
    keys: [
      {
        pubkey: signer.publicKey,
        isSigner: true,
        isWritable: false,
      },
    ],
    programId,
  });
  return await sendAndConfirmTransaction(connection, tx, [signer]);
};

call(kp, new PublicKey('58g1AhDfBkPgFiskTs9Rgc224G8QsaF32YqJWcFbuBXK')).then(
  (h) => console.log(h)
);
