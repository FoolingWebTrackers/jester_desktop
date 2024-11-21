const ext = Deno.build.os === 'windows' ? '.exe' : '';

// Run the rustc command
const rustcProcess = Deno.run({
  cmd: ['rustc', '-vV'],
  stdout: 'piped',
  stderr: 'piped',
});
const output = await rustcProcess.output();
const rustInfo = new TextDecoder().decode(output);

const errorOutput = await rustcProcess.stderrOutput();
const rustError = new TextDecoder().decode(errorOutput);

rustcProcess.close();

if (rustError) {
  console.error(`Rustc error: ${rustError}`);
  Deno.exit(1);
}

const targetTripleMatch = /host: (\S+)/g.exec(rustInfo);
if (!targetTripleMatch) {
  console.error('Failed to determine platform target triple');
  Deno.exit(1);
}
const targetTriple = targetTripleMatch[1];
const serverPath = `./server${ext}`;
const targetPath = `./src-tauri/binaries/server-${targetTriple}${ext}`;

try {
  await Deno.stat(serverPath);
} catch {
  console.error(`File not found: ${serverPath}`);
  Deno.exit(1);
}

await Deno.mkdir('./src-tauri/binaries', { recursive: true });

try {
  await Deno.rename(serverPath, targetPath);
  console.log(`Renamed ${serverPath} to ${targetPath}`);
} catch (err) {
  console.error(`Failed to rename: ${err.message}`);
  Deno.exit(1);
}
