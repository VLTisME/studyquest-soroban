# StudyQuest

## Description

StudyQuest is a proof-of-learning smart contract built on Stellar Soroban.

The project allows each wallet to create a personal learning quest, update its progress, mark it as completed, and query the stored quest state on-chain. I built this project because many beginner smart contract examples are usually limited to hello-world or counter contracts. StudyQuest is still simple enough to understand and deploy, but it demonstrates a more practical use case with persistent storage, wallet authentication, and multiple contract interactions.

Each quest contains a title, a target progress value, the current progress, and a completion status. For example, a user can create a quest named `"Finish Stellar workshop"` with a target of `100`, then update the progress as they complete the learning task.

## Features

- Create a learning quest for a wallet.
- Store quest data using Soroban persistent storage.
- Update quest progress.
- Automatically mark a quest as completed when progress reaches the target.
- Manually mark a quest as completed.
- Query the current quest state.
- Check whether a wallet already has a quest.
- Require wallet authentication for write operations using `Address.require_auth()`.

## Contract

Network: Stellar Testnet

Contract ID:

```text
CDLX4MDSCECOWGRUGPNUMQ5X4SYAZEHDY7V4NEEICBW5LFZGASDMIJI5
```

Contract link:

```text
https://stellar.expert/explorer/testnet/contract/CDLX4MDSCECOWGRUGPNUMQ5X4SYAZEHDY7V4NEEICBW5LFZGASDMIJI5
```

Deploy transaction:

```text
https://stellar.expert/explorer/testnet/tx/b0ad07366cb884a3b0983bba402f3ab0b5922b1af0fe853ffeebefa2a3b416df
```

Successful interaction transactions:

```text
https://stellar.expert/explorer/testnet/tx/0ac9095a6c6f461c17a5f39b658d8a87cbb362fdededc3416da5c38b86d59992
```

```text
https://stellar.expert/explorer/testnet/tx/db980872d4cfcfa93074cbcd9b4b20fe333bf3917cac9f9a37be7fb5d9378710
```

Contract screenshot:

![Contract screenshot](./contract-detail.png)

## Example Interaction

Source account:

```text
GARBNF7QP7SLTAYEWZMGSL42ENIF6QQVBQFGCFNS4IVFXJPAGCWL3NM5
```

Create a quest:

```bash
stellar contract invoke --id CDLX4MDSCECOWGRUGPNUMQ5X4SYAZEHDY7V4NEEICBW5LFZGASDMIJI5 --source-account workshop --network testnet -- create_quest --user GARBNF7QP7SLTAYEWZMGSL42ENIF6QQVBQFGCFNS4IVFXJPAGCWL3NM5 --title "Finish Stellar workshop" --target 100
```

Result:

```json
{"completed":false,"progress":0,"target":100,"title":"Finish Stellar workshop"}
```

Add progress:

```bash
stellar contract invoke --id CDLX4MDSCECOWGRUGPNUMQ5X4SYAZEHDY7V4NEEICBW5LFZGASDMIJI5 --source-account workshop --network testnet -- add_progress --user GARBNF7QP7SLTAYEWZMGSL42ENIF6QQVBQFGCFNS4IVFXJPAGCWL3NM5 --amount 40
```

Result:

```json
{"completed":false,"progress":40,"target":100,"title":"Finish Stellar workshop"}
```

Get quest:

```bash
stellar contract invoke --id CDLX4MDSCECOWGRUGPNUMQ5X4SYAZEHDY7V4NEEICBW5LFZGASDMIJI5 --source-account workshop --network testnet -- get_quest --user GARBNF7QP7SLTAYEWZMGSL42ENIF6QQVBQFGCFNS4IVFXJPAGCWL3NM5
```

Result:

```json
{"completed":false,"progress":40,"target":100,"title":"Finish Stellar workshop"}
```

## Future Scopes

In the future, StudyQuest can be expanded into a more complete on-chain learning achievement system. Possible improvements include supporting multiple quests per wallet, adding deadlines, assigning difficulty levels, issuing badges after completion, and allowing communities or bootcamps to verify participants' learning progress.

Another possible direction is to integrate this contract with a frontend application. Users could connect their wallet, create learning goals, update progress visually, and share their completed quests as public proof of learning.

## Profile

Name: Vo Lan Tuan

Skills:

- Rust smart contract development
- Stellar Soroban
- Blockchain basics
- Backend development
- AI and machine learning