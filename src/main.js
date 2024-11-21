import { createApp } from 'vue';
import App from './App.vue';
import { Command } from '@tauri-apps/plugin-shell';

// Use spawn to execute the command asynchronously
const command = Command.sidecar('binaries/server', []);

async function run(command) {
    await command.execute();
}
run(command);

createApp(App).mount('#app');

