// polkadot api
// - https://polkadot.js.org/docs/api/start/types.basics

// get rfuel balance
// - https://scan.riochain.io/api/v1/account/5DUo6kB6XzLtyeijTuuC9YVw2jtB3VSMd46aTBvpjXYA2uin

// get transactions
// - https://scan.riochain.io/api/v1/extrinsic?filter[signed]=1&page[number]=20000&page[size]=25

const {
	AIRDROP_SCAN_PAGES
} = process.env;

if (!AIRDROP_SCAN_PAGES) {
	console.error(new Error('NO_AIRDROP_SCAN_PAGES'));
	process.exit(3);
}

const argv = require('minimist')(process.argv.slice(2));
console.log(argv);

const Promise = require('bluebird');
const fs = require('fs');
const JSONbig = require('json-bigint')({useNativeBigInt: true});

const axios = require('axios');
const {ApiPromise, WsProvider} = require('@polkadot/api');
const {get, pick, forEach, find} = require('lodash');

const transactionUrl = 'https://scan.riochain.io/api/v1/extrinsic';
const balances = {};

const fetchPage = async pageNumber => {
	console.log('start', pageNumber);
	const response = await axios.get(transactionUrl, {
		params: {
			'filter[signed]': 1,
			'page[number]': pageNumber,
			'page[size]': 25
		}
	});
	const data = get(response, 'data.data', []);
	await Promise.all(data.map(async tx => {
		const transactionId = get(tx, 'attributes.extrinsic_hash');
		if (!transactionId) {
			console.log('.');
			return null;
		}
		const txResponse = await axios.get(
			`${transactionUrl}/0x${transactionId}`
		);
		const txData = get(txResponse, 'data.data.attributes');
		if (txData.address && txData.address.length !== 48) {
			txData;
		}
		balances[txData.address] = null;
		const param = find(txData.params, p => ['Address'].includes(p.type));
		if (param) {
			if (param.value && param.value.length !== 48) {
				txData;
			}
			balances[param.value] = null;
		} else {
			txData.params;
		}
	}));
	console.log('end', pageNumber);
	return data.length !== 0;
};

const defaultAssets = {
	0: 'RFUEL', // decimals: 12
	1: 'LOCKED_RFUEL', // decimals: 12
	2: 'OM', // decimals: 12
	100: 'rBTC', // decimals: 8
	101: 'rLTC', // decimals: 8
	102: 'rUSDT', // decimals: 6
	103: 'rETH', // decimals: 18
	1001: 'SPM', // decimals: 12
};

function asseTypeToCode(assetType) {
	let code = null;
	Object.entries(defaultAssets).forEach(([key, value]) => {
		if (value === assetType) {
			code = key;
		}
	});
	if (!code) {
		console.warn('NO_ASSET_CODE', assetType);
	}
	return code;
}

