const runCommand = require('./cmd.js');
const genAccounts = require("./exec");

const {
    MNEMONIC,
    RELAY_RAW,
    PARA_RAW
} = process.env;

(async () => {
	const { relay, collator } = await genAccounts();

	await Promise.all([relay, collator].map(async (chainAccounts, chainIdx) => {
		await Promise.all(chainAccounts.map(async (acc, accIdx) => {
			await Promise.all([
				'coll', 'para', 'asgn',
				'acco', 'audi', 'imon',
				'babe', 'gran', 'aura'
			].map(async keyType => {
				const suf = ('00'+(1+accIdx)).substr(-2);
				await runCommand([
					`/rio/src/target/release/${['relaychain-rio', 'parachain-rio'][chainIdx]} key insert`,
					`--base-path /rio/keys/${['relay', 'collator'][chainIdx]}-${suf}`,
					`--chain ${[RELAY_RAW, PARA_RAW][chainIdx]}`,
					`--scheme ${{gran: 'Ed25519'}[keyType] || 'Sr25519'}`,
					`--suri '//${MNEMONIC}//${acc.number}'`,
					`--key-type ${keyType}`
				].join(' '));
			}));
		}));
	}));
})();
