import dotenv from 'dotenv';
import { getKeypairFromEnvironment } from '@solana-developers/node-helpers';
import { join } from 'path';
import { Keypair } from '@solana/web3.js';

dotenv.config({ path: join(__dirname, "..", "..", '.env') });

export const kp = getKeypairFromEnvironment('KEY_PAIR');

export const rkp = Keypair.generate()