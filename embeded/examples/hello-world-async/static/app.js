console.log("Hello, World!");

async function world() {
  console.log("Hello World");

  // 直接内联等待
  await new Promise(resolve => setTimeout(resolve, 1500));

  console.log("Hello world again");
}

await world();
