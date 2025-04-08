import { 
    Connection, 
    Keypair, 
    LAMPORTS_PER_SOL, 
    PublicKey, 
    SystemProgram,
    Transaction
} from "@solana/web3.js";
import { CounterAccount, schema, COUNTER_SIZE  } from "./state-types/types";
import * as borsh from "borsh";
import { expect, test } from "bun:test";

const connection = new Connection("http://localhost:8899", "confirmed");

const payerAccount: Keypair = Keypair.generate();
const counterAccount: Keypair = Keypair.generate();
const programId = new PublicKey("DWHnoMQJDETqg5GfmfLMmjSFCMj8m6QKBvDmvDJo26ih");

test('Account is initialized', async () => {
    const txn = await connection.requestAirdrop(
        payerAccount.publicKey,
        2 * LAMPORTS_PER_SOL,
    );

    await connection.confirmTransaction(txn);

    const data = await connection.getAccountInfo(
        counterAccount.publicKey,
        "confirmed",
    );

    const lamports = await connection.getMinimumBalanceForRentExemption(
        COUNTER_SIZE,
    );

    const createCounterAccountIx = SystemProgram.createAccount({
        fromPubkey: payerAccount.publicKey,
        newAccountPubkey: counterAccount.publicKey,
        lamports,
        space: COUNTER_SIZE,
        programId: programId
    });

    const createAccountTxn = new Transaction();
    createAccountTxn.add(createCounterAccountIx);

    const TransactionHash = await connection.sendTransaction(
        createAccountTxn,
        [
            payerAccount,
            counterAccount,
        ]
    );

    await connection.confirmTransaction(TransactionHash);

    const counterAccountInfo = await connection.getAccountInfo(counterAccount.publicKey);
    if (!counterAccountInfo?.data) {
        throw new Error("Counter account data is undefined");
    }
    const counter = borsh.deserialize(schema, counterAccountInfo.data);

    console.log("Counter:", counter);
    console.log("Type:", typeof counter);
    console.log("Raw Data:", counterAccountInfo.data);

    expect(counter).toEqual(new CounterAccount({ count: BigInt(0)}));
});
