// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use commands::{
    create_new_gist, get_commits_to_repository, get_gist_content, get_gists_for_authenticated_user,
    get_public_gists, get_public_repositories, get_repositories_for_authenticated_user,
    get_users_associated_with_repository,
};

mod api;
mod commands;
mod error;
mod models;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            create_new_gist,
            get_public_gists,
            get_repositories_for_authenticated_user,
            get_gist_content,
            get_users_associated_with_repository,
            get_commits_to_repository,
            get_public_repositories,
            get_gists_for_authenticated_user
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
