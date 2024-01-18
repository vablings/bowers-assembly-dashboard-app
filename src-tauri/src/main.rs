// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use odbc_api::{
    buffers::TextRowSet, ConnectionOptions, Cursor, Environment, ResultSetMetadata,
};
use serde::{Deserialize, Serialize};


static QUERY: &'static str = include_str!("query.sql");

#[derive(Debug, Deserialize, Serialize)] 
struct Material {
    material: String,
    description: String,
    stock: String,
    safety: String,
}

#[tauri::command]
fn fetch() -> String {
    let environment = Environment::new().unwrap();
    let connection = environment
        .connect("SJGRP", "", "", ConnectionOptions::default())
        .unwrap();

    let mut writer = csv::Writer::from_writer(vec![]);
    let conres = connection.execute(&QUERY, ()).unwrap();

    match conres {     
        Some(mut cursor) => {
            let mut headline : Vec<String> = cursor.column_names().unwrap().collect::<Result<_,_>>().unwrap();
            writer.write_record(headline).unwrap();

            let mut buffers = TextRowSet::for_cursor(5000, &mut cursor, Some(4096)).unwrap();
            let mut row_set_cursor = cursor.bind_buffer(&mut buffers).unwrap();
            while let Some(batch) = row_set_cursor.fetch().unwrap() {
                // Within a batch, iterate over every row
                for row_index in 0..batch.num_rows() {
                    // Within a row iterate over every column
                    let record = (0..batch.num_cols()).map(|col_index| {
                        batch
                            .at(col_index, row_index)
                            .unwrap_or(&[])
                    });
                    // Writes row as csv
                    writer.write_record(record).unwrap();
                }
            }

            let data = String::from_utf8(writer.into_inner().unwrap()).unwrap();
            println!("{data}");
            let mut rdr = csv::Reader::from_reader(data.as_bytes());
            let mut materials: Vec<Material> = Vec::new();
            for result in rdr.deserialize() {
                let record: Material = result.unwrap();
                materials.push(record);
            }
            let mat_list_json = serde_json::to_string_pretty(&materials).unwrap();
            println!("{mat_list_json}");
            return mat_list_json;
        }
        None => {
            panic!("uh oh");
        }
    }
}


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}




fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, fetch])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
