use crate::{
  request::{ RequestQueue }
};
use solana_program::{
  program_error::ProgramError,
  program_pack::{ IsInitialized, Pack, Sealed },
};

/// Struct representing the entire data buffer stored for each Oracle.
#[derive(Debug, PartialEq)]
pub struct OracleAccount {
  pub request_queue: RequestQueue,
}

impl Sealed for OracleAccount {}
impl IsInitialized for OracleAccount {
  fn is_initialized(&self) -> bool {
      true
  }
}
impl Pack for OracleAccount {
  const LEN: usize = RequestQueue::LEN;

  fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
    Ok(OracleAccount {
      request_queue: RequestQueue::unpack(src)?
    })
  }

  fn pack_into_slice(&self, dst: &mut [u8]) {
    let queue = self.request_queue.clone();
    RequestQueue::pack(queue, dst).unwrap();
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{
    PUBLIC_KEY_LEN,
    request::{ GetArgs, GetParams, JsonParseArgs, Request, Task }
  };
  use arrayref::{array_mut_ref, mut_array_refs};
  use generic_array::{
    GenericArray,
  };
  use solana_program::{
    pubkey::Pubkey,
  };

  fn create_sample_request() -> Request {
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
    let uint_128_task = Task::Uint32;

    Request {
      tasks: vec![get_task, json_parse_task, uint_128_task],
      call_back_program: Pubkey::new(&[4u8; PUBLIC_KEY_LEN]),
      num_accounts: 0,
      accounts: vec![],
      index: 0,
    }
  }

  #[test]
  fn test_oracle_account_pack_unpack() {
    let request = create_sample_request();
    let request_queue = RequestQueue {
      requests: Box::new([Some(request), None, None, None, None, None, None, None, None, None]),
    };
    let oracle_account = OracleAccount {
      request_queue,
    };
    let mut expected_oracle_account_buffer = [0u8; OracleAccount::LEN];
    let oracle_account_buffer = array_mut_ref![expected_oracle_account_buffer, 0, OracleAccount::LEN];
    let (first_request, _rest) = mut_array_refs![oracle_account_buffer, Request::LEN, RequestQueue::LEN - Request::LEN];
    let request = create_sample_request();
    Request::pack(request, first_request).unwrap();

    let mut oracle_account_buffer = [0u8; OracleAccount::LEN];
    OracleAccount::pack(oracle_account, &mut oracle_account_buffer).unwrap();
    assert_eq!(oracle_account_buffer, expected_oracle_account_buffer);
    
    let request = create_sample_request();
    let request_queue = RequestQueue {
      requests: Box::new([Some(request), None, None, None, None, None, None, None, None, None]),
    };
    let expected_oracle_account = OracleAccount {
      request_queue,
    };

    let oracle_account = OracleAccount::unpack(&oracle_account_buffer).unwrap();
    assert_eq!(oracle_account, expected_oracle_account);
  }
}
