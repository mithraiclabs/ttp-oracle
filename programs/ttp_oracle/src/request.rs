#![cfg_attr(not(feature = "program"), allow(unused))]
use serde::{ Serialize, Deserialize };
use serde_big_array::{ big_array };

// Must list additional array sizes over 32 to support here
big_array! { BigArray; 34, 105, }

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GetParams {
  #[serde(with = "BigArray")]
  pub get: [u8; 34] // 34 bytes of UTF 8 encoded data "https://ftx.us/api/markets/BTC/USD" for initial PoC
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GetArgs {
  pub params: GetParams
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonParseArgs {
  pub path: [u8; 12] // 12 bytes of UTF 8 encoded data. "result.price" for initial PoC
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Task {
  HttpGet(GetArgs),
  JsonParse(JsonParseArgs),
  SolUint256
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Request {
  // For phase 1 only 3 tasks are required
  // TODO allow more tasks to be added 
  pub tasks: [Task; 3], 
  pub offset: u32
}


#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn test_serde_get_params() {
    let url_bytes = b"https://ftx.us/api/markets/BTC/USD";
    let params = GetParams {
      get: *url_bytes
    };
    let serialized_params = bincode::serialize(&params).unwrap();

    // make sure the serialized GetParams is the same as the url_bytes
    assert_eq!(serialized_params, url_bytes);

    let deserialized_params: GetParams = bincode::deserialize(&serialized_params).unwrap();

    assert_eq!(deserialized_params, params);
  }

  #[test]
  fn test_serde_json_parse_args() {
    let path_bytes = b"result.price";
    let json_args = JsonParseArgs {
      path: *path_bytes
    };
    let serialized_json_parse_args = bincode::serialize(&json_args).unwrap();

    assert_eq!(serialized_json_parse_args, path_bytes);

    let deserialized_json_parse_args: JsonParseArgs = bincode::deserialize(&serialized_json_parse_args).unwrap();

    assert_eq!(deserialized_json_parse_args, json_args);
  }

  #[test]
  fn test_serde_task() {
    let url_bytes = b"https://ftx.us/api/markets/BTC/USD";
    let path_bytes = b"result.price";
    let json_args = JsonParseArgs {
      path: *path_bytes
    };
    let params = GetParams {
      get: *url_bytes
    };
    let args = GetArgs {
      params: params
    };
    let get_task = Task::HttpGet(args);
    let json_parse_task = Task::JsonParse(json_args);
    
    let httpget_tag = [0 as u8; 4];
    let httpjson_tag: [u8; 4] = [1, 0, 0, 0];
    let serialized_get_task = bincode::serialize(&get_task).unwrap();
    let serialized_json_task = bincode::serialize(&json_parse_task).unwrap();
    assert_eq!(serialized_get_task[0..4], httpget_tag);
    assert_eq!(serialized_get_task[4..38], *url_bytes);
    assert_eq!(serialized_json_task[0..4], httpjson_tag);
    assert_eq!(serialized_json_task[4..16], *path_bytes);

    let deserialized_get_task: Task = bincode::deserialize(&serialized_get_task).unwrap();
    let deserialized_json_task: Task = bincode::deserialize(&serialized_json_task).unwrap();
    assert_eq!(deserialized_get_task, get_task);
    assert_eq!(deserialized_json_task, json_parse_task);
  }

  #[test]
  fn test_serde_request() {
    let url_bytes = b"https://ftx.us/api/markets/BTC/USD";
    let path_bytes = b"result.price";
    let json_args = JsonParseArgs {
      path: *path_bytes
    };
    let params = GetParams {
      get: *url_bytes
    };
    let args = GetArgs {
      params: params
    };
    let get_task = Task::HttpGet(args);
    let json_parse_task = Task::JsonParse(json_args);
    let uint_256_task = Task::SolUint256;
    let httpget_tag = [0 as u8; 4];
    let json_tag: [u8; 4] = [1, 0, 0, 0];
    let uint256_tag: [u8; 4] = [2, 0, 0, 0];

    let request = Request {
      tasks: [get_task, json_parse_task, uint_256_task],
      offset: 0
    };

    let serialized_request = bincode::serialize(&request).unwrap();   
    assert_eq!(serialized_request[0..4], httpget_tag);
    assert_eq!(serialized_request[4..38], *url_bytes);
    assert_eq!(serialized_request[38..42], json_tag);
    assert_eq!(serialized_request[42..54], *path_bytes);
    assert_eq!(serialized_request[54..58], uint256_tag);

    let deserialized_request: Request = bincode::deserialize(&serialized_request).unwrap();
    assert_eq!(deserialized_request, request);
  }
}