const fs = require('fs');
const JSONbig = require('json-bigint')({ useNativeBigInt: true });
const genAccounts = require('./exec.js');

const {
    RELAY_FILE_FROM,
    RELAY_FILE_TO,
    PARA_WASM,
    PARA_GENESIS
} = process.env;

(async () => {
    const chainspecStr = fs.readFileSync(RELAY_FILE_FROM).toString();
    const paraWasm = fs.readFileSync(PARA_WASM).toString();
    const paraGenesis = fs.readFileSync(PARA_GENESIS).toString();
    const j = JSONbig.parse(chainspecStr);

    const { relay, collator } = await genAccounts();
	const accounts = [...relay, ...collator];

    j.telemetryEndpoints = null;
    j.name = 'Rio Relay Chain Staging Testnet';
    //j.id = 'rio_relay_chain_staging_testnet';
    //j.protocolId = 'rioc';
    //j.chainType = 'Live'; // not working
    j.genesis.runtime.staking.validatorCount = 6;
    j.genesis.runtime.staking.minimumValidatorCount = 4;
	//j.properties.tokenDecimals = 12;
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

    // any account as sudo
    j.genesis.runtime.sudo.key = accounts[4].orig['Public key (SS58)'];

    j.genesis.runtime.staking.invulnerables = relay.slice(0, 4).map(acc => {
        return acc.stash['Public key (SS58)'];
    });

    j.genesis.runtime.staking.stakers = relay.slice(0, 4).map(acc => {
        return [
            acc.stash['Public key (SS58)'],
            acc.orig['Public key (SS58)'],
            1000000000000,
            "Validator"
        ];
    });

    j.genesis.runtime.session.keys = relay.slice(0, 4).map(acc => {
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

    fs.writeFileSync(RELAY_FILE_TO, JSONbig.stringify(j, null, '  '));
})().catch(err => {
    console.error(err);
});
