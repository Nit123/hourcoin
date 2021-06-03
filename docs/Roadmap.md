# Roadmap
### Introduction
As with any "large-scale" software development project, it's crucial to have a roadmap to plan out a course and provide general deadlines regarding new features and updates to the project. 

This document serves as the roadmap for Hourcoin, with the hopes that by the end of Summer 2021, Hourcoin will, at the very least, be a functioning *distributed* (**but not decentralized**) blockchain that utilizes "proof of time" as its consensus protocol where miners submit their blocks to a validator/time-keeper node who uses the timestamp of the submission to determine whether it is accepted and accepted blocks require miners to not mine for an hour afterwards to receive the rewards of mining.

Questions as to the rationale of hourcoin and its "proof of time" protocol will be answered in the whitepaper which is upcoming. This document simply serves as a way to gauge progress towards goals within the project's development.

### Basic Blockchain
The first step is to create a basic blockchain that hopefully accepts miners from different computers and works in essence like [tinycoin](https://github.com/JeremyRubin/tinycoin) but in Rust. This is in part because I want to learn Rust and I feel like in terms of organizing distrbuted computing, Rust is one of the best languages around.

That being said, hourcoin's basic structure should be better than tinycoin and attempt to be sustain general usage (comapred to tinycoin's pure educational usage goal) and this might require more researching about transaction models and consensus checks.

**Things to Do** (points in *italics* have been completed)
- *Create very basic blockchain in Rust using GeekLaunch's [youtube tutorial](https://www.youtube.com/playlist?list=PLwnSaD6BDfXL0RiKT_5nOIdxTxZWpPtAv) as a basis*
- Add elements of tinycoin and improve aspects not included by GeekLaunch
   - *Validate difficulty of block rather than just accepting it as given (blocks no longer store difficulty since the chain will initially be centralized but the difficulty is now verified)*
   - *Validate coinbase value for transaction (make it 2 hourcoin - one for taking an hour to accept transactions and another for waiving mining rights for another hour)*
   - *"Two otherwise identical outputs from different transactions are indistinguishable" -> most likely means adding timestamps to transaction hashes (Assuming this is in fact including transaction timestamps and hashing them in a Merkle-tree fashion, this is correct but might need to make sure about this point later)* 
   - *Do some verification of transaction timestamps...somehow (checks that every output of a transaction occurs after every input)*
   - Determine a way to broadcast transactions.
   - Seperate mining into a client script to be used (i.e. that way anyone can test mining on their own)
   - Create testing scripts to ensure that any changes to the block/mining code will still allow for genesis block to be accepted
   - Create node code that allows for networking to be used to verify/mining (specifically [node.py](https://github.com/JeremyRubin/tinycoin/blob/master/node.py)-like program)
   - Make sure only certified people can change blockchain difficulty (might occur during next phase)

The goal is for these things to be done by the end of June or at the latest July 15th. However, it's very hard to gauge deadlines as being reasonable without getting into the thick of it so might have to reevaluate later.

### Just Add Time
At this point, you'll will have a "functional" (toy) blockchain that uses traditional PoW as its consensus and mining paradigm. Let's not start saying this will be a Bitcoin killer or even as good as Ethereum. It's not. There aren't even wallets or an address system or...just general robustness. Luckily, that's not the point of hourcoin so those things don't necessarily matter but a crucial thing is that there's no "proof of time"! *So, time to add it*!

**Things to do**
- Create a soft fork (probably just another git branch entirely assuming the basic blockchain works standalone)
- Begin creating validators/time-keeper node software
   - This will require researching time protocols (such as NTP and Ouroboros Chronos) to determine what is the best course of action. Since (at this point),you will want to keep the validation process centralized, you will probably use Cloudflare servers to serve as the "true time".
   - Make validators/miners record the time in which a miner attempts to add a block to the chain, validate that the time is correct within a tolerance amount (such as the latency to interact to the Cloudflare server) and then implement a tonce (time only used once) divisor. 
      - That is, take the timestamp when the previous block was accepted, hash that value, take the least significant 5-bits and for the first 60 seconds prior to accepting any new blocks, only accept the block that has a (verifiable) timestamp hash that is divisible by this number in order to challenge miners to complete a time puzzle. This tonce ensures randomized difficulty for miners during the first 60 seconds of a new hour.
      - After 60 seconds, if no block/miner has been accepted, then reduce the tonce to 1 so that it becomes a race to interact with the validator and be accepted. 
      - In order to reduce network congestion, consider blocking IPs/fingerprinting servers that have already attempted to mine during the current hour. However, given that there will be a low amount of miners initially, encourage spoofing or other techniques that simply slow miners down partially but do not prevent them from attempting to mine again.
    - Implement the miner sacrifice protocol: if a miner's candidate block is accepted to the chain, the miner agrees not to mine for the next hour in order to preserve the value of the hourcoin (which represents an hour of the miner's time). 
       - However, because the honor system won't work, especially if the system encourages spoofing initially, force the miner to be in a network session with the validator node for an hour and if at any point, the miner disconnects (even for innocuously reasons like bad connection), the block is rejected by the validator and the acceptance window is reopened.
    - Attempt some level of security but remember the central *security model* behind hourcoin: miners can engage in malicious activity but it should be almost too annoying to do so and the rewards should never be worth it...even at the cost of making hourcoin rewards itself not worth it.
 - Determine how much $$ it takes to keep the testnet running. Remember...you have Azure and AWS credits! 

At this point, you have the basic testnet of hourcoin. If you complete this by the end of Summer 2021, pat yourself on the back and rush to tell people that you made a unique cryptocurrency...that kinda works.

### Beyond Time
Let's say somehow you've enjoyed this process so much, you want to do more. Or you somehow convinced the open sourced community to work on this! In which case, this part of the roadmap are for further steps to actually make hourcoin...legit.

Since (if you can't tell), I'm not very optimisitic that you'll work on this as hardcore as you have been at this point, these goals will be a bit more vague and lofty. However, if I do decide to come back to it, I'll make sure to add more details to each of the goals.

**Things You Can Do**
- Implement wallet addressing and ways to check wallet balance.
- As a result, you will probably need to build a chain explorer or some kind. First as a server/client product then as a website type thing.
- Actually try to be secure...I guess?

### Final thoughts
Thanks to Ansh Shah who helped develop this idea with me. Note that there will be grammatical errors in this roadmap (particularly the use of we, you, and I interchangeably). The idea is I have no idea what we are doing so you'll have to excuse any grammatical errors as I try to figure things out.





  

