import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import {
  createAssociatedTokenAccountInstruction,
  createMint,
  createMintToInstruction,
  getAssociatedTokenAddressSync,
} from "@solana/spl-token";
import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  ParsedAccountData,
  PublicKey,
  SystemProgram,
  TransactionMessage,
  VersionedTransaction,
} from "@solana/web3.js";
import { assert } from "chai";
import fs from "fs";
import path from "path";
import { SplSnapshot } from "../target/types/spl_snapshot";

function getKeypair(name: string): Keypair {
  const file = path.join(__dirname, "keys", `${name}.json`);
  if (!fs.existsSync(file)) {
    const keypair = Keypair.generate();
    fs.writeFileSync(
      file,
      JSON.stringify(Array.from(keypair.secretKey)),
      "utf-8"
    );
  }
  return Keypair.fromSecretKey(
    new Uint8Array(JSON.parse(fs.readFileSync(file, "utf-8")))
  );
}

async function getTokenBalances<T extends {}>(
  connection: Connection,
  accounts: T
) {
  const entries = Object.entries(accounts);
  const { value: result } = await connection.getMultipleParsedAccounts(
    entries.map((e) => e[1] as PublicKey),
    { commitment: "processed" }
  );
  type K = keyof T;
  return result.reduce(
    (res, r, ix) => ({
      ...res,
      [entries[ix][0]]:
        (r?.data as ParsedAccountData)?.parsed?.info?.tokenAmount.uiAmount || 0,
    }),
    {}
  ) as { [key in K]: number };
}

async function ensureFunded(connection: Connection, wallet: PublicKey) {
  const balance = await connection.getBalance(wallet);
  if (balance < 100_000) {
    const signature = await connection.requestAirdrop(wallet, LAMPORTS_PER_SOL);
    await connection.confirmTransaction(signature, "confirmed");
  }
}

