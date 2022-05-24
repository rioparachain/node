const fs = require('fs');
const JSONbig = require('json-bigint')({ useNativeBigInt: true });
const genAccounts = require('./exec.js');

const {
    FILE_FROM = '/tmp/docker1/.chainspec/polkadot-custom-2-plain-local.json',
    FILE_TO = '/tmp/docker1/.chainspec/1.json',
    FILE_ACCOUNTS = '/tmp/accounts.json',
    MNEMONIC = 'chronic scene situate genuine advice gospel lady obvious blood palace economy marble'
} = process.env;

(async () => {
    const chainspecStr = fs.readFileSync(FILE_FROM).toString();
    const paraWasm = fs.readFileSync("/tmp/para-2000-wasm").toString();
    const paraGenesis = fs.readFileSync("/tmp/para-2000-genesis").toString();
    const j = JSONbig.parse(chainspecStr);

    const accounts = await genAccounts();

    j.telemetryEndpoints = null;
    j.name = 'Rio Relay Chain Staging Testnet';
    //j.id = 'rio_relay_chain_staging_testnet';
    //j.protocolId = 'rioc';
    //j.chainType = 'Live'; // not working
    j.genesis.runtime.staking.validatorCount = 6;
    j.genesis.runtime.staking.minimumValidatorCount = 4;
    //j.genesis.runtime.staking.forceEra = 'NotForcing';
    //j.genesis.runtime.configuration.config.max_validators_per_core = 1;

    j.genesis.runtime.balances.balances = accounts.map(acc => {
        return [
            acc.orig['Public key (SS58)'],
            10000000000000000
        ];
    }).concat(accounts.map(acc => {
        return [
            acc.stash['Public key (SS58)'],
            10000000000000000
        ];
    }));

    j.genesis.runtime.paras = [[[0, {
	    genesis_head: paraGenesis,
	    validation_code: paraWasm,
	    parachain: true,
    }]]];

    // j.genesis.runtime.balances.balances = [
    //     [
    //         "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    //         10000000000000000
    //     ],
    //     [
    //         "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
    //         10000000000000000
    //     ],
    //     [
    //         "5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y",
    //         10000000000000000
    //     ],
    //     [
    //         "5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy",
    //         10000000000000000
    //     ],
    //     [
    //         "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw",
    //         10000000000000000
    //     ],
    //     [
    //         "5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL",
    //         10000000000000000
    //     ],
    //     [
    //         "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY",
    //         10000000000000000
    //     ],
    //     [
    //         "5HpG9w8EBLe5XCrbczpwq5TSXvedjrBGCwqxK1iQ7qUsSWFc",
    //         10000000000000000
    //     ],
    //     [
    //         "5Ck5SLSHYac6WFt5UZRSsdJjwmpSZq85fd5TRNAdZQVzEAPT",
    //         10000000000000000
    //     ],
    //     [
    //         "5HKPmK9GYtE1PSLsS1qiYU9xQ9Si1NcEhdeCq9sw5bqu4ns8",
    //         10000000000000000
    //     ],
    //     [
    //         "5FCfAonRZgTFrTd9HREEyeJjDpT397KMzizE6T3DvebLFE7n",
    //         10000000000000000
    //     ],
    //     [
    //         "5CRmqmsiNFExV6VbdmPJViVxrWmkaXXvBrSX8oqBT8R9vmWk",
    //         10000000000000000
    //     ]
    // ];

    j.genesis.runtime.sudo.key = accounts[4].orig['Public key (SS58)'];

    j.genesis.runtime.staking.invulnerables = accounts.slice(0, 4).map(acc => {
        return acc.stash['Public key (SS58)'];
    });
    // j.genesis.runtime.staking.invulnerables = [
    //     "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY",
    //     "5HpG9w8EBLe5XCrbczpwq5TSXvedjrBGCwqxK1iQ7qUsSWFc"
    // ];

    j.genesis.runtime.staking.stakers = accounts.slice(0, 4).map(acc => {
        return [
            acc.stash['Public key (SS58)'],
            acc.orig['Public key (SS58)'],
            1000000000000,
            "Validator"
        ];
    });

    // j.genesis.runtime.staking.stakers = [
    //     [
    //         "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY",
    //         "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    //         1000000000000,
    //         "Validator"
    //     ],
    //     [
    //         "5HpG9w8EBLe5XCrbczpwq5TSXvedjrBGCwqxK1iQ7qUsSWFc",
    //         "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
    //         1000000000000,
    //         "Validator"
    //     ]
    // ];

    j.genesis.runtime.session.keys = accounts.slice(0, 4).map(acc => {
        return [
            acc.stash['Public key (SS58)'],
            acc.stash['Public key (SS58)'],
            {
                "grandpa": acc.grandpa['Public key (SS58)'],
                "babe": acc.orig['Public key (SS58)'],
                "im_online": acc.orig['Public key (SS58)'],
                "para_validator": acc.orig['Public key (SS58)'],
                "para_assignment": acc.orig['Public key (SS58)'],
                "authority_discovery": acc.orig['Public key (SS58)']
            }
        ];
    });

    // j.genesis.runtime.session.keys = [
    //     [
    //         "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY",
    //         "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY",
    //         {
    //             "grandpa": "5FA9nQDVg267DEd8m1ZypXLBnvN7SFxYwV7ndqSYGiN9TTpu",
    //             "babe": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    //             "im_online": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    //             "para_validator": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    //             "para_assignment": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    //             "authority_discovery": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
    //         }
    //     ],
    //     [
    //         "5HpG9w8EBLe5XCrbczpwq5TSXvedjrBGCwqxK1iQ7qUsSWFc",
    //         "5HpG9w8EBLe5XCrbczpwq5TSXvedjrBGCwqxK1iQ7qUsSWFc",
    //         {
    //             "grandpa": "5GoNkf6WdbxCFnPdAnYYQyCjAKPJgLNxXwPjwTh6DGg6gN3E",
    //             "babe": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
    //             "im_online": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
    //             "para_validator": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
    //             "para_assignment": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
    //             "authority_discovery": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"
    //         }
    //     ]
    // ];

    fs.writeFileSync(FILE_TO, JSONbig.stringify(j, null, '  '));
    fs.writeFileSync(FILE_ACCOUNTS, JSONbig.stringify(accounts, null, '  '));
})().catch(err => {
    console.error(err);
});
