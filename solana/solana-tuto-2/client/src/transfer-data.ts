import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
  sendAndConfirmTransaction,
} from '@solana/web3.js';
import { connect } from './utils/connect';
import { kp, rkp } from './utils/key-pair';
import { getBalanceInSol } from './balance';

const transfer = async (from: Keypair, to: PublicKey, amount: number) => {
  const connection = await connect();
  const tx = new Transaction();
  const is = SystemProgram.transfer({
    fromPubkey: from.publicKey,
    toPubkey: to,
    lamports: amount,
  });
  tx.add(is);
  const txh = await sendAndConfirmTransaction(connection, tx, [from]);
  return txh;
};

transfer(kp, rkp.publicKey, 0.5 * LAMPORTS_PER_SOL).then(async (txh) => {
    await transfer(rkp, kp.publicKey, 0.4 * LAMPORTS_PER_SOL)
    // const b = getBalanceInSol(rkp.publicKey);
    // console.log(b)
  console.log(txh);
});
