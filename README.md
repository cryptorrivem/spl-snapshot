# spl_snapshot

Allow to transfer the balance changes of a System or Token Account to another account.

## Problem

Accepting payments in many coins requires to either accept receiving the tokens and then swapping, or swap the tokens on user's wallet to latter send. This affects the usability as we want to reduce the friction of having multiple transactions to know the exact amount a platform will receive in the token they operate in.

First approach was to do a simulation of the swap in user's wallet to add a regular SPL token transfer, but things like swaps might vary due to market conditions.

To make the exact output (increment) of the swap to be immediately transferred to another wallet the need for this program arised.

This allows to read the source system or token account balance, execute the swap and immediately after transfer that difference to another account. This way, deposits to non-full decentralized apps can benefit from a reduced friction by speeding the time to receive the appropriate token with the exact amount.

## Use cases

- Call any program that creates an increment in balance in a system or token account and move that amount to another account

- Swap `SOL` or any token for `USDC` using Jupiter and transfer the obtained `USDC` to a `USDC` vault token account

- Swap `USDC` or any token for `SOL` and transfer the obtained `SOL` to a system account vault

## Usage

Wrap your transaction with the `snapshot` and `transfer` instructions for system accounts, or `token_snapshot` and `token_transfer` for token accounts.

> The `payer` is specified separately to allow unfunded `owner` accounts to be used. Same `payer` account is passed to the `transfer` instruction to give back the rent from the `snapshot_account` PDA.

System account snapshot & transfer:

```
    const [snapshotAccount] = PublicKey.findProgramAddressSync(
        [Buffer.from("snapshot"), owner.toBuffer()],
        programId
    );

    const instructions = [
        // snapshot
        await program.methods
            .snapshot()
            .accounts({
                snapshotAccount,
                owner,
                payer,
                systemProgram: SystemProgram.programId,
            })
            .instruction(),

          // instructions that perform balance changes over `owner` account
          ...

          // transfer balance change from `owner` to `destination` account
          await program.methods
            .transfer()
            .accounts({
                snapshotAccount,
                destination,
                owner,
                payer,
                systemProgram: SystemProgram.programId,
            })
            .instruction(),
    ];
```

Token account snapshot & transfer:

```
    const [snapshotAccount] = PublicKey.findProgramAddressSync(
        [Buffer.from("snapshot"), tokenAccount.toBuffer()],
        program.programId
    );

    const instructions = [
        // snapshot
        await program.methods
            .tokenSnapshot()
            .accounts({
                snapshotAccount,
                tokenAccount,
                payer,
                systemProgram: SystemProgram.programId,
            })
            .instruction(),

        // instructions that perform balance changes over `owner` account
        ...

        // transfer to destination account
        await program.methods
            .tokenTransfer()
            .accounts({
                snapshotAccount,
                tokenAccount,
                destinationAccount,
                owner,
                payer,
                tokenProgram: TOKEN_PROGRAM_ID,
            })
            .instruction(),
    ];
```
