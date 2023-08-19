# <img src="https://cryptologos.cc/logos/solana-sol-logo.png?v=025" with="25" height="25"> Token Trading Platform with On-chain Dividend System

This is the foundational smart contract of the [token trading platform](https://s3t-trade.vercel.app/) that I have developed to showcase how tokens can function as stocks. In this system, shareholders (those who hold the platform's tokens) can retrieve their portion of company profits, which are transparently recorded on the Solana blockchain at predefined intervals.

This on-chain program was deployed on devnet at: [S3T2JExjrp6a48LNcxNxBHKa1GTB3bXRmwH3wepB5KQ](https://solscan.io/account/S3T2JExjrp6a48LNcxNxBHKa1GTB3bXRmwH3wepB5KQ?cluster=devnet)

# This on-chain program was divided into two parts.

## Token Trading
Acting as a main product of this token trading platform which is tokens trading

### Token Listing

![image](https://github.com/KKQanT/ft-trading/assets/71139706/1f035c2c-ee7a-445b-90f5-9701c9e87863)


When listing your tokens for sale, they will be transferred to the token account belonging to the escrow account created by the on-chain program, which will embed details such as the preferred price per token, the amount of tokens, and the owner. The tokens can be moved out of this account on two occasions:

You call the program to close the escrow account (only you have the authority to do this) and retrieve tokens along with the amount of Sol you paid to create this account.
Someone purchases the token via this on-chain program.

### Tokens Purchasing

![image](https://github.com/KKQanT/ft-trading/assets/71139706/0d2ce665-c1ba-4511-8d35-998e60d57a70)

In a single transaction, when purchasing tokens, an amount of Sol equivalent to the token price will be deducted from your wallet and transferred directly to the seller. Concurrently, the seller's escrow will automatically send the requested amount of tokens to your token account. Within this same transaction, the platform will impose a 10% fee, which will be transferred to the platform's dividend storage account. This fee will subsequently be distributed among the platform's shareholders.

## Distribute platform profit to shareholders

### Claim Share Score

![image](https://github.com/KKQanT/ft-trading/assets/71139706/5aa0945c-df73-408c-afe3-4d91c71eacd7)

If you are a shareholder of our platform (holding our NFTs), you can verify your NFT to claim a percentage share of our income at each time frame (currently set to 1 day), referred to as an "epoch." By verifying your NFTs on-chain, the on-chain program will create an account called "user-share-account" to store shared data, which will later be used to claim dividends in Sol after the end of each epoch.

Note:

During each epoch, when you verify your token, you will be asked for a small amount of Sol. This amount is used to pay Solana for storing data on-chain, and it will be returned to you when you claim your reward after the epoch ends.
The on-chain program includes an account called "WhitelistedToken," which stores the recent epoch of the corresponding NFT used for claiming. This mechanism prevents a single NFT from being claimed multiple times within the same epoch.

### Use share score to claim profit

![image](https://github.com/KKQanT/ft-trading/assets/71139706/8b070353-d414-4ea9-90f9-8c157e00f92d)

Each epoch will have a corresponding account to store the "total_n_share," which denotes the total number of platform NFTs used for verification during that epoch. Each shareholder will have their own "UserShareAccount" to store a value called "n_share," which indicates the number of times they have used NFTs for verification in each epoch. For instance, if you hold just one NFT from our company and verify the NFT via our on-chain program during an epoch when there were 20 other NFTs verified by other holders, you would be considered a 5% (1/20) shareholder of the platform for that epoch. Consequently, you can then claim 5% of the platform's income.
