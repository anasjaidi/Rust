import { Connection, Cluster, clusterApiUrl, PublicKey, LAMPORTS_PER_SOL } from '@solana/web3.js';
import { kp } from './utils/key-pair';
const connect = async () => {
  return new Connection(clusterApiUrl('devnet'), 'confirmed');
};


const getBalanceInSol = async (pk: PublicKey) => {
    const connection = await connect()
    const balance = await connection.getBalance(pk)
    return balance / LAMPORTS_PER_SOL
}
getBalanceInSol(kp.publicKey).then(b => console.log(b))