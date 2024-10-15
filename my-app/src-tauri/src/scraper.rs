use std::sync::{Arc, Mutex};
use std::time::Duration;
use headless_chrome::{Browser, LaunchOptions, Tab};
use polars_excel_writer::PolarsXlsxWriter;
use regex::Regex;
use tauri::api::dialog::{FileDialogBuilder, MessageDialogBuilder};
use tauri::api::path::desktop_dir;
use polars::prelude::*;
use std::fs::File;
use std::io::{self, Write};

#[tauri::command]
pub async fn scraper(store_names: Vec<String>, headless: bool) -> Result<(), String> {
  let browser = Browser::new(LaunchOptions {
    headless,
    window_size: Some((1280, 1280)),
    ..Default::default()
  }).map_err(|e| e.to_string())?;

  let tab = browser.new_tab().map_err(|e| e.to_string())?;
  // login
  set_up(&tab)?;

  // 空のDataframeを生成 df_result
  let mut df_result = DataFrame::new(vec![
    Series::new("date".into(), Vec::<String>::new())
    ]).map_err(|e| e.to_string())?;
  
  let mut error_stores = Vec::new();

  for (index, store_name) in store_names.iter().enumerate() {
    // チーム選択
    let search_char = retrieve_search_string(&store_name);
    println!("{}", &search_char);
    tab.wait_for_element(r#"input[type="search"]"#)
      .map_err(|e| e.to_string())?
      .type_into(&search_char)
      .map_err(|e| e.to_string())?;

    let ns_button = match tab.wait_for_element_with_custom_timeout(
      r#"div[department_name="ネットスーパー"]"#,
      Duration::new(2, 0)
    ) {
      Ok(ns_button) => ns_button,
      Err(_) => {
        fill_input_field_faster(&tab, r#"input[type="search"]"#, "").map_err(|e| e.to_string())?;
        error_stores.push(store_name.to_string());
        continue;
      }
    };
    ns_button.click().map_err(|e| e.to_string())?;

    let shift_button = match tab.wait_for_element_with_custom_timeout(
      r#"a[href="/shift"]"#,
      Duration::new(2, 0)
    ) {
      Ok(shift_button) => shift_button,
      Err(_) => {
        fill_input_field_faster(&tab, r#"input[type="search"]"#, "").map_err(|e| e.to_string())?;
        error_stores.push(store_name.to_string());
        continue;
      }
    };
    shift_button.click().map_err(|e| e.to_string())?;

    // 実績計画抽出
    let date_list = match get_text_content_from_elements(&tab, "td.col-header.dateDisplay") {
      Ok(date_list) => date_list,
      Err(_) => {
        fill_input_field_faster(&tab, r#"input[type="search"]"#, "").map_err(|e| e.to_string())?;
        error_stores.push(store_name.to_string());
        continue;
      }
    };
    let daysum_list = match get_text_content_from_elements(&tab, "td.day-sum") {
      Ok(daysum_list) => daysum_list,
      Err(_) => {
        fill_input_field_faster(&tab, r#"input[type="search"]"#, "").map_err(|e| e.to_string())?;
        error_stores.push(store_name.to_string());
        continue;
      }
    };
    let (required, plan) = extract_numbers_from_strings(daysum_list)?;

    // Dataframeを作成して、df_resultと日付を元にjoin
    let df = match create_dataframe(date_list, required, plan, &store_name) {
      Ok(df) => df,
      Err(_) => {
        fill_input_field_faster(&tab, r#"input[type="search"]"#, "").map_err(|e| e.to_string())?;
        error_stores.push(store_name.to_string());
        continue;
      }
    };

    if df_result.height() == 0 {
      df_result = df;
    } else {
      df_result = df_result.join(&df, ["日付"], ["日付"], JoinType::Inner.into()).map_err(|e| e.to_string())?;
    }
  
    // 次ページへ　enumerateとlengthを比較して同じときは実行しないようにする
    if index != store_names.len()-1 {
      js_clicker(&tab, "i.bars")?;
      js_clicker(&tab, r#"a[href="/department/select"]"#)?;
    }
  }

  println!("{:?}", df_result);
  let desktop_path = get_desktop_path_or_prompt();
  if let Some(ref path) = desktop_path {
    let df_path = format!("{}\\output.xlsx", path);
    println!("{}", df_path);
    df_to_excel(&df_result, &df_path).map_err(|e| e.to_string())?;
    if !error_stores.is_empty() {
      let error_path = format!("{}\\error-log.txt", path);
      write_vec_to_file(&error_stores, &error_path).map_err(|e| e.to_string())?;
      println!("{:?}", error_stores);
    }
  } else {
    MessageDialogBuilder::new("保存場所取得失敗", "保存する場所を正しく指定してください")
      .show(|response| {
        if response {
          println!("User clicked OK");
        } else {
          println!("User clicked Cancel");
        }
      }
    );
  }

  Ok(())
}

fn set_up(tab: &Arc<Tab>) -> Result<(), String>{
  tab.navigate_to("https://work-opt.jpn.panasonic.com/login").map_err(|e| e.to_string())?;

  fill_input_field_faster(
    &tab,
    r#"input[placeholder="企業コード"]"#,
    "0200701645"
  )?;

  fill_input_field_faster(
    &tab,
    r#"input[placeholder="メールアドレス"]"#,
    "watanabe-tsuba@aeonpeople.biz"
  )?;
  
  fill_input_field_faster(
    &tab,
    r#"input[placeholder="パスワード"]"#,
    "cRUpUqeUVM6Xbh8"
  )?;

  tab.wait_for_element("button.ui.blue")
    .map_err(|e| e.to_string())?
    .click()
    .map_err(|e| e.to_string())?;
  
  // チーム設定
  js_clicker(&tab, r#"a[href="/company/department/select"]"#)?;
  tab.wait_until_navigated()
    .map_err(|e|  e.to_string())?;

  Ok(())
}

fn retrieve_search_string(store_name: &str) -> String {
  let prefix: String = store_name.chars().take(4).collect();
  format!("{} ネット", prefix)
}

fn fill_input_field_faster(tab: &Arc<Tab>, selector: &str, value: &str) -> Result<(), String> {
  tab.wait_for_element(selector)
    .map_err(|e| e.to_string())?
    .call_js_fn(&format!(
      r#"
        function inputCode () {{
          document.querySelectorAll('{}')[0].value = '{}';
        }}
      "#,
      selector, value
    ), vec![], true)
    .map_err(|e| e.to_string())?;
  
  tab.find_element(selector)
    .map_err(|e| e.to_string())?
    .type_into(" ")
    .map_err(|e| e.to_string())?;
  tab.press_key("Backspace")
    .map_err(|e| e.to_string())?;
  
  Ok(())
}

fn js_clicker(tab: &Arc<Tab>, selector: &str) -> Result<(), String> {
  // 要素を待つ部分のエラーハンドリング
  tab.wait_for_element(selector)
    .map_err(|e| format!("Error waiting for element '{}': {}", selector, e.to_string()))?
    .call_js_fn(
      &format!(
        r#"
          function jsClicker () {{
            document.querySelectorAll('{}')[0].click();
          }}
        "#, 
        selector
      ), 
      vec![], true
    )
    // JavaScript 関数呼び出し部分のエラーハンドリング
    .map_err(|e| format!("Error executing jsClicker for '{}': {}", selector, e.to_string()))?;
  Ok(())
}

fn get_text_content_from_elements(tab: &Arc<Tab>, selector: &str) -> Result<Vec<String>, String> {
  // Find elements matching the selector
  let elements = tab
    .wait_for_elements(selector)
    .map_err(|e| e.to_string())?;

  // Extract the text content from each element
  let mut text_contents = Vec::new();
  for element in elements {
    let text = element
      .get_inner_text()
      .map_err(|e| e.to_string())?;
    text_contents.push(text);
  }

  Ok(text_contents)
}

fn extract_numbers_from_strings(data: Vec<String>) -> Result<(Vec<f64>, Vec<f64>), String> {
  // 正規表現パターンを作成
  let re = Regex::new(r"必要：(?P<required>\d+(\.\d+)?)\s*計画：(?P<plan>\d+(\.\d+)?)")
    .map_err(|e| e.to_string())?;

  let mut required_numbers = Vec::new();
  let mut plan_numbers = Vec::new();

  // 各文字列を解析
  for entry in data {
    if let Some(captures) = re.captures(&entry) {
      // 必要の後の数字を抽出してベクターに追加
      if let Some(required_match) = captures.name("required") {
        let required_value: f64 = required_match.as_str().parse().map_err(|e: std::num::ParseFloatError| e.to_string())?;
        required_numbers.push(required_value);
      }

      // 計画の後の数字を抽出してベクターに追加
      if let Some(plan_match) = captures.name("plan") {
        let plan_value: f64 = plan_match.as_str().parse().map_err(|e: std::num::ParseFloatError| e.to_string())?;
        plan_numbers.push(plan_value);
      }
    } else {
      return Err(format!("Failed to parse entry: {}", entry));
    }
  }

  Ok((required_numbers, plan_numbers))
}

fn get_desktop_path () -> Option<String> {
  desktop_dir().map(|path| path.to_string_lossy().to_string())
}

fn create_dataframe(
  date_list: Vec<String>,
  required: Vec<f64>,
  plan: Vec<f64>,
  store_name: &str
) -> Result<DataFrame, String> {
  let data_series = Series::new("日付".into(), date_list);
  let required_series = Series::new(format!("必要_{}", store_name).into(), required);
  let plan_series = Series::new(format!("計画_{}", store_name).into(), plan);
  let df = DataFrame::new(vec![data_series, required_series, plan_series]).map_err(|e| e.to_string())?;

  Ok(df)
}

fn df_to_excel(df: &DataFrame, path: &str) -> PolarsResult<()> {
  let mut xlsx_writer = PolarsXlsxWriter::new();
  xlsx_writer.set_freeze_panes(1, 1);
  xlsx_writer.write_dataframe(df)?;
  xlsx_writer.save(path)?;

  Ok(())
}

fn write_vec_to_file(vec: &Vec<String>, file_path: &str) -> io::Result<()> {
  let mut file = File::create(file_path)?;  // ファイルを作成

  // Vec内の要素を1行ずつ書き込む
  for line in vec {
    writeln!(file, "{}", line)?;  // writeln!は自動的に改行を追加
  }

  Ok(())
}

fn get_desktop_path_or_prompt() -> Option<String> {
  // desktop_path が Some か None かを確認
  match get_desktop_path() {
    Some(path) => Some(path),  // 既にあるパスを返す
    None => {
      // 選択されたパスを保持するための変数を用意
      let selected_path = Arc::new(Mutex::new(None));

      // クロージャにパスを渡すため、Arc<Mutex<Option<String>>> を使う
      let selected_path_clone: Arc<Mutex<Option<String>>> = Arc::clone(&selected_path);

      FileDialogBuilder::new()
        .set_title("Choose output folder")
        .pick_folder(move |folder_path| {
          if let Some(path) = folder_path {
            let mut selected_path = selected_path_clone.lock().unwrap();
            *selected_path = Some(path.display().to_string());
          }
        });

      // ロックして選択されたパスを返す
      let result = selected_path.lock().unwrap().clone();
      result
    }
  }
}