describe("spl_snapshot", async () => {
  // Configure the client to use the local cluster.
  const authority = getKeypair("authority");
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SplSnapshot as Program<SplSnapshot>;
  const connection = program.provider.connection;

  const wallet1 = getKeypair("wallet1");
  const wallet2 = getKeypair("wallet2");
  const wallet3 = getKeypair("wallet3");

  const mintKeypair = getKeypair("mint");
  const decimals = 6;

  const ata1 = getAssociatedTokenAddressSync(
    mintKeypair.publicKey,
    wallet1.publicKey
  );
  const ata2 = getAssociatedTokenAddressSync(
    mintKeypair.publicKey,
    wallet2.publicKey
  );
  const ata3 = getAssociatedTokenAddressSync(
    mintKeypair.publicKey,
    wallet3.publicKey
  );

  it("should create mint and ATAs", async () => {
    await ensureFunded(connection, authority.publicKey);
    await ensureFunded(connection, wallet1.publicKey);
    await ensureFunded(connection, wallet2.publicKey);
    await ensureFunded(connection, wallet3.publicKey);

    await createMint(
      connection,
      authority,
      authority.publicKey,
      null,
      decimals,
      mintKeypair
    );

    const instructions = [
      createAssociatedTokenAccountInstruction(
        authority.publicKey,
        ata1,
        wallet1.publicKey,
        mintKeypair.publicKey
      ),
      createAssociatedTokenAccountInstruction(
        authority.publicKey,
        ata2,
        wallet2.publicKey,
        mintKeypair.publicKey
      ),
      createMintToInstruction(
        mintKeypair.publicKey,
        ata1,
        authority.publicKey,
        10 * 10 ** decimals
      ),
    ];

    const latestBlockhash =
      await program.provider.connection.getLatestBlockhash();
    const tx = new VersionedTransaction(
      new TransactionMessage({
        payerKey: authority.publicKey,
        instructions: instructions,
        recentBlockhash: latestBlockhash.blockhash,
      }).compileToV0Message()
    );
    tx.sign([authority]);

    const signature = await program.provider.connection.sendTransaction(tx);
    await program.provider.connection.confirmTransaction(
      { signature, ...latestBlockhash },
      "confirmed"
    );
  });

  describe("with existing token account", () => {
    const tokenAccount = ata1;
    const [snapshotAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from("snapshot"), tokenAccount.toBuffer()],
      program.programId
    );
    const destinationAccount = ata2;

    it("should credit the increment", async () => {
      const increment = 5;
      const accounts = {
        tokenAccount,
        destinationAccount,
      };
      const initialBalances = await getTokenBalances(connection, accounts);

      const instructions = [
        // snapshot
        await program.methods
          .snapshot()
          .accounts({
            snapshotAccount,
            tokenAccount,
            owner: wallet1.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .instruction(),
        // mint to token account
        createMintToInstruction(
          mintKeypair.publicKey,
          ata1,
          authority.publicKey,
          increment * 10 ** decimals
        ),
        // transfer to destination account
        await program.methods
          .transfer()
          .accounts({
            snapshotAccount,
            tokenAccount,
            destinationAccount,
            owner: wallet1.publicKey,
            tokenProgram: TOKEN_PROGRAM_ID,
          })
          .instruction(),
      ];

      const latestBlockhash =
        await program.provider.connection.getLatestBlockhash();
      const tx = new VersionedTransaction(
        new TransactionMessage({
          payerKey: wallet1.publicKey,
          instructions,
          recentBlockhash: latestBlockhash.blockhash,
        }).compileToV0Message()
      );
      tx.sign([wallet1, authority]);

      const signature = await program.provider.connection.sendTransaction(tx);

      await program.provider.connection.confirmTransaction(
        { signature, ...latestBlockhash },
        "confirmed"
      );

      const finalBalances = await getTokenBalances(connection, accounts);

      assert.equal(
        finalBalances.tokenAccount,
        initialBalances.tokenAccount,
        "Token account balance should not have changed"
      );
      assert.equal(
        finalBalances.destinationAccount,
        initialBalances.destinationAccount + increment,
        "Destination account received the transaction increment"
      );
    });
  });

  describe("with new token account", () => {
    const tokenAccount = ata3;
    const [snapshotAccount] = PublicKey.findProgramAddressSync(
      [Buffer.from("snapshot"), tokenAccount.toBuffer()],
      program.programId
    );
    const destinationAccount = ata2;

    it("should credit the increment", async () => {
      const increment = 5;
      const accounts = {
        tokenAccount,
        destinationAccount,
      };
      const initialBalances = await getTokenBalances(connection, accounts);

      const instructions = [
        // snapshot
        await program.methods
          .snapshot()
          .accounts({
            snapshotAccount,
            tokenAccount,
            owner: wallet3.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .instruction(),
        // create token account
        createAssociatedTokenAccountInstruction(
          wallet3.publicKey,
          ata3,
          wallet3.publicKey,
          mintKeypair.publicKey
        ),
        // mint to token account
        createMintToInstruction(
          mintKeypair.publicKey,
          ata3,
          authority.publicKey,
          increment * 10 ** decimals
        ),
        // transfer to destination account
        await program.methods
          .transfer()
          .accounts({
            snapshotAccount,
            tokenAccount,
            destinationAccount,
            owner: wallet3.publicKey,
            tokenProgram: TOKEN_PROGRAM_ID,
          })
          .instruction(),
      ];

      const latestBlockhash =
        await program.provider.connection.getLatestBlockhash();
      const tx = new VersionedTransaction(
        new TransactionMessage({
          payerKey: wallet3.publicKey,
          instructions,
          recentBlockhash: latestBlockhash.blockhash,
        }).compileToV0Message()
      );
      tx.sign([wallet3, authority]);

      const signature = await program.provider.connection.sendTransaction(tx);

      await program.provider.connection.confirmTransaction(
        { signature, ...latestBlockhash },
        "confirmed"
      );

      const finalBalances = await getTokenBalances(connection, accounts);

      assert.equal(
        finalBalances.tokenAccount,
        initialBalances.tokenAccount,
        "Token account balance should not have changed"
      );
      assert.equal(
        finalBalances.destinationAccount,
        initialBalances.destinationAccount + increment,
        "Destination account received the transaction increment"
      );
    });
  });
});
