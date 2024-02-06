import { Connection, clusterApiUrl } from "@solana/web3.js";

export const connect = async () => {
    return new Connection(clusterApiUrl('devnet'), 'confirmed');
  };