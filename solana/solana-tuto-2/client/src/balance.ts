import { PublicKey, LAMPORTS_PER_SOL } from '@solana/web3.js';
import { kp } from './utils/key-pair';
import { connect } from './utils/connect';



const getBalanceInSol = async (pk: PublicKey) => {
    const connection = await connect()
    const balance = await connection.getBalance(pk)
    return balance / LAMPORTS_PER_SOL
}
getBalanceInSol(kp.publicKey).then(b => console.log(b))