const types = {
	"Amount": "i128",
	"AmountOf": "Amount",
	"CurrencyId": "u32",
	"CurrencyIdOf": "CurrencyId",
	"Price": "FixedU128",
	"OracleKey": "CurrencyId",
	"Chain": {
		"_enum": [
			"Rio",
			"Bitcoin",
			"Litecoin",
			"Ethereum",
			"EOS",
			"Polkadot",
			"Kusama",
			"ChainX"
		]
	},
	"AssetInfo": {
		"chain": "Chain",
		"symbol": "Text",
		"name": "Text",
		"decimals": "u8",
		"desc": "Text"
	},
	"FeeExchangeV1": {
		"max_payment": "Compact<Balance>"
	},
	"FeeExchange": {
		"_enum": {
			"V1": "Compact<FeeExchangeV1>"
		}
	},
	"Restriction": {
		"_enum": [
			"Transferable",
			"Depositable",
			"Withdrawable",
			"Slashable",
			"Reservable",
			"Unreservable"
		]
	},
	"TxHash": "H256",
	"Deposit": {
		"account_id": "AccountId",
		"amount": "Balance"
	},
	"Auths": {
		"mask": "u8"
	},
	"Auth": {
		"_enum": [
			"Register",
			"Deposit",
			"Withdraw",
			"Sudo"
		]
	},
	"WithdrawState": {
		"_enum": {
			"Pending": null,
			"Cancelled": null,
			"Rejected": null,
			"Approved": null,
			"Success": "TxHash",
			"ReBroadcasted": "TxHash"
		}
	},
	"ChainAddress": "Text",
	"Memo": "Text",
	"WithdrawInfo": {
		"asset_id": "AssetId",
		"who": "AccountId",
		"value": "Balance",
		"addr": "ChainAddress",
		"memo": "Text"
	},
	"WithdrawItem": {
		"currency_id": "CurrencyId",
		"applicant": "AccountId",
		"value": "Balance",
		"addr": "ChainAddress",
		"memo": "Text",
		"state": "WithdrawState"
	},
	"DepositAddrInfo": {
		"_enum": {
			"Bip32": "Bip32",
			"Create2": "Create2"
		}
	},
	"Bip32": {
		"x_pub": "Text",
		"path": "Text"
	},
	"Create2": {
		"creator_address": "Vec<u8>",
		"implementation_address": "Vec<u8>",
		"vault_address": "Vec<u8>"
	},
	"String": "Text",
	"WithdrawItemForRpc": {
		"currency_id": "CurrencyId",
		"applicant": "AccountId",
		"value": "String",
		"addr": "String",
		"memo": "String",
		"state": "WithdrawState",
		"fee": "String"
	},
	"AccountDepositAddr": {
		"deposit_addr_info": "DepositAddrInfo",
		"index": "Option<u64>"
	},
	"riogateway": {
		"withdrawList": {
			"description": "get current withdraw list(include pending and approve)",
			"params": [
				{
					"name": "at",
					"isOptional": true
				}
			],
			"type": "BTreeMap<u64, WithdrawItemForRpc>"
		},
		"pendingWithdrawList": {
			"description": "get current pending withdraw list",
			"params": [
				{
					"name": "at",
					"isOptional": true
				}
			],
			"type": "BTreeMap<u64, WithdrawItemForRpc>"
		},
		"depositAddress": {
			"description": "get deposit address info for an account and asset, if this account have not apply, in bip32 path would return `nil`",
			"params": [
				{
					"name": "at",
					"who": "AccountId",
					"currency_id": "CurrencyId",
					"isOptional": true
				}
			],
			"type": "DepositAddrForRpc"
		}
	}
};

const getAssets = async (api, address) => {
	const o = {
		address,
		timestamp: Date.now(),
		balances: {}
	};

	await Promise.all(Object.keys(defaultAssets).filter(assetId => assetId !== 0).map(async assetId => {
		o.balances[defaultAssets[assetId]] = await api.query.rioAssets.accounts(address, assetId).catch((e) => {
			console.log(address, assetId, 'fail');
			console.error(e);
			return null;
		});
	}));

	o.balances[defaultAssets[0]] = (await api.query.system.account(address).catch((e) => {
		console.log(address, 0, 'fail');
		console.error(e);
		return {data: null};
	})).data;

	return o;
};

