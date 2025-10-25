use std::collections::BTreeMap;

use namada_tx_prelude::*;
use namada_tx_prelude_01502 as namada_tx_prelude;
use parameters_storage::get_gas_cost_key;

pub type ChannelId = &'static str;
pub type BaseToken = &'static str;
pub type Precision = u64;

pub type MintTokenLimit = token::Amount;
pub type ThroughtputTokenLimit = token::Amount;
pub type Gas = token::Amount;
pub type MinimumGasPrice = Option<Gas>;
pub type MayBeIncentivized = bool;

const OSMO_CHANNEL_ID: &str = "channel-29";
const MINT_LIMIT: u64 = 1_000_000;
const THROUGHPUT_LIMIT: u64 = 1_000_000;
const MINIMUM_GAS_PRICE: u64 = 1;

const OSMO_TOKENS: [(BaseToken, Precision, MinimumGasPrice); 117] = [
    (
        "uosmo", // OSMO
        6,
        Some(Gas::from_u64(MINIMUM_GAS_PRICE)),
    ),
    (
        "factory/osmo1k6c8jln7ejuqwtqmay3yvzrg3kueaczl96pk067ldg8u835w0yhsw27twm/alloyed/allETH", // Ethereum (alloyed)
        18,
        Some(Gas::from_u64(MINIMUM_GAS_PRICE)),
    ),
    ("transfer/channel-208/wbtc-satoshi", 8,None), // Wrapped Bitcoin (Ethereum via Axelar)
    ("transfer/channel-208/uusdt", 6,None), // Tether USD (Ethereum via Axelar)
    ("transfer/channel-208/dai-wei", 18,None), // Dai Stablecoin
    ("transfer/channel-208/busd-wei", 18,None), // Binance USD
    ("transfer/channel-5/basecro", 8,None), // Cronos POS Chain
    ("transfer/channel-208/wbnb-wei", 18,None), // Binance Coin (Axelar)
    ("transfer/channel-208/wmatic-wei", 18,None), // Polygon (ex-MATIC) (Axelar)
    ("transfer/channel-208/wavax-wei", 18,None), // Avalanche
    ("transfer/channel-72/uluna", 6,None), // Terra Classic
    ("transfer/channel-208/dot-planck", 10,None), // Polkadot (Moonbeam via Axelar)
    ("transfer/channel-88/uscrt", 6,None), // Secret Network
    ("transfer/channel-72/uusd", 6,None), // TerraClassicUSD
    ("transfer/channel-75/ustars", 6,None), // Stargaze
    ("transfer/channel-113/uhuahua", 6,None), // Chihuahua
    ("transfer/channel-4/uxprt", 6,None), // Persistence
    ("transfer/channel-1/uakt", 6,None), // Akash
    ("transfer/channel-2/udvpn", 6,None), // Sentinel
    ("transfer/channel-95/boot", 0,None), // bostrom
    ("transfer/channel-165/usomm", 6,None), // Sommelier
    ("transfer/channel-229/afet", 18,None), // Fetch.ai (Fetch.ai)
    ("transfer/channel-122/inj", 18,None), // Injective
    ("transfer/channel-222/nhash", 9,None), // Provenance
    ("transfer/channel-251/uluna", 6,None), // Terra
    ("transfer/channel-208/link-wei", 18,None), // Chainlink (Axelar)
    ("transfer/channel-208/aave-wei", 18,None), // Aave
    ("transfer/channel-208/shib-wei", 18,None), // Shiba Inu (Axelar)
    ("transfer/channel-326/stuatom", 6,None), // Stride Staked ATOM
    ("transfer/channel-208/uaxl", 6,None), // Axelar
    ("transfer/channel-326/stuosmo", 6,None), // Stride Staked OSMO
    ("transfer/channel-557/umars", 6,None), // Mars Protocol (Mars Hub)
    ("transfer/channel-208/cbeth-wei", 18,None), // Coinbase Wrapped Staked ETH
    ("transfer/channel-208/reth-wei", 18,None), // Rocket Pool Ether
    ("transfer/channel-208/wsteth-wei", 18,None), // Wrapped Lido Staked Ether (Axelar)
    ("transfer/channel-874/untrn", 6,None), // Neutron
    ("transfer/channel-2186/factory/wormhole14ejqjyq8um4p3xfqj74yld5waqljf88fz25yxnma0cngspxe3les00fpjx/8sYgCzLRJC3J7qPn2bNbx6PiGcarhyx8rBhVaNnfvHCA", 8,None), // Solana (Wormhole)
    ("transfer/channel-2186/factory/wormhole14ejqjyq8um4p3xfqj74yld5waqljf88fz25yxnma0cngspxe3les00fpjx/95mnwzvJZJ3fKz77xfGN2nR5to9pZmH8YNvaxgLgw5AR", 5,None), // Bonk
    ("transfer/channel-2186/factory/wormhole14ejqjyq8um4p3xfqj74yld5waqljf88fz25yxnma0cngspxe3les00fpjx/8iuAc6DSeLvi2JDUtwJxLytsZT8R19itXebZsNReLLNi", 6,None), // Tether USD (Ethereum via Wormhole)
    ("transfer/channel-2186/factory/wormhole14ejqjyq8um4p3xfqj74yld5waqljf88fz25yxnma0cngspxe3les00fpjx/46YEtoSN1AcwgGSRoWruoS6bnVh8XpMp5aQTpKohCJYh", 8,None), // Sui (Wormhole)
    ("transfer/channel-2186/factory/wormhole14ejqjyq8um4p3xfqj74yld5waqljf88fz25yxnma0cngspxe3les00fpjx/5wS2fGojbL9RhGEAeQBdkHPUAciYDxjDTMYvdf9aDn2r", 8,None), // Aptos Coin (Wormhole)
    ("transfer/channel-2186/factory/wormhole14ejqjyq8um4p3xfqj74yld5waqljf88fz25yxnma0cngspxe3les00fpjx/GGh9Ufn1SeDGrhzEkMyRKt5568VbbxZK2yvWNsd6PbXt", 6,None), // USDC (Ethereum via Wormhole)
    ("transfer/channel-2186/factory/wormhole14ejqjyq8um4p3xfqj74yld5waqljf88fz25yxnma0cngspxe3les00fpjx/5BWqpR48Lubd55szM5i62zK7TFkddckhbT48yy6mNbDp", 8,None), // Ethereum (Wormhole)
    ("transfer/channel-750/uusdc", 6,None), // USDC
    ("factory/osmo1s794h9rxggytja3a4pmwul53u98k06zy2qtrdvjnfuxruh7s8yjs6cyxgd/ucdt", 6,None), // CDT Stablecoin
    ("transfer/channel-6994/utia", 6,None), // Celestia
    ("transfer/channel-6787/adydx", 18,None), // dYdX Protocol (dYdX Protocol)
    ("transfer/channel-2716/FX", 18,None), // Function X
    ("transfer/channel-6897/usat", 14,None), // Nomic Bitcoin
    ("transfer/channel-874/factory/neutron1ug740qrkquxzrk2hh29qrlx3sktkfml3je7juusc2te7xmvsscns0n2wry/wstETH", 18,None), // Wrapped Lido Staked Ether
    ("factory/osmo1f5vfcph2dvfeqcqkhetwv75fda69z7e5c2dldm3kvgj23crkv6wqcn47a0/umilkTIA", 6,None), // milkTIA
    ("factory/osmo1z0qrq605sjgcqpylfl4aa6s90x738j7m58wyatt0tdzflg2ha26q67k743/wbtc", 8,None), // Wrapped Bitcoin
    ("factory/osmo17fel472lgzs87ekt9dvk0zqyh5gl80sqp4sk4n/LAB", 6,None), // LAB
    ("factory/osmo1s3l0lcqc7tu0vpj6wdjz9wqpxv8nk6eraevje4fuwkyjnwuy82qsx3lduv/boneOsmo", 6,None), // BackBone Labs Liquid Staked OSMO
    ("factory/osmo1em6xs47hd82806f5cxgyufguxrrc7l0aqx7nzzptjuqgswczk8csavdxek/alloyed/allUSDT", 6,None), // Tether USD
    ("factory/osmo1z6r6qdknhgsc0zeracktgpcxf43j6sekq07nw8sxduc9lg0qjjlqfu25e3/alloyed/allBTC", 8,None), // Bitcoin
    ("transfer/channel-208/arbitrum-weth-wei", 18,None), // Ethereum (Arbitrum via Axelar)
    ("transfer/channel-208/base-weth-wei", 18,None), // Ethereum (Base via Axelar)
    ("transfer/channel-208/polygon-weth-wei", 18,None), // Ethereum (Polygon via Axelar)
    ("factory/osmo1n3n75av8awcnw4jl62n3l48e6e4sxqmaf97w5ua6ddu4s475q5qq9udvx4/alloyed/allSOL", 9,None), // Solana
    ("ibc/0FA9232B262B89E77D1335D54FB1E1F506A92A7E4B51524B400DC69C68D28372", 6,None), // Penumbra
    ("factory/osmo14mafhhp337yjj2aujplawz0tks6jd2lel4hkwz4agyzhvvztzaqsqzjq8x/alloyed/allTRX", 6,None), // Tron
    ("factory/osmo1nufyzqlm8qhu2w7lm0l4rrax0ec8rsk69mga4tel8eare7c7ljaqpk2lyg/alloyed/allOP", 12,None), // Optimism
    ("transfer/channel-208/op-wei", 18,None), // Optimism (Axelar)
    ("factory/osmo1f588gk9dazpsueevdl2w6wfkmfmhg5gdvg2uerdlzl0atkasqhsq59qc6a/alloyed/allSHIB", 12,None), // Shiba Inu
    ("factory/osmo1p7x454ex08s4f9ztmm7wfv7lvtgdkfztj2u7v7fezfcauy85q35qmqrdpk/alloyed/allARB", 12,None), // Arbitrum
    ("factory/osmo18zdw5yvs6gfp95rp74qqwug9yduw2fyr8kplk2xgs726s9axc5usa2vpgw/alloyed/allLINK", 12,None), // Chainlink
    ("factory/osmo1nnlxegt0scm9qkzys9c874t0ntapv4epfjy2w49c0xdrp3dr0v4ssmelzx/alloyed/allPEPE", 12,None), // Pepe
    ("factory/osmo1r53fx9fvcdzncrs7zkn4gw5vfelx5gk8k5wc6wqha2jpkh992rusr5tk02/alloyed/allDOT", 10,None), // Polkadot
    ("transfer/channel-874/factory/neutron1ndu2wvkrxtane8se2tr48gv7nsm46y5gcqjhux/MARS", 6,None), // Mars Protocol
    ("factory/osmo12lnwf54yd30p6amzaged2atln8k0l32n7ncxf04ctg7u7ymnsy7qkqgsw4/alloyed/allTON", 9,None), // Toncoin
    ("transfer/channel-79840/stBTC", 18,None), // Lorenzo stBTC
    ("factory/osmo1myv2g72h8dan7n4hx7stt3mmust6ws03zh6gxc7vz4hpmgp5z3lq9aunm9/AVAIL.rt", 18,None), // Avail (Ethereum via Router)
    ("factory/osmo10c4y9csfs8q7mtvfg4p9gd8d0acx0hpc2mte9xqzthd7rd3348tsfhaesm/sICP-icrc-ckBTC", 8,None), // Chain-key Bitcoin
    ("transfer/channel-208/uni-wei", 18,None), // Uniswap (Axelar)
    ("factory/osmo1eqjda4pc6e09jtxzxggf6jl3jye2yn453ja58we5gxwzmf5ah28qvlnaz8/alloyed/allUNI", 12,None), // Uniswap
    ("transfer/channel-82819/uint3", 6,None), // Int3face
    ("transfer/channel-82819/factory/int31zlefkpe3g0vvm9a4h0jf9000lmqutlh99h7fsd/dogecoin-doge", 8,None), // Dogecoin (Int3)
    ("transfer/channel-82819/factory/int31zlefkpe3g0vvm9a4h0jf9000lmqutlh99h7fsd/bitcoin-btc", 8,None), // Bitcoin (Int3)
    ("transfer/channel-82819/factory/int31zlefkpe3g0vvm9a4h0jf9000lmqutlh99h7fsd/bitcoin-cash-bch", 8,None), // Bitcoin Cash (Int3)
    ("transfer/channel-82819/factory/int31zlefkpe3g0vvm9a4h0jf9000lmqutlh99h7fsd/litecoin-ltc", 8,None), // Litecoin (Int3)
    ("transfer/channel-208/arbitrum-uusdt", 6,None), // Tether USD (Ethereum) (Arbitrum via Axelar)
    ("transfer/channel-208/polygon-uusdt", 6,None), // Tether USD (Ethereum) (Polygon via Axelar)
    ("transfer/channel-208/cbbtc-satoshi", 8,None), // Coinbase Wrapped BTC (Axelar)
    ("transfer/channel-208/lbtc-satoshi", 8,None), // Lombard Staked Bitcoin (Ethereum via Axelar)
    ("factory/osmo10pk4crey8fpdyqd62rsau0y02e3rk055w5u005ah6ly7k849k5tsf72x40/alloyed/allDOGE", 8,None), // Dogecoin
    ("factory/osmo1csp8fk353hnq2tmulklecxpex43qmjvrkxjcsh4c3eqcw2vjcslq5jls9v/alloyed/allLTC", 8,None), // Litecoin
    ("factory/osmo1cranx3twqxfrgeqvgsu262gy54vafpc9xap6scye99v244zl970s7kw2sz/alloyed/allBCH", 8,None), // Bitcoin Cash
    ("transfer/channel-85077/uom", 6,None), // MANTRA
    ("transfer/channel-94814/uatone", 6,None), // AtomOne
    ("factory/osmo1n6asrjy9754q8y9jsxqf557zmsv3s3xa5m9eg5/uspice", 6,None), // Spice
    ("factory/osmo1qnglc04tmhg32uc4kxlxh55a5cmhj88cpa3rmtly484xqu82t79sfv94w0/alloyed/allXRP", 6,None), // Ripple
    ("transfer/channel-89321/uxion", 6,None), // Xion
    ("factory/osmo1ss0n3ghv5rr4z4y54fnkprc69tegmdm3ejlkgr2z4utnyg7eljgs9pztvs/alloyed/allFIL", 12,None), // Filecoin
    ("transfer/channel-82819/factory/int31zlefkpe3g0vvm9a4h0jf9000lmqutlh99h7fsd/ton-ton", 9,None), // Toncoin (Int3)
    ("factory/osmo10c4y9csfs8q7mtvfg4p9gd8d0acx0hpc2mte9xqzthd7rd3348tsfhaesm/sICP-native-ICP", 8,None), // Internet Computer
    ("transfer/channel-91017/uelys", 6,None), // Elys Network
    ("transfer/channel-113/factory/chihuahua1mzcfzh4ufk2cta59pm9a6wdyvv8c4v5epqzj46/Chihuahua", 6,None), // Chihuahua
    ("transfer/channel-874/factory/neutron1ut4c6pv4u6vyu97yw48y8g7mle0cat54848v6m97k977022lzxtsaqsgmq/udtia", 6,None), // dTIA
    ("factory/osmo1nqu7rc5mj5p2cgyfp7gl3lw7kw99cltple3xtzl2cs5fyw0r2tasr7xv48/alloyed/allSUI", 8,None), // Sui
    ("factory/osmo1zynnzvwdu72zc4mxqnnp348ksfmayldqyfs8khdud3myr7m5h8nsqwta2v/alloyed/allAPT", 8,None), // Aptos Coin
    ("factory/osmo1zetxzc5nka4jm203ljjtjf933jwjh45ge6spfeef447rnnhqxc4qrazrcz/alloyed/allBNB", 12,None), // Binance Coin
    ("factory/osmo1zem8r6dv6u38f6qpg546zy30946av8h5srgug0s4gcyy6cfecf3seac083/alloyed/allDYDX", 12,None), // dYdX Protocol
    ("factory/osmo1mdvn6lmykp2z345ncpf647dztslyll8cyhwj9pltrc0lf7nva3cqvrp6qs/alloyed/allFET", 12,None), // Fetch.ai
    ("transfer/channel-98416/unil", 6,None), // Nillion
    ("transfer/channel-101635/ubbn", 6,None), // Babylon
    ("transfer/channel-0/transfer/channel-1340/uclbtc", 8,None), // Lombard Staked Bitcoin
    ("transfer/channel-0/transfer/08-wasm-1369/0x7a56e1c57c7475ccf742a1832b028f0456652f97", 18,None), // SolvBTC
    ("transfer/channel-102122/uinit", 6,None), // Initia
    ("transfer/channel-89298/umilk", 6,None), // MilkyWay
    ("transfer/channel-0/transfer/08-wasm-1369/0x2260fac5e5542a773aa44fbcfedf7c193bc2c599", 8,None), // Wrapped Bitcoin (Ethereum via Eureka)
    ("transfer/channel-0/transfer/08-wasm-1369/0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2", 18,None), // Ethereum (Eureka)
    ("transfer/channel-0/transfer/08-wasm-1369/0xdac17f958d2ee523a2206206994597c13d831ec7", 6,None), // Tether USD (Ethereum via Eureka)
    ("transfer/channel-82819/factory/int31zlefkpe3g0vvm9a4h0jf9000lmqutlh99h7fsd/solana-sol", 9,None), // Solana (Int3)
    ("transfer/channel-94814/uphoton", 6,None), // Photon
    ("transfer/channel-82819/factory/int31zlefkpe3g0vvm9a4h0jf9000lmqutlh99h7fsd/xrpl-xrp", 6,None), // Ripple (xrpl via Int3)
    ("transfer/channel-750/uusdn", 6,None), // Noble Dollar
];

