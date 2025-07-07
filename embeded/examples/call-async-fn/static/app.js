class MyError {
  constructor(message) {
    this.message = message;
  }
}

async function world_err() {
  console.log("Hello World");

  // 直接内联等待
  await new Promise(resolve => setTimeout(resolve, 1000));

  console.log("Hello world again");

  throw new MyError("error");
}

async function world_ok() {
  console.log("Hello World");

  // 直接内联等待
  await new Promise(resolve => setTimeout(resolve, 1000));

  console.log("Hello world again");
}

export {
  world_err,
  world_ok,
}