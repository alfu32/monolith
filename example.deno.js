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

let dbtest=monolith.open("dbtest")
console.log(dbtest);
console.log(dbtest.open);
console.log(dbtest.close);
console.log(dbtest.write_all_index);
console.log(dbtest.get_from_index);
console.log(dbtest.read_record_at_index);
console.log(dbtest.read_all_matching);
console.log(dbtest.read_all);
console.log(dbtest.write);
//monolith.all("dbtest","--json")
//monolith.all("dbtest","--csv")
console.log(dbtest.read_all());
Deno.core.print("Boom!\n");