#[transaction]
fn apply_tx(ctx: &mut Ctx, _tx_data: BatchedTx) -> TxResult {
    // Read the current gas cost map
    let gas_cost_key = get_gas_cost_key();
    let mut minimum_gas_price: BTreeMap<Address, token::Amount> =
        ctx.read(&gas_cost_key)?.unwrap_or_default();

    // Enable IBC deposit/withdraws limits
    for (base_token, precision, can_be_used_for_gas) in OSMO_TOKENS {
        let ibc_denom = format!("transfer/{OSMO_CHANNEL_ID}/{base_token}");
        let token_address = ibc::ibc_token(&ibc_denom).clone();

        let factor = 10u64.pow(precision as u32);
        let mint_limit = MintTokenLimit::from_u64(MINT_LIMIT * factor);
        let throughput_limit = ThroughtputTokenLimit::from_u64(THROUGHPUT_LIMIT * factor);

        let mint_limit_token_key = ibc::mint_limit_key(&token_address);
        ctx.write(&mint_limit_token_key, mint_limit)?;

        let throughput_limit_token_key = ibc::throughput_limit_key(&token_address);
        ctx.write(&throughput_limit_token_key, throughput_limit)?;

        // Check if this ibc token should also be used to pay for gas
        if let Some(gas) = can_be_used_for_gas {
            minimum_gas_price.insert(token_address.clone(), gas);
        }
    }

    // Write the gas cost map back to storage
    ctx.write(&gas_cost_key, minimum_gas_price)?;

    Ok(())
}
