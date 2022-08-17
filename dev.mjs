import { resolve } from "path"
import { spawn } from "child_process"

const vite = spawn("pnpm", ["run", "dev"], { cwd: resolve("./packages/renderer") });
const tauri = spawn("cargo", ["tauri", "dev"], { cwd: resolve("./packages/main") });

// Common

const logData = (data) => {
    console.log(data.toString());
};

// Vite events

vite.stdout.on('data', logData);
vite.stderr.on('data', logData);

// Tauri events

tauri.stdout.on('data', logData);
tauri.stderr.on('data', logData);

// Mutual killing

tauri.on('close', () => {
    vite.kill();
});

vite.on('close', () => {
    tauri.kill();
});
