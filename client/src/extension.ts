import { workspace, ExtensionContext } from "vscode";

import {
  Executable,
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
} from "vscode-languageclient/node";

let client: LanguageClient;

export function activate(_context: ExtensionContext) {
  const run: Executable = {
    command: process.env.SERVER_PATH,
    options: { env: { ...process.env, RUST_LOG: "debug" } },
  };

  const serverOptions: ServerOptions = { run, debug: run };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: "file", language: "shellscript" }],
    synchronize: {
      fileEvents: workspace.createFileSystemWatcher("**/.clientrc"),
    },
  };

  client = new LanguageClient(
    "alphabeticalBackupLSP",
    "Alphabetical Backup LSP",
    serverOptions,
    clientOptions
  );

  client.start();
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }

  return client.stop();
}
