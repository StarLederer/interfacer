import { resolve } from "path"
import { spawn } from "child_process"

const logData = (data) => {
    console.log(data.toString());
};

const pnpm = spawn("pnpm", ["install"], { cwd: resolve("./packages/renderer") });
pnpm.stdout.on('data', logData);
pnpm.stderr.on('data', logData);
