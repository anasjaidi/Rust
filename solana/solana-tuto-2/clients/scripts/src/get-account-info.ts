import { PublicKey } from "@solana/web3.js";
import { connect } from "./utils/connect";
import { kp } from "./utils/key-pair";

const getAccountInfos = async (account: PublicKey) => {
    const connection = await connect()

    const infos = await connection.getAccountInfo(account)
    return infos
}

getAccountInfos(kp.publicKey).then(infos => console.log(infos))