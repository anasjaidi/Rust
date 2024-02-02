import { Keypair } from '@solana/web3.js';
import env from 'dotenv';
import { getKeypairFromEnvironment } from '@solana-developers/helpers';
import path from 'path';

const generate = () => {
  const keypair = Keypair.generate();

  console.log(`The public key is: `, keypair.publicKey.toBase58());
  console.log(`The secret key is: `, keypair.secretKey);
  console.log(`✅ Finished!`);
};

env.config(path.join(__dirname, '.env'));

const load = () => {
  const keypair = getKeypairFromEnvironment('SECRET_KEY');
  console.log(`The public key is: `, keypair.publicKey.toBase58());
  console.log(`The secret key is: `, keypair.secretKey);

  console.log(
    `✅ Finished! We've loaded our secret key securely, using an env file!`
  );
};

load();
