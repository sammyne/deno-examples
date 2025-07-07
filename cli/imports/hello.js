function greet(who) {
  console.log(`hello world ${who}`);
}

async function greet_async(who) {
  console.log(`hello world ${who}`);

  // 直接内联等待
  await new Promise(resolve => setTimeout(resolve, 1500));
  
  await console.log(`hello world ${who} again`);
  console.log(`hello world ${who} again and again`);
}

export { greet, greet_async };
