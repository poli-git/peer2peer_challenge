# Bitcoin crawling

## Step 1: Code review

Here is a simple Rust binary project performing a handshake with a Bitcoin node.

Considering step 2 will consist in using this code to write a Bitcoin network crawler, **perform a code review on this project**. List what you like about it and what you don't. Describe how easy to read and how maintainable this code is. Mention where the code isn't the most idiomatic, ergonomic or reliable. Suggest fixes and adjustements (no need to actually fix these, just suggest the fixes). Treat it like a pull request. 

Write findings in the `step1.md` file

## Step 2: Feature implementation

Please focus on this stage only once you have finished the review, as carefully reading the existing code will give you some hints on completing this step.

**Extend this program with a new feature: network crawling. Make it so this program can collect the addresses of 5000 peers.** 

Be aware that retrieving these addresses may take up to a minute and that some nodes may only give you a single peer address on the first try and a second try might be required. You won't need to validate that all these addresses correspond to responsive nodes, but you'll need to crawl some of those in order to get 5000 peers. We especially recommend you don't even try to connect to ipv6 and Tor addresses.

For the sake of facilitating our review and saving you time and effort, please refrain from fixing all the rough edges you noticed during the review unless you really feel this is necessary, and focus on the crawling feature.

We encourage you to take advantage of the fact this directory is a git repository. You can stay on the main branch.
