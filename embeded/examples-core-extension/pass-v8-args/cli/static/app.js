import *  as lib from "sammyne:hello_world";

const s = "hello world";

let b = lib.string_to_bytes(s);
console.log(`b = ${b}`);


let ss = lib.string_from_bytes(b);
console.log(`ss = '${ss}'`);

let some = await lib.return_option(false);
console.log(`some = ${some}`);
let none = await lib.return_option(true);
console.log(`none = ${none}`);
