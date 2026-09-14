# Original request

> ok lets try raft first, try the skill put it in examples/ dir in the same dir as the skill

Context: a completely standalone, general Rust system, separate from blockchain and Tempo. Build Raft consensus from scratch with a key-value client API. Exercise the UML-first workflow, including an independent Antithesis research pass before implementation, then a separate fresh agent to write tests.

## Inferred scope for this first example

Implement core Raft rather than wrapping an existing consensus library. Static membership (3 or 5 nodes, also support 1 for tests), elections, AppendEntries replication, durable term/vote/log, restart recovery, and a usable HTTP JSON API. Reads go through the replicated log for simple linearizable semantics. No snapshots or membership changes in this iteration. This is an educational but runnable service, not production certification. Treat peers as trusted and crash-faulting; no Byzantine tolerance. Use only local development addresses by default.

Reference: https://raft.github.io/raft.pdf (original extended Raft paper). The reference was proposed by the assistant; additional user references were requested asynchronously. Do not record an unanswered scope question as user confirmation.
