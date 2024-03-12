# Skill Tracker Module

A skills/Income management system for HR

Tracking employee skills, and paying them according to their real skills set is a real challenge. As a result, promotions are often based on subjective factors, don't take in account all the skills and knowledge
that were not included in the job offer, but were still necessary to complete the job.
The aim of skill tracker is to solve this exact problem, by allowing employees to submit their skill set, and allowing their peers to actually confirm which skills are being used through the use of a _Task MarketPlace_ .

**Two pallets were created for this module**

## Skills Pallet

Manages the addition of new employees, addition of new skills to the database, and addition of _Unverified_Skills_.

## Market Pallet

Manages the addition of a _Working_Task_, addition of a _Curator_ for the task, attribution of _Skill_Points_ and _Experience_Points_ to employees as well as employee wage increase. Verification of _Unverified_Skills_ is also included.

## Roles

The different roles acting in this program are described below:

Alice, Bob and Charlie are members of the HR council:

- They input new employees in the system
- They review the addition of new skills in the skill database by employees
- They review the addition of a new working task to the MarketPlace by employees
- Once a working task is accepted by the council, the council vote on the selection of a curator for the working task

Employees can accomplish the following tasks:

- Submit a new skill to the council, for addition to the skills database
- Add a skill to their profile: this skill will be added the Unverified_Skills category
- Add a working task to the market place, and suggest a curator for the task: Employees completing the task will get Skill Points, and unverified skills related to the task will be moved to the verified category
- As a curator, they review the completion of a working task, and award employees who completed the task

## Interface

### Pallet Skills

- `new_employee` - Adds a new employee to the database, only accessible by members of the council
- `submit_skill` - Employee submits a skill for addition to the skill database.
- `add_my_skills`- Employee adds a database skill to his profile. This skill is unverified at this point.

### Pallet Market

- `propose_task` - Employee creates a new _Working_Task_ that will be completed by his/her skilled peers.
- `propose_curator` - Curator role is proposed to the employee by the Council. the curator/employee is suggested by the task creator during task creation.
- `accept_curator` - Employee accepts Curator role.
- `pick_task` - Employee takes on an available working task
- `curator_rewards_worker`- Curator rewards Employee for completion of a _Working_Task_ in _Skill_points_ and _reward_fees_ .
- `worker_claims_reward` - Employee accepts the reward granted by the curator

### Build, Launch, Front-End

Use the following command to build the node without launching it:

```sh
cargo build --release
```

Next, you can launch the node:

```sh
./target/release/node-template --dev
```

At the moment The Front-End only convers extrinsics of the Skills pallet:

- `submit_skill`
- `add_my_skills`

  They are demonstrated in the following Youtube video: https://youtu.be/KCsrnkx4uj8?si=Jkby6887Umj7z6v6 .
  the tests file in the folder `pallets/market/src/tests.rs` shows an example of successful workflow.
  You can also use Polkadot-JS to replicate this workflow.

For the Front-End, you will need to go under the `FrontEnd` folder, and run `npm install` before following the instructions given in the ReadMe file, located in the same folder.

### Embedded Docs

After you build the project, you can use the following command to explore its parameters and subcommands:

```sh
./target/release/node-template -h
```

