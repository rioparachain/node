const { exec } = require('child_process');

module.exports = async command => {
    const stdout = await new Promise((resolve, reject) => {
        exec(command, (stderr, stdout) => stderr
            ? reject(stderr)
            : resolve(stdout)
        );
    });
    const k = stdout.split(/\n/g);
    const o = {};
    /* example: {
      'Secret phrase': 'harbor history matter vacuum result service grit solid thought vapor toward sudden'
      'Secret seed': '0xece619a707d7b6813e90b74b223cdd03af7b4817632001d141d3ab5f813b8fa9',
      'Public key (hex)': '0x40b9cacd5356cfd030f5a0a59576261ea299dc418e2e599d59dde39f2966c42d',
      'Account ID': '0x40b9cacd5356cfd030f5a0a59576261ea299dc418e2e599d59dde39f2966c42d',
      'Public key (SS58)': '5DXa7YNoBEJwwnadQ7WZNKQ5cPvq2ZJkXxRAqaSz8jt4cMMy',
      'SS58 Address': '5DXa7YNoBEJwwnadQ7WZNKQ5cPvq2ZJkXxRAqaSz8jt4cMMy'
    }*/
    k.forEach(v => {
        if (!v) return;
        let [key, value] = v.split(':');
        o[key.trim()] = value.trim();
    });
    //console.log(o);
    return o;
};