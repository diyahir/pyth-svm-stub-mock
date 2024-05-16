import * as anchor from "@coral-xyz/anchor";
import { BN, Program, web3 } from "@coral-xyz/anchor";
import { PythStub } from "../target/types/pyth_stub";
import { parsePriceData } from "@pythnetwork/client";
import { Transaction } from "@solana/web3.js";
import { expect } from "chai";

const PYTH_PRICE_ACCOUNT_SIZE = 3312;
describe("pyth-stub", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const collateralTokenFeed = new web3.Account();
  const signer = provider.wallet as anchor.Wallet;

  const program = anchor.workspace.PythStub as Program<PythStub>;
  const connection = program.provider.connection;

  it("Is initialized and updating price!", async () => {
    // create a new account
    const createAccountTx = await new Transaction().add(
      web3.SystemProgram.createAccount({
        fromPubkey: program.provider.publicKey,
        newAccountPubkey: collateralTokenFeed.publicKey,
        space: PYTH_PRICE_ACCOUNT_SIZE,
        lamports: await connection.getMinimumBalanceForRentExemption(
          PYTH_PRICE_ACCOUNT_SIZE
        ),
        programId: program.programId,
      })
    );
    const res = await program.provider.sendAndConfirm(createAccountTx, [
      collateralTokenFeed,
    ]);
    // console.log("Account created:", res);

    const price = new BN(123);
    const expo = -4;
    const slot = new BN(1);
    const timeStamp = new BN(100000);

    const tx = await program.methods
      .writePythPrice(price, expo, slot, timeStamp)
      .accounts({
        target: collateralTokenFeed.publicKey,
        signer: signer.publicKey,
      })
      .signers([signer.payer, collateralTokenFeed])
      .rpc();
    console.log("Your transaction signature", tx);

    const latestBlockHash = await connection.getLatestBlockhash();
    await connection.confirmTransaction(
      {
        blockhash: latestBlockHash.blockhash,
        lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
        signature: tx,
      },
      "confirmed"
    );
    const txDetails = await program.provider.connection.getTransaction(tx, {
      maxSupportedTransactionVersion: 0,
      commitment: "confirmed",
    });
    const logs = txDetails?.meta?.logMessages || null;
    if (!logs) {
      console.log("No logs found");
    }
    console.log("Logs", logs);

    const info = await program.provider.connection.getAccountInfo(
      collateralTokenFeed.publicKey
    );
    const parsedPrice = parsePriceData(info?.data);
    // console.log("Parsed price", parsedPrice);
    const expectedPriceComponent = BigInt(price.toString());

    expect(parsedPrice.aggregate.priceComponent).to.be.equal(
      expectedPriceComponent
    );

    expect(parsedPrice.exponent).to.be.equal(expo);

    const expectedSlot = BigInt(slot.toString());
    expect(parsedPrice.validSlot).to.be.equal(expectedSlot);

    const expectedTimeStamp = BigInt(timeStamp.toString());
    expect(parsedPrice.timestamp).to.be.equal(expectedTimeStamp);
  });
});
