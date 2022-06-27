const fs = require('fs');
const JSONbig = require('json-bigint')({ useNativeBigInt: true });
const genAccounts = require('./exec.js');

const {
    PARA_FILE_FROM,
    PARA_FILE_TO
} = process.env;

(async () => {
    const para_chainspecStr = fs.readFileSync(PARA_FILE_FROM).toString();
    const w = JSONbig.parse(para_chainspecStr);

	const { relay, collator } = await genAccounts();
	const accounts = [...relay, ...collator];

    w.name = 'Rio Parachain Chain Staging Testnet';
    w.id = 'parachain_local';
    //w.id = 'rio_parachain_staging_testnet';
    //w.protocolId = 'riop';
    //w.chainType = 'Live';
    w.relay_chain = 'polkadot-local';
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
    w.genesis.runtime.collatorSelection.invulnerables = collator.slice(0, 2).map(acc => {
        return acc.orig['Public key (SS58)'];
    });
    w.genesis.runtime.collatorSelection.desiredCandidates = 4;
    w.genesis.runtime.session.keys = collator.slice(0, 2).map(acc => {
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
