const axios = require('axios').default;

// Make a request for a user with a given ID
axios.get('http://15.0.129.230:9933/health')
	.then(function (response) {
		// handle success
		console.log(response);
	})
	.catch(function (error) {
		// handle error
		console.log(error);
	});


