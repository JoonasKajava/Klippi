import type { UserSettings } from "$lib/models/UserSettings";
import { invoke } from "@tauri-apps/api";


export async function get_user_settings() : Promise<UserSettings> {
    return invoke<UserSettings>("get_user_settings");
}