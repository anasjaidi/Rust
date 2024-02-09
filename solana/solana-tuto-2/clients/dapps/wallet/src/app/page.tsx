'use client';

import Image from 'next/image';
import {
  ConnectionProvider,
  WalletProvider,
  useConnection,
  useWallet,
} from '@solana/wallet-adapter-react';
import {
  WalletModalProvider,
  WalletMultiButton,
} from '@solana/wallet-adapter-react-ui';
import * as web3 from '@solana/web3.js';
import { useMemo, useEffect, useState } from 'react';
import '@solana/wallet-adapter-react-ui/styles.css';
import CMP from './cmp';

export default function Home() {
  const endpoint = web3.clusterApiUrl('devnet');
  const wallets = useMemo(() => [], []);

  return (
    <ConnectionProvider endpoint={endpoint}>
      <WalletProvider wallets={wallets}>
        <WalletModalProvider>
          <WalletMultiButton />
          <CMP />
        </WalletModalProvider>
      </WalletProvider>
    </ConnectionProvider>
  );
}
