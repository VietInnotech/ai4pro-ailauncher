# Bundle input artifacts

Place runtime artifacts here before building release packages.

Required layout:

```text
bundle/
  binaries/
    llama-server-aarch64-apple-darwin
    llama-server-x86_64-pc-windows-msvc.exe
  runtime/
    sherpa-onnx-vit/
      python3
      python.exe
      ...packaged Python runtime and site-packages...
```

Do not place model files here.
Model files remain operator-managed under the app data root:

```text
<app-root>/models/llama/default/model.gguf
<app-root>/models/sherpa/default/
```

`scripts/validate-bundle-artifacts.mjs` fails release builds when required runtime artifacts are missing or model-like files are placed under this directory.
