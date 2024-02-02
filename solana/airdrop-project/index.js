const {
  Connection,
  PublicKey,
  clusterApiUrl,
  Keypair,
  LAMPORTS_PER_SOL,
} = require('@solana/web3.js');

const { publicKey, secretKey } = new Keypair();

const airdropSol = async () => {
    try {
        const con = new Connection(clusterApiUrl('devnet'), 'confirmed')
        const sig = await con.requestAirdrop(publicKey, 2 * LAMPORTS_PER_SOL);
        await con.confirmTransaction(sig);
    } catch (error) {
        console.error(error)
    }
}

const getBalence = async () => {
    try {
        const con = new Connection(clusterApiUrl('devnet'), 'confirmed')
        console.log(await con.getBalance(publicKey))
    } catch (error) {
        console.error(error)
    }
}
airdropSol().then(res => {

    getBalence()
})