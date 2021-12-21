const {
  quicktype,
  InputData,
  JSONSchemaInput,
  JSONSchemaStore,
} = require("quicktype-core");
const path = require("path");
const fs = require("fs").promises;

async function* walk(dir) {
  for await (const d of await fs.opendir(dir)) {
    const entry = path.join(dir, d.name);
    if (d.isDirectory()) yield* await walk(entry);
    else if (d.isFile()) yield entry;
  }
}

async function main() {
  const schemaInput = new JSONSchemaInput(new JSONSchemaStore());
  const targetLanguage = "rust";

  const schemaRoot = path.join(
    __dirname,
    "rust",
    "open-metadata",
    "json",
    "schema"
  );

  const typesPath = path.join(schemaRoot, "type");
  // const typeSchemaFiles = await fs.readdir(typesPath);
  //
  // typeSchemaFiles.forEach(async (s) => {
  //   const filepath = path.join(typesPath, s);
  //   const schema = await fs.readFile(filepath, "utf-8");
  //   schemaInput.addSourceSync({ name: s, schema: schema });
  // });
  //
  // const filepath = path.join(
  //   __dirname,
  //   "rust",
  //   "open-metadata",
  //   "json",
  //   "schema",
  //   "entity",
  //   "tags",
  //   "tagCategory.json"
  // );
  // const schema = await fs.readFile(filepath, "utf-8");
  // schemaInput.addSourceSync({ name: "tagCategory.json", schema: schema });

  // const files = await fs.readdir(path.join(schemaRoot, "entity"));
  // console.log(files.filter((s) => s.includes(".json")));

  for await (const p of walk(schemaRoot)) {
    const schema = await fs.readFile(p, "utf-8");
    schemaInput.addSourceSync({ name: p, schema: schema });
  }

  const inputData = new InputData();
  inputData.addInput(schemaInput);

  const { lines: pythonPerson } = await quicktype({
    inputData,
    lang: targetLanguage,
    rendererOptions: { density: "dense" },
  });

  const targetFilepath = path.join(
    __dirname,
    "rust",
    "open-metadata",
    "src",
    "schema",
    "generated.rs"
  );
  await fs.writeFile(targetFilepath, pythonPerson.join("\n"));
}

main();
