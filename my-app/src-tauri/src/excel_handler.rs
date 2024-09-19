use calamine::{open_workbook, DataType, Reader, Xlsx};
use tauri::api::dialog::FileDialogBuilder;

#[tauri::command]
pub async fn select_excel_file() -> Result<String, String> {
  let (tx, rx) = std::sync::mpsc::channel();
  FileDialogBuilder::new().add_filter("Excel Files", &["xls", "xlsx"]).pick_file(move |file_path| {
    if let Some(path) = file_path {
      tx.send(path.to_string_lossy().to_string()).expect("Failed to send file path")
    }
  });

  rx.recv().map_err(|_| "File selection cancelled".to_string())
}

#[tauri::command]
pub async fn read_excel_file(path: String) -> Result<Vec<String>, String> {
  let mut workbook: Xlsx<_> = open_workbook(path).map_err(|e| format!("Failed to open file: {}", e))?;
  let sheet_names = workbook.sheet_names();
  let first_sheet_name = &sheet_names[0];

  match workbook.worksheet_range(first_sheet_name) {
    Ok(range) => {
      let mut store_names = Vec::new();
      for row in range.rows() {
        if let Some(cell) = row.get(0) {
          if let Some(value) = cell.as_string() {
            store_names.push(value.to_string());
          }
        }
      }
      Ok(store_names)
    }
    Err(e) => Err(format!("Failed to read sheet: {}", e)),
  }
}