use std::borrow::Cow;

use deno_core::{error::AnyError, include_js_files, op, Extension, anyhow::bail, Op};
use scryer_prolog::machine::parsed_results::{Value, QueryMatch, QueryResolution};

use super::get_prolog_service;

#[op]
async fn spawn_engine(engine_name: String) -> Result<(), AnyError> {
    let mut service = get_prolog_service().await;
    service.spawn_engine(engine_name).await
}

#[op]
async fn remove_engine(engine_name: String) -> Result<(), AnyError> {
    let mut service = get_prolog_service().await;
    service.remove_engine(engine_name).await
}


pub fn prolog_value_to_json_tring(value: Value) -> String {
    match value {
        Value::Integer(i) => format!("{}", i),
        Value::Float(f) => format!("{}", f),
        Value::Rational(r) => format!("{}", r),
        Value::Atom(a) => format!("{}", a.as_str()),
        Value::String(s) => 
            if let Err(_e) = serde_json::from_str::<serde_json::Value>(s.as_str()) {
                //treat as string literal
                //escape double quotes
                format!("\"{}\"", s
                    .replace("\"", "\\\"")
                    .replace("\n", "\\n")
                    .replace("\t", "\\t")
                    .replace("\r", "\\r"))
            } else {
                //return valid json string
                s
            },
        Value::List(l) => {
            let mut string_result = "[".to_string();
            for (i, v) in l.iter().enumerate() {
                if i > 0 {
                    string_result.push_str(", ");
                }
                string_result.push_str(&prolog_value_to_json_tring(v.clone()));
            }
            string_result.push_str("]");
            string_result
        }
        Value::Structure(s, l) => {
            let mut string_result = format!("\"{}\": [", s.as_str());
            for (i, v) in l.iter().enumerate() {
                if i > 0 {
                    string_result.push_str(", ");
                }
                string_result.push_str(&prolog_value_to_json_tring(v.clone()));
            }
            string_result.push_str("]");
            string_result
        }
        _ => "null".to_string(),
    }
}

fn prolog_match_to_json_string(query_match: &QueryMatch) -> String {
    let mut string_result = "{".to_string();
    for (i, (k, v)) in query_match.bindings.iter().enumerate() {
        if i > 0 {
            string_result.push_str(", ");
        }
        string_result.push_str(&format!("\"{}\": {}", k, prolog_value_to_json_tring(v.clone())));
    }
    string_result.push_str("}");
    string_result
}

#[op]
async fn run_query(engine_name: String, query: String) -> Result<String, AnyError> {
    let service = get_prolog_service().await;
    let result = service
        .run_query(engine_name, query)
        .await?;

    if let Err(prolog_error) = result {
        bail!(prolog_error);
    }

    match result.unwrap() {
        QueryResolution::True => Ok("true".to_string()),
        QueryResolution::False => Ok("false".to_string()),
        QueryResolution::Matches(matches) => {
            let matches_json: Vec<String> = matches
                .iter()
                .map(|m| prolog_match_to_json_string(m))
                .collect();
            Ok(format!("[{}]", matches_json.join(", ")))
        }
    }
}

#[op]
async fn load_module_string(
    engine_name: String,
    module_name: String,
    program_lines: Vec<String>,
) -> Result<(), AnyError> {
    let service = get_prolog_service().await;
    service
        .load_module_string(engine_name, module_name, program_lines)
        .await
}

pub fn build() -> Extension {
    Extension {
        name: "prolog_service",
        js_files: Cow::Borrowed(&include_js_files!(holochain_service "src/prolog_service/prolog_service_extension.js",)),
        ops: Cow::Borrowed(&[
            spawn_engine::DECL,
            remove_engine::DECL,
            run_query::DECL,
            load_module_string::DECL,
        ]),
        ..Default::default()
    }
}
