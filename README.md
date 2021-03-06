# friends-list

This app was initialized with [create-near-app]

A [smart contract] written in [Rust] for an app initialized with [create-near-app]

#Steps:

1. npx create-near-app --contract=rust my-project
2. check Cargo.toml file
3. add contract code
4. run tests: cargo test -- --nocapture
5. near login
6. to see keys: near keys friend.testnet
7. Build SmartContract: cargo build --target wasm32-unknown-unknown --release
https://www.near-sdk.io/building/basic-build
8. optional mimification: https://www.near-sdk.io/building/post-processing-tools
9. create subaccount: near create-account friends.abong.testnet --masterAccount abong.testnet
near delete crossword.abong.testnet abong.testnet
near state friends.abong.testnet  Dev Acct: near state dev-1642982642042-83128772883131
10. Deploy: use yarn dev to deploy contract to testnet. see below
11. call contract methods: 
view only: 
near view dev-1642982642042-83128772883131 get_friend '{\"friend_id\": \"azinwi.testnet\"}' --accountId abong.testnet
            
near view dev-1642982642042-83128772883131 get_friends '{\"from_index\": 0, \"limit\": 5}' --accountId abong.testnet

 near call dev-1642982642042-83128772883131 add_friend_request '{\"friend_id\": \"azinwi.testnet\"}' --accountId abong.testnet






# Quick Start

To run this project locally:

1. Prerequisites: Make sure you've installed [Node.js] ≥ 12
2. Install dependencies: `yarn install`
3. Run the local development server: `yarn dev` (see `package.json` for a
   full list of `scripts` you can run with `yarn`)

Now you'll have a local development environment backed by the NEAR TestNet!

Go ahead and play with the app and the code. As you make code changes, the app will automatically reload.

# Exploring The Code

1. The "backend" code lives in the `/contract` folder. See the README there for
   more info.
2. The frontend code lives in the `/src` folder. `/src/index.html` is a great
   place to start exploring. Note that it loads in `/src/index.js`, where you
   can learn how the frontend connects to the NEAR blockchain.
3. Tests: there are different kinds of tests for the frontend and the smart
   contract. See `contract/README` for info about how it's tested. The frontend
   code gets tested with [jest]. You can run both of these at once with `yarn run test`.

# Deploy

Every smart contract in NEAR has its [own associated account][near accounts]. When you run `yarn dev`, your smart contract gets deployed to the live NEAR TestNet with a throwaway account. When you're ready to make it permanent, here's how.

## Step 0: Install near-cli (optional)

[near-cli] is a command line interface (CLI) for interacting with the NEAR blockchain. It was installed to the local `node_modules` folder when you ran `yarn install`, but for best ergonomics you may want to install it globally:

    yarn install --global near-cli

Or, if you'd rather use the locally-installed version, you can prefix all `near` commands with `npx`

Ensure that it's installed with `near --version` (or `npx near --version`)

## Step 1: Create an account for the contract

Each account on NEAR can have at most one contract deployed to it. If you've already created an account such as `your-name.testnet`, you can deploy your contract to `friends-list.your-name.testnet`. Assuming you've already created an account on [NEAR Wallet], here's how to create `friends-list.your-name.testnet`:

1. Authorize NEAR CLI, following the commands it gives you:

   near login

2. Create a subaccount (replace `YOUR-NAME` below with your actual account name):

   near create-account friends-list.YOUR-NAME.testnet --masterAccount YOUR-NAME.testnet

## Step 2: set contract name in code

Modify the line in `src/config.js` that sets the account name of the contract. Set it to the account id you used above.

    const CONTRACT_NAME = process.env.CONTRACT_NAME || 'friends-list.YOUR-NAME.testnet'

## Step 3: deploy!

One command:

    yarn deploy

As you can see in `package.json`, this does two things:

1. builds & deploys smart contract to NEAR TestNet
2. builds & deploys frontend code to GitHub using [gh-pages]. This will only work if the project already has a repository set up on GitHub. Feel free to modify the `deploy` script in `package.json` to deploy elsewhere.

# Troubleshooting

On Windows, if you're seeing an error containing `EPERM` it may be related to spaces in your path. Please see [this issue](https://github.com/zkat/npx/issues/209) for more details.

[create-near-app]: https://github.com/near/create-near-app
[node.js]: https://nodejs.org/en/download/package-manager/
[jest]: https://jestjs.io/
[near accounts]: https://docs.near.org/docs/concepts/account
[near wallet]: https://wallet.testnet.near.org/
[near-cli]: https://github.com/near/near-cli
[gh-pages]: https://github.com/tschaub/gh-pages
