import dotenv from 'dotenv';
import { getKeypairFromEnvironment } from '@solana-developers/node-helpers';
import { join } from 'path';

dotenv.config({ path: join(__dirname, "..", "..", '.env') });

export const kp = getKeypairFromEnvironment('KEY_PAIR');
