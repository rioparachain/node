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
    w.genesis.runtime.collatorSelection.invulnerables = collator.slice(1, 3).map(acc => {
        return acc.orig['Public key (SS58)'];
    });
    w.genesis.runtime.collatorSelection.desiredCandidates = 4;
    w.genesis.runtime.session.keys = collator.slice(1, 3).map(acc => {
        return [
            acc.orig['Public key (SS58)'],
            acc.orig['Public key (SS58)'],
            {
                "aura": acc.orig['Public key (SS58)'],
            }
        ];
    });

	// .: rio-gateway :.
	// Register = 1 << 0, | 1
	// Deposit = 1 << 1, | 2
	// Withdraw = 1 << 2, | 4
	// Sudo = 1 << 3, | 8
	w.genesis.runtime.rioGateway.admins = [
		[ collator[0].orig['Public key (SS58)'], { mask: 15 } ],
		// rio requested addr
		[ '5ECTV5r1u6GK4nMqNSvaL5t4fgU5sTMs375iV82Z4bxUHU7Y', { mask: 4 } ],
		[ '5DUo6kB6XzLtyeijTuuC9YVw2jtB3VSMd46aTBvpjXYA2uin', { mask: 2 } ],
		[ '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY', { mask: 2 } ],
		// test addr
		[ 'WFJmNPw8gBVdqUwz5WsRLfSEJ9Cynd2gxckz3mAVbkraDaq', { mask: 2 } ]
	];

    fs.writeFileSync(PARA_FILE_TO, JSONbig.stringify(w, null, '  '));
})().catch(err => {
    console.error(err);
});
