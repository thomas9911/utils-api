import { parseArgs } from "jsr:@std/cli/parse-args";
import { readAll } from "jsr:@std/io/read-all";
import type { Reader, Writer } from "jsr:@std/io/types";
import { loadSync } from "jsr:@std/dotenv";

let URL = "https://utils.tostitijd.com";
const HELP = `
Usage:
    deno run --no-prompt --allow-net="utils.tostitijd.com" main.ts [SUBCOMMAND]

    SUBCOMMANDS:
       help         Show this message and exit.
       random       Generate a random number.
       uuid         Generate a UUID.
       graphql      Format or minify graphql

    random:
      --output      select output format of the random number. One of: 'u32', 'u64', 'u128', 'f32', 'f64'
      --binary      output binary data instead of a decimal number directly to the stdout (output flag will be ignored)
      --size        when binary flag given output this many bytes

    uuid:
      --format      select output format of the UUID. One of: 'braced', 'hyphenated', 'simple', 'urn'
      --version     select the version of uuid. One of: 'v4', 'v7'

    graphql:
      takes in graphql from the stdin and outputs to the stdout

      --pretty      pretty print the graphql
      --minify      minify the graphql (default)
`;

async function fetchData(
  path: string,
  params: URLSearchParams,
): Promise<string> {
  const response = await fetch(URL + path + params.toString());
  if (!response.ok) {
    const rawText = await response.text();
    const errorText = rawText.replace(
      "Failed to deserialize query string: u",
      "U",
    );
    throw new Error(errorText);
  }
  const data = await response.text();
  return data;
}

interface UuidArgs {
  version?: "v4" | "v7" | string;
  format?: "braced" | "hyphenated" | "simple" | "urn" | string;
}

function uuidArgsIntoParameters(args: UuidArgs): URLSearchParams {
  const params = new URLSearchParams();
  if (args.version) {
    params.set("version", args.version);
  }
  if (args.format) {
    params.set("format", args.format);
  }
  return params;
}

async function uuid(args: UuidArgs): Promise<string> {
  const path = "/api/uuid?";
  const params = uuidArgsIntoParameters(args);
  return await fetchData(path, params);
}

interface RandomArgs {
  output?: "u32" | "u64" | "u128" | "f32" | "f64" | string;
}

async function random(args: RandomArgs): Promise<string> {
  const path = "/api/random?";
  const params = new URLSearchParams();
  if (args.output) {
    params.set("output", args.output);
  }

  return await fetchData(path, params);
}

interface RandomStreamArgs {
  size?: number | string;
  outWriter: Writer;
}

async function randomStream(args: RandomStreamArgs): Promise<void> {
  const path = "/api/random/stream?";
  const params = new URLSearchParams();
  if (args.size) {
    params.set("size", args.size.toString());
  }

  const response = await fetch(URL + path + params.toString());
  if (!response.ok) {
    const rawText = await response.text();
    const errorText = rawText.replace(
      "Failed to deserialize query string: i",
      "I",
    ).replace("string", "size");
    throw new Error(errorText);
  }

  for await (const chunk of response.body!) {
    await args.outWriter.write(chunk);
  }
}

interface GraphQLArgs {
  minify?: boolean;
  pretty?: boolean;
  inReader: Reader;
  outWriter: Writer;
}

async function graphql(args: GraphQLArgs): Promise<void> {
  const input = await readAll(args.inReader);

  const pathPrettier = "/api/graphql/prettier";
  const pathMinifier = "/api/graphql/minifier";

  let path;
  if (args.pretty) {
    path = pathPrettier;
  } else {
    path = pathMinifier;
  }

  const response = await fetch(URL + path, { method: "POST", body: input });
  if (!response.ok) {
    throw new Error(await response.text());
  }

  for await (const chunk of response.body!) {
    await args.outWriter.write(chunk);
  }
}

async function main() {
  try {
    // Only used in dev
    loadSync({ export: true });
    URL = Deno.env.get("UTILS_API_URL") || URL;
  } catch (error) {
    if ((error as Error).name != "NotCapable") {
      throw error;
    }
  }

  const args = parseArgs(Deno.args, {
    boolean: ["pretty", "minify"],
    string: ["format", "version", "size", "output"],
  });
  try {
    switch (args._[0]) {
      case "uuid":
        console.log(await uuid({ version: args.version, format: args.format }));
        break;

      case "random":
        if (args.binary) {
          await randomStream({ size: args.size, outWriter: Deno.stdout });
          return;
        }
        console.log(await random({ output: args.output }));
        break;

      case "graphql":
        await graphql({
          inReader: Deno.stdin,
          outWriter: Deno.stdout,
          minify: args.minify,
          pretty: args.pretty,
        });
        break;

      default:
        console.log(HELP);
        break;
    }
  } catch (error) {
    console.error((error as Error).message);
    Deno.exit(1);
  }
}

main();
