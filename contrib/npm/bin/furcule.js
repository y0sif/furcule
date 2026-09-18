#!/usr/bin/env node
import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";
const here = dirname(fileURLToPath(import.meta.url));
const bin = join(here, process.platform === "win32" ? "furcule.exe" : "furcule-bin");
const r = spawnSync(bin, process.argv.slice(2), { stdio: "inherit" });
process.exit(r.status ?? 1);
