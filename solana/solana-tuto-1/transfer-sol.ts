import { getKeypairFromEnvironment } from '@solana-developers/helpers';
import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
  clusterApiUrl,
  sendAndConfirmTransaction,
} from '@solana/web3.js';
import env from 'dotenv';
import path from 'path';

env.config({ path: path.join(__dirname, '.env') });

const connect = () => {
  return new Connection(clusterApiUrl('devnet'), {
    commitment: 'confirmed',
  });
};

const sendTransfer = async (
  sender: PublicKey,
  recipient: PublicKey,
  amount: number,
  signers: Keypair[]
) => {
  const co = connect();
  const transaction = new Transaction();

  const sendSolInstruction = SystemProgram.transfer({
    fromPubkey: sender,
    toPubkey: recipient,
    lamports: LAMPORTS_PER_SOL * amount,
  });

  transaction.add(sendSolInstruction);

  return await sendAndConfirmTransaction(co, transaction, signers);
};

const load = () => {
  const keypair = getKeypairFromEnvironment('SECRET_KEY');
  console.log(`The public key is: `, keypair.publicKey.toBase58());
  console.log(`The secret key is: `, keypair.secretKey);

  console.log(
    `✅ Finished! We've loaded our secret key securely, using an env file!`
  );
  return keypair;
};

const kp = load();

sendTransfer(kp.publicKey, new PublicKey("C1V5Fgm6C8gpeat1rgQf9YpBsGcr677943Nf69Fku6f1"), 0.5, [kp])