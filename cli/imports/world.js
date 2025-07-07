import { greet, greet_async } from "./hello.js"; 

greet("world");

greet_async("world#1");
greet_async("world#2");

class MyError {
  constructor(message) {
    this.message = message;
  }
}

async function world() {
  console.log("Hello World");

  // 直接内联等待
  await new Promise(resolve => setTimeout(resolve, 1500));

  console.log("Hello world again");

  //throw new Error("error");
  throw new MyError("error");

  // return "hello world";
}

export {
  world
}
