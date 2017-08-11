let ffi = require('ffi');
let path = require('path');

let fibonacci = ffi.Library(path.join(__dirname, './target/debug/libfibonacci'), {
    calc: ['int', ['int']]
});

let result = fibonacci.calc(30);
console.log("Fibonacci: " + result);