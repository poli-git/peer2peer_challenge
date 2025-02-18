
There are several alternatives (or combination of them) that can be implemented to discover peers:

### mDNS

It allows peers to discover each other when they are on the same local network without any configuration. It is obviously the simplest discovery mode, but limited to local networks. 

### Rendezvous

It's goal is to provide a lightweight mechanism for generalized peer discovery. As its name indicates, it requires that there be nodes that act as rendezvous. In the protocol implementation examples you can see it better.

### Kademlia

This is the best option in the context of a network with many nodes, where a portion of these nodes may offer limited connectivity. It's a distributed hash table for decentralized peer-to-peer computer networks.