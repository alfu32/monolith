// example.deno.js
Deno.core.print("Hello runjs!\n");

console.log("Hello", "runjs!");
console.error("Boom!");

const path = "./log.txt";
try {
    const contents = await runjs.readFile(path);
    console.log("Read from a file", contents);
} catch (err) {
    console.error("Unable to read file", path, err);
}

await runjs.writeFile(path, "I can write to a file.");
await runjs.writeFile("stays.log", "I can write to a file.");
const contents = await runjs.readFile(path);
console.log("Read from a file", path, "contents:", contents);
console.log("Removing file", path);
runjs.removeFile(path);
console.log("File removed");


monolith.all("dbtest","--json")
monolith.all("dbtest","--csv")
Deno.core.print("Boon!\n");