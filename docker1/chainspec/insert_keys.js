const runCommand = require('./cmd.js');
const fs = require('fs');

const {
    MNEMONIC = 'chronic scene situate genuine advice gospel lady obvious blood palace economy marble',
    FILE_ACCOUNTS = '/tmp/accounts.json',
    CHAINSPEC_RAW = '/tmp/docker1/.chainspec/polkadot-custom-2-raw.json',
    PARA_CHAINSPEC_RAW = '/tmp/docker1/.chainspec/parachain-rio-custom-raw.json'
} = process.env;

(async () => {
    const accounts = JSON.parse(
        fs.readFileSync(FILE_ACCOUNTS)
    );
    accounts.forEach(async acc => {
        //await runCommand(`rm -rf /tmp/node-0${acc.number}`);
	
	//if (acc.number < 7) {

        await runCommand([
            '/tmp/riochain/target/release/relaychain-rio key insert',
            `--base-path /tmp/node-0${acc.number}`,
            `--chain ${CHAINSPEC_RAW}`,
            `--scheme Sr25519`,
            `--suri '//${MNEMONIC}//${acc.number}'`,
            `--key-type coll`
        ].join(' '));
        await runCommand([
            '/tmp/riochain/target/release/relaychain-rio key insert',
            `--base-path /tmp/node-0${acc.number}`,
            `--chain ${CHAINSPEC_RAW}`,
            `--scheme Sr25519`,
            `--suri '//${MNEMONIC}//${acc.number}'`,
            `--key-type para`
        ].join(' '));
        await runCommand([
            '/tmp/riochain/target/release/relaychain-rio key insert',
            `--base-path /tmp/node-0${acc.number}`,
            `--chain ${CHAINSPEC_RAW}`,
            `--scheme Sr25519`,
            `--suri '//${MNEMONIC}//${acc.number}'`,
            `--key-type asgn`
        ].join(' '));
        await runCommand([
            '/tmp/riochain/target/release/relaychain-rio key insert',
            `--base-path /tmp/node-0${acc.number}`,
            `--chain ${CHAINSPEC_RAW}`,
            `--scheme Sr25519`,
            `--suri '//${MNEMONIC}//${acc.number}'`,
            `--key-type acco`
        ].join(' '));

        await runCommand([
            '/tmp/riochain/target/release/relaychain-rio key insert',
            `--base-path /tmp/node-0${acc.number}`,
            `--chain ${CHAINSPEC_RAW}`,
            `--scheme Sr25519`,
            `--suri '//${MNEMONIC}//${acc.number}'`,
            `--key-type audi`
        ].join(' '));

        await runCommand([
            '/tmp/riochain/target/release/relaychain-rio key insert',
            `--base-path /tmp/node-0${acc.number}`,
            `--chain ${CHAINSPEC_RAW}`,
            `--scheme Sr25519`,
            `--suri '//${MNEMONIC}//${acc.number}'`,
            `--key-type imon`
        ].join(' '));

        await runCommand([
            '/tmp/riochain/target/release/relaychain-rio key insert',
            `--base-path /tmp/node-0${acc.number}`,
            `--chain ${CHAINSPEC_RAW}`,
            `--scheme Sr25519`,
            `--suri '//${MNEMONIC}//${acc.number}'`,
            `--key-type babe`
        ].join(' '));

        await runCommand([
            '/tmp/riochain/target/release/relaychain-rio key insert',
            `--base-path /tmp/node-0${acc.number}`,
            `--chain ${CHAINSPEC_RAW}`,
            `--scheme Ed25519`,
            `--suri '//${MNEMONIC}//${acc.number}'`,
            `--key-type gran`
        ].join(' '));

	//} else {
        await runCommand([
            '/tmp/riochain/target/release/parachain-rio key insert',
            `--base-path /tmp/node-0${acc.number}`,
            `--chain ${PARA_CHAINSPEC_RAW}`,
            `--scheme Sr25519`,
            `--suri '//${MNEMONIC}//${acc.number}'`,
            `--key-type coll`
        ].join(' '));
        await runCommand([
            '/tmp/riochain/target/release/parachain-rio key insert',
            `--base-path /tmp/node-0${acc.number}`,
            `--chain ${PARA_CHAINSPEC_RAW}`,
            `--scheme Sr25519`,
            `--suri '//${MNEMONIC}//${acc.number}'`,
            `--key-type para`
        ].join(' '));
        await runCommand([
            '/tmp/riochain/target/release/parachain-rio key insert',
            `--base-path /tmp/node-0${acc.number}`,
            `--chain ${PARA_CHAINSPEC_RAW}`,
            `--scheme Sr25519`,
            `--suri '//${MNEMONIC}//${acc.number}'`,
            `--key-type asgn`
        ].join(' '));

        await runCommand([
            '/tmp/riochain/target/release/parachain-rio key insert',
            `--base-path /tmp/node-0${acc.number}`,
            `--chain ${PARA_CHAINSPEC_RAW}`,
            `--scheme Sr25519`,
            `--suri '//${MNEMONIC}//${acc.number}'`,
            `--key-type acco`
        ].join(' '));

        await runCommand([
            '/tmp/riochain/target/release/parachain-rio key insert',
            `--base-path /tmp/node-0${acc.number}`,
            `--chain ${PARA_CHAINSPEC_RAW}`,
            `--scheme Sr25519`,
            `--suri '//${MNEMONIC}//${acc.number}'`,
            `--key-type audi`
        ].join(' '));

        await runCommand([
            '/tmp/riochain/target/release/parachain-rio key insert',
            `--base-path /tmp/node-0${acc.number}`,
            `--chain ${PARA_CHAINSPEC_RAW}`,
            `--scheme Sr25519`,
            `--suri '//${MNEMONIC}//${acc.number}'`,
            `--key-type imon`
        ].join(' '));

        await runCommand([
            '/tmp/riochain/target/release/parachain-rio key insert',
            `--base-path /tmp/node-0${acc.number}`,
            `--chain ${PARA_CHAINSPEC_RAW}`,
            `--scheme Sr25519`,
            `--suri '//${MNEMONIC}//${acc.number}'`,
            `--key-type aura`
        ].join(' '));

	//}


    });
})();
