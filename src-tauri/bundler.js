const ext = Deno.build.os === 'windows' ? '.exe' : '';

// Run the rustc command
const rustcProcess = Deno.run({
  cmd: ['rustc', '-vV'],
  stdout: 'piped',
  stderr: 'piped',
});
const output = await rustcProcess.output(); // Resolves a Uint8Array
const rustInfo = new TextDecoder().decode(output);

rustcProcess.close();

const targetTripleMatch = /host: (\S+)/g.exec(rustInfo);
if (!targetTripleMatch) {
  console.error('Failed to determine platform target triple');
  Deno.exit(1);
}
const targetTriple = targetTripleMatch[1];

// Rename the file
await Deno.rename(
  `./server${ext}`,
  `./src-tauri/binaries/server-${targetTriple}${ext}`
);
