import {invoke} from '@tauri-apps/api/tauri';

async function scanDirectory(directoryPath: string) {
    try {
        await invoke<string>('scan_directory', {
            path: directoryPath
        } as any);
    } catch (e) {
        console.error(e);
    }
}

