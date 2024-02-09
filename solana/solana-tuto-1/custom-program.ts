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



const load = () => {
  const keypair = getKeypairFromEnvironment('SECRET_KEY');
  return keypair;
};

const kp = load();

const pk = new PublicKey("C1V5Fgm6C8gpeat1rgQf9YpBsGcr677943Nf69Fku6f1")
console.log({kp, pk})