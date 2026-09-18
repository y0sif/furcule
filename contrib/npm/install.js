// Downloads the furcule release binary matching this platform into bin/.
// The version comes from package.json and must equal a GitHub release tag.
import { createWriteStream, chmodSync, existsSync, mkdirSync, renameSync, rmSync } from "node:fs";
import { pipeline } from "node:stream/promises";
import { createGunzip } from "node:zlib";
import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";
import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const { version } = require("./package.json");
const here = dirname(fileURLToPath(import.meta.url));
const targets = {
  "linux-x64": "x86_64-unknown-linux-gnu",
  "linux-arm64": "aarch64-unknown-linux-gnu",
  "darwin-x64": "x86_64-apple-darwin",
  "darwin-arm64": "aarch64-apple-darwin",
  "win32-x64": "x86_64-pc-windows-msvc",
};
const key = `${process.platform}-${process.arch}`;
const target = targets[key];
if (!target) {
  console.error(`furcule: no prebuilt binary for ${key}; install with cargo instead`);
  process.exit(0);
}
const tag = `v${version}`;
const name = `furcule-${tag}-${target}`;
const isWin = process.platform === "win32";
const url = `https://github.com/y0sif/furcule/releases/download/${tag}/${name}.${isWin ? "zip" : "tar.gz"}`;
const outDir = join(here, "bin");
const outBin = join(outDir, isWin ? "furcule.exe" : "furcule-bin");
mkdirSync(outDir, { recursive: true });

const res = await fetch(url);
if (!res.ok) {
  console.error(`furcule: download failed (${res.status}) ${url}`);
  process.exit(1);
}
const archive = join(outDir, `${name}.${isWin ? "zip" : "tar.gz"}`);
await pipeline(res.body, createWriteStream(archive));
if (isWin) {
  spawnSync("powershell", ["-NoProfile", "-Command", `Expand-Archive -Force '${archive}' '${outDir}'`], { stdio: "inherit" });
  renameSync(join(outDir, name, "furcule.exe"), outBin);
} else {
  spawnSync("tar", ["-xzf", archive, "-C", outDir], { stdio: "inherit" });
  renameSync(join(outDir, name, "furcule"), outBin);
  chmodSync(outBin, 0o755);
}
rmSync(archive, { force: true });
rmSync(join(outDir, name), { recursive: true, force: true });
if (!existsSync(outBin)) process.exit(1);
