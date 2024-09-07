# Test Passed on Devnet

![alt text](image.png)

```
ubuntu@LAPTOP-SJGCH9GK:~/solana-starter/nft-staking$ anchor test --provider.cluster devnet
    Finished release [optimized] target(s) in 0.41s
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.66s
     Running unittests src/lib.rs (/home/ubuntu/solana-starter/nft-staking/target/debug/deps/nft_staking-915183992e0f40ad)
Deploying cluster: https://api.devnet.solana.com
Upgrade authority: /home/ubuntu/.config/solana/id.json
Deploying program "nft_staking"...
Program path: /home/ubuntu/solana-starter/nft-staking/target/deploy/nft_staking.so...
Program Id: 7yfqb2eUrxTmtHcCBrbXrjKfCGebhFohbkvst94KuCx2

Deploy success

Found a 'test' script in the Anchor.toml. Running it as a test suite!

Running test suite: "/home/ubuntu/solana-starter/nft-staking/Anchor.toml"

yarn run v1.22.19
$ /home/ubuntu/solana-starter/nft-staking/node_modules/.bin/ts-mocha -p ./tsconfig.json -t 1000000 'tests/**/*.ts'


  nft-staking
Created Collection NFT: 3CBQAshKis7DjRGNfBWP4bNxKtFmyy822CM62t1zHsGp
    ✔ Mint Collection NFT (2127ms)

Created NFT: BJBymV8MYLrBGwBa6xTypuzqpQpX2NxsNojZ1cAAcadW
    ✔ Mint NFT (2393ms)

Collection NFT Verified!
    ✔ Verify Collection NFT (1436ms)

Config Account Initialized!
Your transaction signature XYafP2zjHftwQt3MxvyjZ6wCsfi252iG6FnBspZjR7TUwTGgX7BJ5hiLQ3am2xFEWrFPvEFaLr62g7tejPeP31y
    ✔ Initialize Config Account (749ms)

User Account Initialized!
Your transaction signature 5GHmXEuL7vTfDmYUGySnyz8rVNwac11Ev4djMxuPQuDuiQNh2wCw1jCpHFoxx88hCRphpE5or9jePkmPrV9M5794
    ✔ Initialize User Account (2337ms)

NFT Staked!
Your transaction signature 2j5tFWyF3yPAsUgnnyCoXfMfGb6o7X7d83rWcHJPvfWQJXSaZu3pJZPJN4cKQL42Jef8dtiHrHBfjLrPEMDf4bZk
    ✔ Stake NFT (795ms)

NFT unstaked!
Your transaction signature 3Zvpf3rJzSy9EWUieejxiwctWNsWcKR3TPXRSEkSCsiALU16iPMWKf789jVCfMREE2tQs4Ymk1JbAoiD8VMYFF55
user points:  0
    ✔ Unstake NFT (7375ms)

Rewards claimed
Your transaction signature hAAZYsh52HZRs6N21PsRjbuUF7VF8GxgkGqwL7R8qdJ6NGrKW84YA4XKHWTf9XUgwK5Ar8rqkada7ZnVaL1YnSD
user points:  0
    ✔ Claim Rewards (1875ms)


  8 passing (19s)

Done in 20.84s.
```
