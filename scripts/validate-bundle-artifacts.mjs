#!/usr/bin/env node
import { accessSync, constants, existsSync, readdirSync, statSync } from 'node:fs';
import { join, relative, sep } from 'node:path';
import { fileURLToPath } from 'node:url';

const scriptDir = fileURLToPath(new URL('.', import.meta.url));
const repoRoot = join(scriptDir, '..');
const bundleRoot = join(repoRoot, 'src-tauri', 'bundle');

const target = process.env.LOCAL_AI_BUNDLE_TARGET ?? `${process.platform}-${process.arch}`;

const targetArtifacts = {
  'darwin-arm64': {
    requiredFiles: [
      'binaries/llama-server-aarch64-apple-darwin',
      'binaries/libllama-common.0.dylib',
      'binaries/libmtmd.0.dylib',
      'binaries/libllama.0.dylib',
      'binaries/libggml.0.dylib',
      'binaries/libggml-cpu.0.dylib',
      'binaries/libggml-blas.0.dylib',
      'binaries/libggml-metal.0.dylib',
      'binaries/libggml-rpc.0.dylib',
      'binaries/libggml-base.0.dylib',
      'runtime/sherpa-onnx-vit/python3',
      'runtime/sherpa-onnx-vit/lib/python3.14/models/vad/silero_vad.onnx',
    ],
    requiredDirectories: [
      'runtime/sherpa-onnx-vit/lib/python3.14/site-packages/sherpa_onnx',
      'runtime/sherpa-onnx-vit/lib/python3.14/site-packages/sherpa_onnx_vit',
    ],
    executableFiles: [
      'binaries/llama-server-aarch64-apple-darwin',
      'runtime/sherpa-onnx-vit/python3',
    ],
  },
  'win32-x64': {
    requiredFiles: [
      'binaries/llama-server-x86_64-pc-windows-msvc.exe',
      'runtime/sherpa-onnx-vit/python.exe',
    ],
    requiredDirectories: [],
    executableFiles: [],
  },
};

const selectedArtifacts = targetArtifacts[target];

if (!selectedArtifacts) {
  console.error('Bundle artifact validation failed.');
  console.error('');
  console.error(`- unsupported release target: ${target}`);
  console.error('');
  console.error('Set LOCAL_AI_BUNDLE_TARGET to one of: darwin-arm64, win32-x64.');
  process.exit(1);
}

const forbiddenModelPatterns = [
  /^models(\/|$)/,
  /(^|\/)model\.gguf$/i,
  /(^|\/)encoder.*\.onnx$/i,
  /(^|\/)decoder.*\.onnx$/i,
  /(^|\/)joiner.*\.onnx$/i,
  /(^|\/)silero_vad\.onnx$/i,
  /(^|\/)tokens\.txt$/i,
  /(^|\/)bpe\.model$/i,
];

const allowedRuntimeModelLikePaths = new Set([
  'runtime/sherpa-onnx-vit/lib/python3.14/models/vad/silero_vad.onnx',
]);

const errors = [];

function relativePath(path) {
  return relative(bundleRoot, path).split(sep).join('/');
}

function requireFile(path) {
  const fullPath = join(bundleRoot, path);
  if (!existsSync(fullPath)) {
    errors.push(`missing required bundled artifact: src-tauri/bundle/${path}`);
    return;
  }

  if (!statSync(fullPath).isFile()) {
    errors.push(`required bundled artifact is not a file: src-tauri/bundle/${path}`);
  }
}

function requireDirectory(path) {
  const fullPath = join(bundleRoot, path);
  if (!existsSync(fullPath)) {
    errors.push(`missing required bundled directory: src-tauri/bundle/${path}`);
    return;
  }

  if (!statSync(fullPath).isDirectory()) {
    errors.push(`required bundled artifact is not a directory: src-tauri/bundle/${path}`);
  }
}

function requireExecutable(path) {
  const fullPath = join(bundleRoot, path);
  if (!existsSync(fullPath)) {
    return;
  }

  try {
    accessSync(fullPath, constants.X_OK);
  } catch {
    errors.push(`required artifact is not executable: src-tauri/bundle/${path}`);
  }
}

function walk(dir, visitor) {
  if (!existsSync(dir)) {
    return;
  }

  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    const fullPath = join(dir, entry.name);
    visitor(fullPath, entry);
    if (entry.isDirectory()) {
      walk(fullPath, visitor);
    }
  }
}

for (const file of selectedArtifacts.requiredFiles) {
  requireFile(file);
}

for (const dir of selectedArtifacts.requiredDirectories) {
  requireDirectory(dir);
}

for (const file of selectedArtifacts.executableFiles) {
  requireExecutable(file);
}

if (!existsSync(join(bundleRoot, 'runtime', 'sherpa-onnx-vit'))) {
  errors.push('missing required bundled runtime directory: src-tauri/bundle/runtime/sherpa-onnx-vit');
}

walk(bundleRoot, (fullPath, entry) => {
  if (!entry.isFile() && !entry.isDirectory()) {
    return;
  }

  const rel = relativePath(fullPath);
  if (!allowedRuntimeModelLikePaths.has(rel) && forbiddenModelPatterns.some((pattern) => pattern.test(rel))) {
    errors.push(`model-like file/path must not be bundled: src-tauri/bundle/${rel}`);
  }
});

if (errors.length > 0) {
  console.error('Bundle artifact validation failed.');
  console.error('');
  for (const error of errors) {
    console.error(`- ${error}`);
  }
  console.error('');
  console.error('Provide runtime artifacts under src-tauri/bundle/. Model files must stay under the app data models/ directory on target machines.');
  process.exit(1);
}

console.log(`Bundle artifact validation passed for ${target}.`);
