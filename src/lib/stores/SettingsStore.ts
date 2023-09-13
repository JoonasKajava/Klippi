import type { UserSettings } from '$lib/models/UserSettings';
import { invoke } from '@tauri-apps/api';

export async function getUserSettings(): Promise<UserSettings> {
    return await invoke<UserSettings>('get_user_settings');
}
