const addon = require('./build/Release/benchmark_addon');

function runSync() {
  console.log('addon', addon);

  console.log('calling noop function 1 million times');
  {
    const start = performance.now();
    for (let i = 0; i < 1e6; i += 1) {
      addon.hello();
    }
    const end = performance.now();
    console.log('time', end - start);
  }

  console.log('calling round-trip function 1 million times with number');
  {
    const start = performance.now();
    for (let i = 0; i < 1e6; i += 1) {
      const result = addon.roundTrip(i);
      if (result !== i) throw new Error('FAILURE');
    }
    const end = performance.now();
    console.log('time', end - start);
  }

  console.log('calling round-trip function 1 million times with strings');
  {
    const strings = [];
    for (let i = 0; i < 1e6; i += 1) {
      strings.push(String(i));
    }

    const start = performance.now();
    for (let i = 0; i < 1e6; i += 1) {
      const result = addon.roundTrip(strings[i]);
      if (result !== strings[i]) throw new Error('FAILURE');
    }
    const end = performance.now();
    console.log('time', end - start);
  }

  const obj = { hello: 'world' };
  if (addon.roundTrip(obj) !== obj) throw new Error('WHAT C++ COPIED DATA!??')
}

runSync();
