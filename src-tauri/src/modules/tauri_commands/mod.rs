use super::file_processing::video_metadata::find_lastest_videos;


#[tauri::command]
pub async fn get_latest_videos(count: usize) -> Vec<String> {
    find_lastest_videos().into_iter().take(count).collect()
}