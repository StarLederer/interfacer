import { resolve } from "path"
import { spawn } from "child_process"

const logData = (data) => {
    console.log(data.toString());
};

const runVite = () => {
    const vite = spawn("pnpm", ["run", "build"], { cwd: resolve("./packages/renderer") });
    vite.stdout.on('data', logData);
    vite.stderr.on('data', logData);
    vite.on('close', (code) => {
        if (code === 0) {
            runTauri();
        }
    });
};

const runTauri = () => {
    const tauri = spawn("cargo", ["tauri", "build"], { cwd: resolve("./packages/main") });
    tauri.stdout.on('data', logData);
    tauri.stderr.on('data', logData);
};

runVite();
