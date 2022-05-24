const {
    MNEMONIC = 'chronic scene situate genuine advice gospel lady obvious blood palace economy marble'
} = process.env;

const runCommand = require('./cmd.js');

// (async () => {
//     const arr = await Promise.all([1,2,3,4,5,6].map(async accNumb => {
//         const orig = await runCommand(`/tmp/polkadot/target/release/polkadot key inspect --scheme Sr25519 '//${MNEMONIC}//1'`);
//         const stash = await runCommand(`/tmp/polkadot/target/release/polkadot key inspect --scheme Sr25519 '//${MNEMONIC}//1//stash'`);
//         return {
//             orig, stash
//         };
//     }));
//     // const o = await runCommand(`/tmp/polkadot/target/release/polkadot key inspect --scheme Sr25519 '//Alice//stash'`);
//     console.log(JSON.stringify(arr, null, '  '));
// })();

module.exports = async () => {
    const arr = await Promise.all([1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16].map(async accNumb => {
        const orig = await runCommand(`/tmp/riochain/target/release/relaychain-rio key inspect --scheme Sr25519 '//${MNEMONIC}//${accNumb}'`);
        const stash = await runCommand(`/tmp/riochain/target/release/relaychain-rio key inspect --scheme Sr25519 '//${MNEMONIC}//${accNumb}//stash'`);
        const grandpa = await runCommand(`/tmp/riochain/target/release/relaychain-rio key inspect --scheme Ed25519 '//${MNEMONIC}//${accNumb}'`);
        return {
            orig, stash, grandpa,
            number: accNumb
        };
    }));
    // const o = await runCommand(`/tmp/polkadot/target/release/polkadot key inspect --scheme Sr25519 '//Alice//stash'`);
    // console.log(JSON.stringify(arr, null, '  '));
    return arr;
};
