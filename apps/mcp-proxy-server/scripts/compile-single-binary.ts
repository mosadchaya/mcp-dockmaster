import { join } from "https://deno.land/std@0.219.0/path/mod.ts";

console.log("Compiling single binary cwd", Deno.cwd());

const DIST_DIR = join(Deno.cwd(), "../../dist/apps/mcp-proxy-server");

// Clean and create dist directory
await Deno.remove(DIST_DIR, { recursive: true }).catch(() => {
  console.error("Failed to remove dist directory");
});
await Deno.mkdir(DIST_DIR, { recursive: true }).catch(() => {
  console.error("Failed to create dist directory");
});

const targets = [
  "x86_64-pc-windows-msvc",
  "aarch64-apple-darwin",
  "x86_64-unknown-linux-gnu",
];

for (const target of targets) {
  console.log(`Compiling for ${target}`);
  const output = join(DIST_DIR, `mcp-proxy-server-${target}`);
  console.log("Output file", output);
  const compileProcess = new Deno.Command("deno", {
    args: [
      "compile",
      "--allow-all",
      `--target=${target}`,
      "--output",
      output,
      ...(target === "x86_64-pc-windows-msvc" ? ["--no-terminal"] : []),
      "src/index.ts",
    ],
    stdout: "inherit",
    stderr: "inherit",
  });

  const compileOutput = await compileProcess.output();
  if (compileOutput.code !== 0) {
    console.error(`Failed to compile for ${target}`);
    Deno.exit(1);
  }

  if (Deno.build.os !== "windows" && target !== "x86_64-pc-windows-msvc") {
    await Deno.chmod(output, 0o755);
  }
}

console.log("Compiled successfully for all platforms");
