import {
  Connection,
  LAMPORTS_PER_SOL,
  PublicKey,
  clusterApiUrl,
} from '@solana/web3.js';

const connect = () => {
  return new Connection(clusterApiUrl('devnet'), {
    commitment: 'confirmed',
  });
};

const airdropSol = async (address: string, sol: number) => {
  const connection = connect();
  const pubKey = new PublicKey(address);
  const signature = await connection.requestAirdrop(pubKey, sol * LAMPORTS_PER_SOL);
  await connection.confirmTransaction(signature)
};

const getBalenceInSOl = async (address: string) => {
  const connection = connect();
  await airdropSol(address, 0.5);
  const pubKey = new PublicKey(address);
  const bal = await connection.getBalance(pubKey);
  const sol = bal / LAMPORTS_PER_SOL;
  console.log(sol);
};

getBalenceInSOl('5PMQ3wyecaDufJaAnDfvBvszp21Hr7awBFcsXVxQB3Hm');
