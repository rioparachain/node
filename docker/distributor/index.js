const express = require('express');
const app = express();
const port = 3000;
const bodyParser = require('body-parser');

console.log('Version 1.0.1');

// parse application/x-www-form-urlencoded
app.use(bodyParser.urlencoded({ extended: false }))

// parse application/json
app.use(bodyParser.json())

// ---

let instances = [];

const relayInstances = () => {
	return instances.filter(v => v.type == 'relay');
};

const collatorInstances = () => {
	return instances.filter(v => v.type == 'collator');
};

app.use((req, res, next) => {
	console.log(req.url);
	next();
});

app.get('/reset', (req, res) => {
	instances = [];
	res.send(`OK=0`);
});

app.get('/', (req, res) => {
	res.json(instances);
});

app.get('/relay', async (req, res) => {
	await new Promise(resolve => setTimeout(resolve, Math.floor(Math.random() * 10000)));
	const freeIndex = relayInstances().length + 1;
	res.send(freeIndex.toString());
});

app.get('/collator', async (req, res) => {
	await new Promise(resolve => setTimeout(resolve, Math.floor(Math.random() * 10000)));
	const freeIndex = collatorInstances().length + 1;
	res.send(freeIndex.toString());
});

// --

// {\"key\": \"${ACCOUNT_PUBLIC_KEY}\", \"ip\": \"${IP_LOCAL}\"}
const example = {
	key: '...',
	ip: '...',
	account: 1,
	type: 'relay|collator'

};
app.post('/relay/:account', (req, res) => {
	console.log(req.params, req.body, req.query);
	instances.push({...req.params, ...req.body, ...req.query, type: 'relay'});
	console.log(relayInstances().length);
	res.json(relayInstances());
});

app.post('/collator/:account', (req, res) => {
	console.log(req.params, req.body);
	instances.push({...req.params, ...req.body, ...req.query, type: 'collator'});
	res.json(collatorInstances());
});

// -- to run collator

app.get('/relay/:account', (req, res) => {
	res.json(relayInstances()
		.filter(v => v.account === req.params.account)
	);
});

// ===

app.listen(port, '0.0.0.0', () => {
	console.log(`Example app listening on port ${port}`)
});
