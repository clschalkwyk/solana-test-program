import * as anchor from "@coral-xyz/anchor";
import {SystemProgram} from "@solana/web3.js"; // fok anchor.web3
import {assert} from "chai";


describe('mycalcudapp', () =>{
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  const calculatorAccount = anchor.web3.Keypair.generate();
  const program = anchor.workspace.mycaludapp;

  it('Creates a calculator', async () =>{
    await program.rpc.create("Welcome Frikkie!", {
      accounts:{
        calculator:calculatorAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId
      },
      signers:[calculatorAccount]
    })
    const account = await program.account.calculator.fetch(calculatorAccount.publicKey);
    console.log(account)
    assert.ok(account.greeting === "Welcome Frikkie!")
  })
})
