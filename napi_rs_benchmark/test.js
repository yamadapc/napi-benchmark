const addon = require('./napi_rs_benchmark.darwin-arm64');

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

  testRustMap()
  testRustVec()
}

function testRustMap() {
  console.log('inserting 1 million entries into rust hash map')
  const map = addon.createRustMap();
  const strings = [];
  for (let i = 0; i < 1e6; i += 1) {
    strings.push(String(i));
  }

  {
    const start = performance.now();
    for (let i = 0; i < 1e6; i += 1) {
      const obj = {prop: i}
      addon.rustMapInsert(map, strings[i], obj);
    }
    const end = performance.now();
    console.log('time', end - start);
  }

  console.log('looking-up 1 million entries from rust hash map')
  {
    const start = performance.now();
    for (let i = 0; i < 1e6; i += 1) {
      const storedValue = addon.rustMapLookup(map, strings[i]);
      if (i !== storedValue.prop) {
        throw new Error('something is wrong')
      }
    }
    const end = performance.now();
    console.log('time', end - start);
  }
}

function testRustVec() {
  console.log('inserting 1 million entries into rust vec')
  const vec = addon.createRustVec();

  {
    const start = performance.now();
    for (let i = 0; i < 1e6; i += 1) {
      const obj = {prop: i}
      addon.rustVecInsert(vec, obj);
    }
    const end = performance.now();
    console.log('time', end - start);
  }

  console.log('looking-up 1 million entries from rust vec')
  {
    const start = performance.now();
    for (let i = 0; i < 1e6; i += 1) {
      const storedValue = addon.rustVecLookup(vec, i);
      if (i !== storedValue.prop) {
        throw new Error('something is wrong')
      }
    }
    const end = performance.now();
    console.log('time', end - start);
  }
}

runSync();
