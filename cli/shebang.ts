#!/usr/bin/env -S deno run --allow-env
const path = Deno.env.get("DENO_INSTALL");

console.log("Deno Install Path:", path);