You can generate and view the [Rust Docs](https://doc.rust-lang.org/cargo/commands/cargo-doc.html) for this template with this command:

```sh
cargo +nightly doc --open
```

### Single-Node Development Chain

The following command starts a single-node development chain that doesn't persist state:

```sh
./target/release/node-template --dev
```

To purge the development chain's state, run the following command:

```sh
./target/release/node-template purge-chain --dev
```

To start the development chain with detailed logging, run the following command:

```sh
RUST_BACKTRACE=1 ./target/release/node-template -ldebug --dev
```

Development chains:

- Maintain state in a `tmp` folder while the node is running.
- Use the **Alice** and **Bob** accounts as default validator authorities.
- Use the **Alice** account as the default `sudo` account.
- Are preconfigured with a genesis state (`/node/src/chain_spec.rs`) that includes several prefunded development accounts.

To persist chain state between runs, specify a base path by running a command similar to the following:

```sh
// Create a folder to use as the db base path
$ mkdir my-chain-state

// Use of that folder to store the chain state
$ ./target/release/node-template --dev --base-path ./my-chain-state/

// Check the folder structure created inside the base path after running the chain
$ ls ./my-chain-state
chains
$ ls ./my-chain-state/chains/
dev
$ ls ./my-chain-state/chains/dev
db keystore network
```

### Connect with Polkadot-JS Apps Front-End

After you start the node template locally, you can interact with it using the hosted version of the [Polkadot/Substrate Portal](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944) front-end by connecting to the local node endpoint.
A hosted version is also available on [IPFS (redirect) here](https://dotapps.io/) or [IPNS (direct) here](ipns://dotapps.io/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer).
You can also find the source code and instructions for hosting your own instance on the [polkadot-js/apps](https://github.com/polkadot-js/apps) repository.

### Polkadot-JS calls

The series of calls below can be used to replicate a successful worflow:

- Alice_stash makes a donation
  `0x10000c0403001cbd2d43530a44705ad088af313e18f80b53ef16b36177cd4b77b846f2a5f07c0f0080a1a76b4a35040300e659a7a1628cdd93febc04a4e0646ea20e9f5f0ce097d9a05290d4a9e054df4e0f0000434fd7946a040300306721211d5404bd9da88e0204360a1a9ab8b87c66c1bc2fcdd37f3c2222cc200f008062175ed158`

- Funds sent to Treasury:
  `0x07000408006d6f646c70792f747273727900000000000000000000000000000000000000001f000000677f9aa7c24eca05`

Part 1

- Alice create 3 employees: Eve, Ferdie, and Bob_Stash
  `0x10000c08051cbd2d43530a44705ad088af313e18f80b53ef16b36177cd4b77b846f2a5f07c2c46657264696520426c75650805e659a7a1628cdd93febc04a4e0646ea20e9f5f0ce097d9a05290d4a9e054df4e24457665204576616e730805fe65717dad0447d715f660a0a58411de509b42e6efb8375f562f58a554d5860e2c426f6262792042726f776e`

- Ferdie submit a new skill: "Rust Programming"
  `0x080240527573742050726f6772616d6d696e670102`

- Alice vote for Ferdie
  `0x10000408031cbd2d43530a44705ad088af313e18f80b53ef16b36177cd4b77b846f2a5f07c01`

- Bob vote for Ferdie and close the session
  `0x10000808031cbd2d43530a44705ad088af313e18f80b53ef16b36177cd4b77b846f2a5f07c0108041cbd2d43530a44705ad088af313e18f80b53ef16b36177cd4b77b846f2a5f07c`

- Ferdie add an unverified skill to her profile
  `0x080600000000`

Part 2

- Eve submit a task proposal, suggest Bob_Stash as Curator
  `0x0903000000000070c9b28b00000000000000000000001074657374fe65717dad0447d715f660a0a58411de509b42e6efb8375f562f58a554d5860e`

- Alice vote for Eve proposal
  `0x0906e659a7a1628cdd93febc04a4e0646ea20e9f5f0ce097d9a05290d4a9e054df4e0100`

- Bob vote for Eve proposal
  `0x1000080906e659a7a1628cdd93febc04a4e0646ea20e9f5f0ce097d9a05290d4a9e054df4e01000907e659a7a1628cdd93febc04a4e0646ea20e9f5f0ce097d9a05290d4a9e054df4e00`

We need "Funded" status for the proposal at this point:
Go to the Governance tab-->boounties --> takes 1_min

- Council(Alice) propose the curator suggested by Eve for review to the council
  `0x0904e659a7a1628cdd93febc04a4e0646ea20e9f5f0ce097d9a05290d4a9e054df4e`

- Curator (Bob_Stash) accepts role => Coming soon, available in tests
  `0x0905e659a7a1628cdd93febc04a4e0646ea20e9f5f0ce097d9a05290d4a9e054df4e`

- Worker (Ferdie) pick the task
  `0x0908e659a7a1628cdd93febc04a4e0646ea20e9f5f0ce097d9a05290d4a9e054df4e`

- After task completion (Confirmed off-chain), Curator (Bob-Stash) rewards Worker (Ferdie)
  `0x090ae659a7a1628cdd93febc04a4e0646ea20e9f5f0ce097d9a05290d4a9e054df4e1cbd2d43530a44705ad088af313e18f80b53ef16b36177cd4b77b846f2a5f07c`

- Worker (Ferdie) claims reward
  `0x0909e659a7a1628cdd93febc04a4e0646ea20e9f5f0ce097d9a05290d4a9e054df4e`

If you you the Front-End running, you can actually see the employee worker profile being updated, in particular the number of _SP_ points and the _Unverified_Skill_ changing to _Verified_Skill_ .

### Multi-Node Local Testnet

If you want to see the multi-node consensus algorithm in action, see [Simulate a network](https://docs.substrate.io/tutorials/build-a-blockchain/simulate-network/).

## Template Structure

A Substrate project such as this consists of a number of components that are spread across a few directories.

### Node

A blockchain node is an application that allows users to participate in a blockchain network.
Substrate-based blockchain nodes expose a number of capabilities:

- Networking: Substrate nodes use the [`libp2p`](https://libp2p.io/) networking stack to allow the
  nodes in the network to communicate with one another.
- Consensus: Blockchains must have a way to come to [consensus](https://docs.substrate.io/fundamentals/consensus/) on the state of the network.
  Substrate makes it possible to supply custom consensus engines and also ships with several consensus mechanisms that have been built on top of [Web3 Foundation research](https://research.web3.foundation/en/latest/polkadot/NPoS/index.html).
- RPC Server: A remote procedure call (RPC) server is used to interact with Substrate nodes.

There are several files in the `node` directory.
Take special note of the following:

- [`chain_spec.rs`](./node/src/chain_spec.rs): A [chain specification](https://docs.substrate.io/build/chain-spec/) is a source code file that defines a Substrate chain's initial (genesis) state.
  Chain specifications are useful for development and testing, and critical when architecting the launch of a production chain.
  Take note of the `development_config` and `testnet_genesis` functions,.
  These functions are used to define the genesis state for the local development chain configuration.
  These functions identify some [well-known accounts](https://docs.substrate.io/reference/command-line-tools/subkey/) and use them to configure the blockchain's initial state.
- [`service.rs`](./node/src/service.rs): This file defines the node implementation.
  Take note of the libraries that this file imports and the names of the functions it invokes.
  In particular, there are references to consensus-related topics, such as the [block finalization and forks](https://docs.substrate.io/fundamentals/consensus/#finalization-and-forks) and other [consensus mechanisms](https://docs.substrate.io/fundamentals/consensus/#default-consensus-models) such as Aura for block authoring and GRANDPA for finality.

### Runtime

In Substrate, the terms "runtime" and "state transition function" are analogous.
Both terms refer to the core logic of the blockchain that is responsible for validating blocks and executing the state changes they define.
The Substrate project in this repository uses [FRAME](https://docs.substrate.io/learn/runtime-development/#frame) to construct a blockchain runtime.
FRAME allows runtime developers to declare domain-specific logic in modules called "pallets".
At the heart of FRAME is a helpful [macro language](https://docs.substrate.io/reference/frame-macros/) that makes it easy to create pallets and flexibly compose them to create blockchains that can address [a variety of needs](https://substrate.io/ecosystem/projects/).

Review the [FRAME runtime implementation](./runtime/src/lib.rs) included in this template and note the following:

- This file configures several pallets to include in the runtime.
  Each pallet configuration is defined by a code block that begins with `impl $PALLET_NAME::Config for Runtime`.
- The pallets are composed into a single runtime by way of the [`construct_runtime!`](https://paritytech.github.io/substrate/master/frame_support/macro.construct_runtime.html) macro, which is part of the [core FRAME pallet library](https://docs.substrate.io/reference/frame-pallets/#system-pallets).

### Pallets

The runtime in this project is constructed using many FRAME pallets that ship with [the Substrate repository](https://github.com/paritytech/substrate/tree/master/frame) and a template pallet that is [defined in the `pallets`](./pallets/template/src/lib.rs) directory.

A FRAME pallet is comprised of a number of blockchain primitives, including:

- Storage: FRAME defines a rich set of powerful [storage abstractions](https://docs.substrate.io/build/runtime-storage/) that makes it easy to use Substrate's efficient key-value database to manage the evolving state of a blockchain.
- Dispatchables: FRAME pallets define special types of functions that can be invoked (dispatched) from outside of the runtime in order to update its state.
- Events: Substrate uses [events](https://docs.substrate.io/build/events-and-errors/) to notify users of significant state changes.
- Errors: When a dispatchable fails, it returns an error.

Each pallet has its own `Config` trait which serves as a configuration interface to generically define the types and parameters it depends on.

## Alternatives Installations

Instead of installing dependencies and building this source directly, consider the following alternatives.

### Nix

Install [nix](https://nixos.org/) and
[nix-direnv](https://github.com/nix-community/nix-direnv) for a fully plug-and-play
experience for setting up the development environment.
To get all the correct dependencies, activate direnv `direnv allow`.

### Docker

Please follow the [Substrate Docker instructions here](https://github.com/paritytech/substrate/blob/master/docker/README.md) to build the Docker container with the Substrate Node Template binary.
