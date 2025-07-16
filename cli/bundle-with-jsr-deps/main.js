import { encodeBase64, decodeBase64 } from "@std/encoding";
import { assertEquals } from "@std/assert";

const foobar = new TextEncoder().encode("foobar");
assertEquals(encodeBase64(foobar), "Zm9vYmFy");
assertEquals(decodeBase64("Zm9vYmFy"), foobar);
