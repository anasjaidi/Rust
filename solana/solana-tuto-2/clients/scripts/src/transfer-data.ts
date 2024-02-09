import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
  TransactionInstruction,
  sendAndConfirmTransaction,
} from '@solana/web3.js';
import { connect } from './utils/connect';
import { kp, rkp } from './utils/key-pair';
import { getBalanceInSol } from './balance';

const transfer = async (from: Keypair, to: PublicKey, amount: number) => {
  const connection = await connect();
  const tx = new Transaction();
  const autoIs = SystemProgram.transfer({
    fromPubkey: from.publicKey,
    toPubkey: to,
    lamports: amount,
  });
  const isData = Buffer.alloc(12); // 4 for type + 8 for Lmaports
  isData.writeUint32LE(2, 0);
  isData.writeBigUint64LE(BigInt(amount), 4);
  const manualIs = new TransactionInstruction({
    keys: [
      {
        isSigner: true,
        isWritable: true,
        pubkey: from.publicKey,
      },
      {
        isSigner: false,
        isWritable: true,
        pubkey: to,
      },
    ],
    programId: SystemProgram.programId,
    data: isData,
  });
  tx.add(manualIs);
  const txh = await sendAndConfirmTransaction(connection, tx, [from]);
  return txh;
};

transfer(kp, rkp.publicKey, 0.5 * LAMPORTS_PER_SOL).then(async (txh) => {
  await transfer(rkp, kp.publicKey, 0.49 * LAMPORTS_PER_SOL);
  const b = await getBalanceInSol(rkp.publicKey);
  console.log(b);
  console.log(txh);
});
