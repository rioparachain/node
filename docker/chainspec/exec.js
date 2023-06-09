const {
	MNEMONIC
} = process.env;

if (!MNEMONIC) {
	console.error(new Error('MNEMONIC phrase is not set!'));
	process.exit(2);
}

const runCommand = require('./cmd.js');

// GENERATING PUBLIC KEYS
module.exports = async () => {
    const all = [];
    for(let i=0; i<=60; i++) all.push(i);
    const arr = await Promise.all(all.map(async accNumb => {
		const suf = ('00'+accNumb).substr(-2);
        const orig = await runCommand(`/rio/src/target/release/relaychain-rio key inspect --scheme Sr25519 '//${MNEMONIC}//${suf}'`);
        const stash = await runCommand(`/rio/src/target/release/relaychain-rio key inspect --scheme Sr25519 '//${MNEMONIC}//${suf}//stash'`);
        const grandpa = await runCommand(`/rio/src/target/release/relaychain-rio key inspect --scheme Ed25519 '//${MNEMONIC}//${suf}'`);
        return {
            orig, stash, grandpa,
            number: suf
        };
    }));

	const relay = arr.splice(0, arr.length/2);
    return {
		relay,
		collator: arr
	};
};