(async () => {
	const date = new Date();
	if (argv.action === 'accounts') {
		// await Promise.map(Array(50), (_, pageNumber) =>
		const scanPages = parseInt(AIRDROP_SCAN_PAGES || '50');
		await Promise.map(Array(scanPages), (_, pageNumber) =>
			fetchPage(pageNumber),
			{concurrency: 6}
		);

		// fs.writeFileSync(`./addreses_${date.toISOString()}.json`, JSONbig.stringify(Object.keys(balances), null, '  '));
		// fs.writeFileSync(`./addreses_latest.json`, JSONbig.stringify(Object.keys(balances), null, '  '));
		fs.writeFileSync(`./addreses_latest.json`, JSONbig.stringify(Object.keys(balances), null, '  '));

		console.log('Fetched', Object.keys(balances).length, 'addresses');
		if (!Object.keys(balances).length) {
			process.exit(1);
		}
		if (Object.keys(balances).filter(key => key === null).length > 0) {
			console.log('With errors!!!');
		}
		return null;
	} else if (argv.action === 'export' && (argv['chainspec-polkadot'] || argv['chainspec-parachain'])) {
		// } else if (true) {
		let arr = JSONbig.parse(fs.readFileSync('./balances_latest.json').toString());
		// cleanup free balances
		arr = arr.filter(({balances}) => {
			const zeroBalance = Object.keys(balances).every(
				asset => Object.keys(balances[asset]).every(
					assetType => balances[asset][assetType] == 0
				)
			);
			return !zeroBalance;
		});

		const j = JSONbig.parse(fs.readFileSync(
			argv['chainspec-polkadot'] || argv['chainspec-parachain']
		).toString());
		console.log('before', j.genesis.runtime.balances.balances.length);
		arr.forEach(({address, balances}) => {
			if (!balances.RFUEL) {
				console.warn('NO_RFUEL', address);
				return null;
			}
			let val = BigInt(balances.RFUEL.free);

			if (!val) {
				// console.warn('NO_BALANCE', val, 'for', arr[i]);
				return null;
			}

			if (argv['chainspec-parachain']) {
				if (val < 1_000_000_000) {
					console.warn('LOW_BALANCE', val, 'for', address);
					return null;
				}
				j.genesis.runtime.balances.balances.push([address, val]);
			} else if (argv['chainspec-polkadot']) {
				if (val < 10_000_000_000_000) {
					console.warn('LOW_BALANCE_POLKADOT', val, 'for', address);
					return null;
				}
				j.genesis.runtime.balances.balances.push([address, val / 100n]);
			}
		});
		console.log('after', j.genesis.runtime.balances.balances.length);

		// adding assets
		if (argv['chainspec-parachain']) {
			arr.forEach(({address, balances}, i) => {
				Object.keys(balances).forEach(assetType => {
					const assetCode = asseTypeToCode(assetType);
					let b;
					j.genesis.runtime.rioAssets.init.forEach(([code, _1, _2, bals]) => {
						// console.log(code, assetCode, code == assetCode);
						if (code == assetCode) {
							b = bals;
						}
					});
					if (!b) {
						if (assetType != 'RFUEL') {
							console.warn('NOT_FOUND_ASSET', {assetType, assetCode});
						}
						return null;
					}
					//console.log(b);
					let val = BigInt(balances[assetType].free);
					if (assetType == 'LOCKED_RFUEL') {
						val = BigInt(balances[assetType].reserved);
					}

					if (!val) {
						return;
					}

					b.push([address, val]);
				});
			});
		}

		argv['chainspec-polkadot'] && fs.writeFileSync(argv['chainspec-polkadot'], JSONbig.stringify(j, null, '  '));
		argv['chainspec-parachain'] && fs.writeFileSync(argv['chainspec-parachain'], JSONbig.stringify(j, null, '  '));

		return null;
	} else if (argv.action === 'balances') {
		const wsProvider1 = new WsProvider('wss://node.v1.riochain.io/ws');
		// const wsProvider2 = new WsProvider('wss://node.v1.riochain.io/ws');
		// const wsProvider1 = new WsProvider('wss://node.riochain.io/');
		const wsProvider2 = new WsProvider('wss://node.riochain.io/');
		const api1 = await ApiPromise.create({provider: wsProvider1, types});
		const api2 = await ApiPromise.create({provider: wsProvider2, types});
		const apiPool = [api1, api2];

		// ==========
		// console.log(await getAssets(api, '5DJJUjtm39QhgFPZ8TQuHsTr8cXEFd2Ma7iPwWX2a4FLe8Q4'));

		const balanceArr = JSONbig.parse(
			fs.readFileSync(`./addreses_latest.json`).toString()
		);

		// const balanceFn = `./balances_${date.toISOString()}.json`;
		// const writeStream = Promise.promisifyAll(fs.createWriteStream(balanceFn));

		const [addressInitial] = balanceArr;
		const assInitial = await getAssets(api1, addressInitial);
		// await writeStream.writeAsync(`[\n\t${JSONbig.stringify(assInitial)}`);
		const okInitial = assInitial.balances[defaultAssets[0]];
		console.log(0, `> ${addressInitial}`, okInitial ? 'ok' : 'fail');

		const bal = {};

		const reconnectLimit = 500;
		let cnt = reconnectLimit;
		await Promise.map(balanceArr, async (address, i) => {
			// await Promise.map(balanceArr.slice(1), async (address, i) => {
			// await Promise.map(['5DJJUjtm39QhgFPZ8TQuHsTr8cXEFd2Ma7iPwWX2a4FLe8Q4'], async (address, i) => {
			// await Promise.map(['5EWeVJ3CU7rVT7SgpAe3wHcVysMVGqmxnKEKbNGJnQsboWhi'], async (address, i) => {
			// await Promise.map(['5Ei1vMoZ8UVYsxZDAM9LfsoAT3oYD98KRfDEMkggJLJ6pFqn'], async (address, i) => {
			if (address.length < 10) return;

			if (cnt <= 0) {
				cnt = reconnectLimit;
				apiPool[0] = await ApiPromise.create({provider: wsProvider1, types});
				apiPool[1] = await ApiPromise.create({provider: wsProvider2, types});
			}
			const api = apiPool[cnt % 2];
			cnt--;
			console.log(i + 1, `> ${address}`, 'start');
			let ass = await getAssets(api, address);
			// let ok = ass.balances[defaultAssets[0]];
			// console.log(i + 1, `> ${address}`, ok ? 'ok' : 'retry');
			// if (!ok) {
			// 	ass = await getAssets(api, address);
			// 	ok = ass.balances[defaultAssets[0]];
			// 	console.log(i + 1, `> ${address}`, ok ? 'ok' : 'fail');
			// }
			//await writeStream.writeAsync(`,\n\t"${address}": ${JSONbig.stringify(ass.balances)}`);
			// await writeStream.writeAsync(`,\n\t${JSONbig.stringify(ass)}`);
			bal[address] = ass;
			// await new Promise(resolve => setTimeout(resolve, 500));
			ass = null;
		}, {concurrency: 20});

		//await writeStream.writeAsync('\n]\n');
		//writeStream.end();

		console.log('\n\n\n\n===================== TEST BALANCES ======================\n\n\n\n');
		//  0: 'RFUEL', // decimals: 12
		// 	1: 'LOCKED_RFUEL', // decimals: 12
		// 	2: 'OM', // decimals: 12
		// 	100: 'rBTC', // decimals: 8
		// 	101: 'rLTC', // decimals: 8
		// 	102: 'rUSDT', // decimals: 6
		// 	103: 'rETH', // decimals: 18
		// 	1001: 'SPM', // decimals: 12
		bal['5ERMJZEbW12wzqtpxhY551yrm5TUJmqyp7qgvLKVutSCNkRC'] = {
			"address": "5ERMJZEbW12wzqtpxhY551yrm5TUJmqyp7qgvLKVutSCNkRC",
			"timestamp": Date.now(),
			"description": "Sergiy Khoroshko test balance",
			"balances": {
				"RFUEL": {
					"free": 10e12,
					"reserved": 0,
					"miscFrozen": 0,
					"feeFrozen": 0
				},
				"LOCKED_RFUEL": {
					"free": 0,
					"reserved": 0.8e12,
					"miscFrozen": 0,
					"feeFrozen": 0
				},
				"OM": {
					"free": 2e12,
					"reserved": 0,
					"miscFrozen": 0,
					"feeFrozen": 0
				},
				"rBTC": {
					"free": 0.3e8,
					"reserved": 0,
					"miscFrozen": 0,
					"feeFrozen": 0
				},
				"rUSDT": {
					"free": 400e6,
					"reserved": 0,
					"miscFrozen": 0,
					"feeFrozen": 0
				},
				"rETH": {
					"free": 5e18,
					"reserved": 0,
					"miscFrozen": 0,
					"feeFrozen": 0
				},
				"rLTC": {
					"free": 6e8,
					"reserved": 0,
					"miscFrozen": 0,
					"feeFrozen": 0
				},
				"SPM": {
					"free": 7e12,
					"reserved": 0,
					"miscFrozen": 0,
					"feeFrozen": 0
				}
			}
		};

		fs.writeFileSync(`./balances_latest.json`, JSONbig.stringify(Object.values(bal), null, '  '));
		//fs.writeFileSync(balanceFn, JSONbig.stringify(Object.values(bal)));
		//fs.copyFileSync(balanceFn, `./balances_latest.json`);

		// gen csv
		// let csv = [];
		// let csvHeader = null;
		// Object.values(bal).forEach(({address, balances}) => {
		// 	const row = [address];
		// 	csvHeader = csvHeader || ['address'].concat(Object.keys(balances)).join(',');
		// 	Object.keys(balances).forEach(asset => {
		// 		row.push(balances[asset].free);
		// 	});
		// 	csv.push(row.join(','));
		// });
		// // sort not working
		// // csv = csv.sort((x, y) => x[1] - y[1] || x[2] - y[2] || x[3] - y[3] || x[4] - y[4]);
		// csv = [csvHeader].concat(csv);
		// fs.writeFileSync(`./balances_latest.csv`, csv.join('\n'));

		console.log('Done in', Date.now() - date, 'msec');
	}

	process.exit(0);
})();
