#![cfg_attr(not(feature = "program"), allow(unused))]
use solana_sdk::{
  program_error::ProgramError,
  program_pack::{ Pack, Sealed },
};
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use generic_array::typenum::U34;
use generic_array::{ArrayLength, GenericArray};
use serde::{ Serialize, Deserialize };

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(bound = "N: ArrayLength<u8>")]
pub struct GetParams<N: ArrayLength<u8>> {
  pub get: GenericArray<u8, N> // 34 bytes of UTF 8 encoded data "https://ftx.us/api/markets/BTC/USD" for initial PoC
}

impl Sealed for GetParams<U34> {}
impl Pack for GetParams<U34> {
  const LEN: usize  = 34;
  fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
    let get = src;
    Ok(GetParams {
        get: *GenericArray::from_slice(get),
    })
  }
   fn pack_into_slice(&self, dst: &mut [u8]) {
        dst.copy_from_slice(self.get.as_slice());
    }
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GetArgs {
  pub params: GetParams<U34>
}
impl Sealed for GetArgs {}
impl Pack for GetArgs {
  const LEN: usize  = 34;
  fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
    let params = GetParams::unpack_from_slice(src)?;
    Ok(GetArgs {
        params: params
    })
  }
   fn pack_into_slice(&self, dst: &mut [u8]) {
        dst.copy_from_slice(self.params.get.as_slice());
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonParseArgs {
  pub path: [u8; 12] // 12 bytes of UTF 8 encoded data. "result.price" for initial PoC
}

impl Sealed for JsonParseArgs {}
impl Pack for JsonParseArgs {
  const LEN: usize  = 12;
  fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
    Ok(JsonParseArgs {
        path: *array_ref![src, 0, 12],
    })
  }
   fn pack_into_slice(&self, dst: &mut [u8]) {
        dst.copy_from_slice(&self.path[0..12]);
    }
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
  fn test_pack_unpack_get_params() {
    let url_bytes = b"https://ftx.us/api/markets/BTC/USD";
    let params = GetParams {
      get: *GenericArray::from_slice(url_bytes)
    };

    let &mut mut serialized_params = &mut [0 as u8; 34];
    params.pack_into_slice(&mut serialized_params);

    // make sure the serialized GetParams is the same as the url_bytes
    assert_eq!(&serialized_params, url_bytes);

    let deserialized_params: GetParams<U34> = GetParams::unpack_from_slice(&serialized_params).unwrap();

    assert_eq!(deserialized_params, params);
  }

  #[test]
  fn test_pack_unpack_json_parse_args() {
    let path_bytes = b"result.price";
    let json_args = JsonParseArgs {
      path: *path_bytes
    };
    let &mut mut serialized_json_parse_args = &mut [0 as u8; 12];
    json_args.pack_into_slice(&mut serialized_json_parse_args);

    assert_eq!(&serialized_json_parse_args, path_bytes);

    let deserialized_json_parse_args: JsonParseArgs = JsonParseArgs::unpack_from_slice(&serialized_json_parse_args).unwrap();

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
      get: *GenericArray::from_slice(url_bytes)
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
      get: *GenericArray::from_slice(url_bytes)
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