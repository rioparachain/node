const fs = require('fs');
const JSONbig = require('json-bigint')({ useNativeBigInt: true });
const genAccounts = require('./exec.js');

const {
    PARA_FILE_FROM = '/tmp/docker1/.chainspec/parachain-rio-custom-plain-local.json',
    PARA_FILE_TO = '/tmp/docker1/.chainspec/2.json',
    MNEMONIC = 'chronic scene situate genuine advice gospel lady obvious blood palace economy marble'
} = process.env;

(async () => {
    const para_chainspecStr = fs.readFileSync(PARA_FILE_FROM).toString();
    const w = JSONbig.parse(para_chainspecStr);

    const accounts = await genAccounts();

    w.name = 'Rio Parachain Chain Staging Testnet';
    w.id = 'parachain_local';
    //w.id = 'rio_parachain_staging_testnet';
    //w.protocolId = 'riop';
    //w.chainType = 'Live';
    //w.relay_chain = 'relaychain-rio';
    w.relay_chain = 'polkadot-local';
    //w.relay_chain = 'local_testnet';
    //w.relay_chain = 'westend_local_testnet';
    //w.relay_chain = 'rio_relay_chain_staging_testnet';
    w.para_id = 0;
    w.genesis.runtime.balances.balances = accounts.map(acc => {
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
    w.genesis.runtime.collatorSelection.invulnerables = accounts.slice(6, 8).map(acc => {
        return acc.orig['Public key (SS58)'];
    });
    w.genesis.runtime.collatorSelection.desiredCandidates = 4;
    w.genesis.runtime.session.keys = accounts.slice(6, 8).map(acc => {
        return [
            acc.orig['Public key (SS58)'],
            acc.orig['Public key (SS58)'],
            {
                "aura": acc.orig['Public key (SS58)'],
            }
        ];
    });
    fs.writeFileSync(PARA_FILE_TO, JSONbig.stringify(w, null, '  '));
})().catch(err => {
    console.error(err);
});
