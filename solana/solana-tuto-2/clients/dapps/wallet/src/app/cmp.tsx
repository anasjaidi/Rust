'use client';
import { useConnection, useWallet } from '@solana/wallet-adapter-react';
import React, { useEffect, useState } from 'react';
import * as web3 from '@solana/web3.js';
const pk = 'C1V5Fgm6C8gpeat1rgQf9YpBsGcr677943Nf69Fku6f1';

function CMP() {
  const [balance, setBalance] = useState(0);
  const { publicKey, connect, sendTransaction } = useWallet();

  const { connection } = useConnection();
  useEffect(() => {
    connect();
    if (!(connection && publicKey)) {
      return;
    }
    connection.onAccountChange(
      publicKey,
      (updatedAccountInfo) => {
        setBalance(updatedAccountInfo.lamports / web3.LAMPORTS_PER_SOL);
      },
      'confirmed'
    );

    connection.getAccountInfo(publicKey).then((info) => {
      setBalance(info?.lamports!);
    });
  }, [connection, publicKey, connect]);

  const onTransfer = async (_: any) => {
    _.preventDefault();

    const transaction = new web3.Transaction();
    const recipientPubKey = new web3.PublicKey(pk);

    const sendSolInstruction = web3.SystemProgram.transfer({
      fromPubkey: publicKey!,
      toPubkey: recipientPubKey,
      lamports: web3.LAMPORTS_PER_SOL * 0.1,
    });

    transaction.add(sendSolInstruction);
    sendTransaction(transaction, connection, {}).then((sig: any) => {
      console.log(sig);
    });
  };
  return (
    <>
      <div>{balance / web3.LAMPORTS_PER_SOL}</div>
      <button onClick={onTransfer}>Transfer</button>
    </>
  );
}

export default CMP;
