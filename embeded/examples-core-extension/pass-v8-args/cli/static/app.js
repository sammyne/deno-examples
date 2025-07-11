import *  as lib from "sammyne:hello_world";

const s = "hello world";

let b = lib.string_to_bytes(s);
console.log(`b = ${b}`);


let ss = lib.string_from_bytes(b);
console.log(`ss = '${ss}'`);