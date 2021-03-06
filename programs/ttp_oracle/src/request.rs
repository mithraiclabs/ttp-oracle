use crate::{ PUBLIC_KEY_LEN };
use solana_program::{
  pubkey::Pubkey,
  program_error::ProgramError,
  program_pack::{ IsInitialized, Pack, Sealed },
};
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use generic_array::{ 
  ArrayLength, 
  GenericArray,
  typenum::U34
};

pub type RequestIndex = u8;

pub const TASK_ARRAY_SIZE: usize = 3;
pub const REQUEST_QUEUE_SIZE: usize = 10;
pub const REQUEST_INDEX_SIZE: usize = 1;


#[derive(Clone, Debug, PartialEq)]
pub struct GetParams<N: ArrayLength<u8>> {
  pub get: GenericArray<u8, N> // 34 bytes of UTF 8 encoded data "https://ftx.us/api/markets/BTC/USD" for initial PoC
}

impl Sealed for GetParams<U34> {}
impl Pack for GetParams<U34> {
  const LEN: usize = 34;
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


#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
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

#[repr(C, u16)]
#[derive(Clone, Debug, PartialEq)]
pub enum Task {
  HttpGet(GetArgs),
  JsonParse(JsonParseArgs),
  Uint32,
}

impl Task {
  fn decode_task(kind: [u8; 2], data: [u8; 34]) -> Result<Self, ProgramError> {
    match u16::from_le_bytes(kind) {
      0 => Ok(Task::HttpGet(
        GetArgs::unpack_from_slice(&data)?
      )),
      1 => Ok(Task::JsonParse(JsonParseArgs::unpack_from_slice(&data)?)),
      2 => Ok(Task::Uint32),
      _ => Err(ProgramError::InvalidAccountData),
    }
  }

  fn encode_task(&self, kind: &mut [u8], task_data:&mut [u8])  {
    match self {
      Task::HttpGet(task) => {
        let tag: u16 = 0;
        kind.copy_from_slice(&tag.to_le_bytes()[0..2]);
        task.pack_into_slice(task_data);
      },
      Task::JsonParse(task) => {
        let tag: u16 = 1;
        kind.copy_from_slice(&tag.to_le_bytes()[0..2]);
        task.pack_into_slice(&mut task_data[0..12]);
      },
      Task::Uint32 => {
        let tag: u16 = 2;
        kind.copy_from_slice(&tag.to_le_bytes()[0..2]);
      }
      // TODO propogate error here?
    }
  }
}
impl Sealed for Task {}
impl Pack for Task {
  const LEN: usize  = 36;
  fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
    let src = array_ref![src, 0, 36];
    let (kind, task) = array_refs![src, 2, 34];
    return Task::decode_task(*kind, *task);
  }
   fn pack_into_slice(&self, dst: &mut [u8]) {
     let dst = array_mut_ref![dst, 0, 36];
     let (
        tag_dst,
        task_dst,
      ) = mut_array_refs![dst, 2, 34];
      self.encode_task(tag_dst, task_dst)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Request {
  // For phase 1 only 3 tasks are required
  // TODO allow more tasks to be added 
  pub tasks: [Task; TASK_ARRAY_SIZE], 
  pub call_back_program: Pubkey,
  pub index: RequestIndex,
}

impl Request {
  /// Check the first 8 bytes of Request buffer. If the first 8 bytes
  /// are 0, then the Request is determined empty.
  /// 
  /// This function must check at least the first 3 bytes, since the
  /// first two could be 0 as the initial Task enum tag.
  fn is_empty_buffer(buf: &[u8; Request::LEN]) -> bool {
    u64::from_le_bytes(*array_ref![buf, 0, 8]) == 0
  }

  
}

impl Sealed for Request {}
impl IsInitialized for Request {
  fn is_initialized(&self) -> bool {
      true
  }
}
impl Pack for Request {
  const LEN: usize  = Task::LEN * TASK_ARRAY_SIZE + PUBLIC_KEY_LEN + REQUEST_INDEX_SIZE;
  fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
    let src = array_ref![src, 0, Request::LEN];
    let (task_1, task_2, task_3, program_id_bytes, index_bytes) = 
      array_refs![src, Task::LEN, Task::LEN, Task::LEN, PUBLIC_KEY_LEN, REQUEST_INDEX_SIZE];
    let call_back_program = Pubkey::new(program_id_bytes);
    return Ok(Request {
      tasks: [
        Task::unpack_from_slice(task_1)?,
        Task::unpack_from_slice(task_2)?,
        Task::unpack_from_slice(task_3)?
      ],
      call_back_program: call_back_program,
      index: u8::from_le_bytes(*index_bytes)
    });
  }

  fn pack_into_slice(&self, dst: &mut [u8]) {
    let dst = array_mut_ref![dst, 0, Request::LEN];
    let (task_1, task_2, task_3, call_back_program, index) =
    mut_array_refs![dst, Task::LEN, Task::LEN, Task::LEN, PUBLIC_KEY_LEN, REQUEST_INDEX_SIZE];
    self.tasks[0].pack_into_slice(task_1);
    self.tasks[1].pack_into_slice(task_2);
    self.tasks[2].pack_into_slice(task_3);
    *call_back_program = self.call_back_program.to_bytes();
    index.copy_from_slice(&[self.index]);
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RequestQueue {
  pub requests: Box<[Option<Request>; REQUEST_QUEUE_SIZE]>,
}

impl Sealed for RequestQueue {}
impl IsInitialized for RequestQueue {
  fn is_initialized(&self) -> bool {
      true
  }
}
impl Pack for RequestQueue {
  const LEN: usize = Request::LEN * REQUEST_QUEUE_SIZE;

  fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
    let mut requests: Box<[Option<Request>; REQUEST_QUEUE_SIZE]> = Box::new([None, None, None, None, None, None, None, None, None, None]);
    for i in 0..REQUEST_QUEUE_SIZE {
      let offset = Request::LEN * i;
      let request_buf = array_ref![src, offset, Request::LEN];
      if !Request::is_empty_buffer(&request_buf) {
        let request = Request::unpack(array_ref![src, offset, Request::LEN])?;
        requests[i] = Some(request);
      }
    }

    Ok(RequestQueue {
      requests: requests
    })
  }

  fn pack_into_slice(&self, dst: &mut [u8]) {
    for i in 0..REQUEST_QUEUE_SIZE {
      if self.requests[i].is_none() {
        continue;
      }
      let request = self.requests[i].clone().unwrap();


      let offset = Request::LEN * i;
      let dst_request_buf = array_mut_ref![dst, offset, Request::LEN];
      // TODO maybe add better ProgramError here or in the Request::pack?
      Request::pack(request, dst_request_buf).unwrap();
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

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
      tasks: [get_task, json_parse_task, uint_128_task],
      call_back_program: Pubkey::new(&[4u8; PUBLIC_KEY_LEN]),
      index: 0,
    }
  }
  
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
  fn test_pack_unpack_task() {
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
    
    let httpget_tag = [0 as u8; 2];
    let httpjson_tag: [u8; 2] = [1, 0];
    let &mut mut serialized_get_task = &mut [0 as u8; 36];
    get_task.pack_into_slice(&mut serialized_get_task);
    let &mut mut serialized_json_task = &mut [0 as u8; 36];
    json_parse_task.pack_into_slice(&mut serialized_json_task);
    assert_eq!(serialized_get_task[0..2], httpget_tag);
    assert_eq!(serialized_get_task[2..36], *url_bytes);
    assert_eq!(serialized_json_task[0..2], httpjson_tag);
    assert_eq!(serialized_json_task[2..14], *path_bytes);

    let deserialized_get_task: Task = Task::unpack_from_slice(&serialized_get_task).unwrap();
    let deserialized_json_task: Task = Task::unpack_from_slice(&serialized_json_task).unwrap();
    assert_eq!(deserialized_get_task, get_task);
    assert_eq!(deserialized_json_task, json_parse_task);
  }

  #[test]
  fn test_request_is_empty() {
    let empty_request_buffer = [0u8; Request::LEN];
    assert!(Request::is_empty_buffer(&empty_request_buffer));

    let mut request_buffer = [0u8; Request::LEN];
    let request = create_sample_request();
    Request::pack(request, &mut request_buffer).unwrap();
    assert_eq!(Request::is_empty_buffer(&request_buffer), false);
  }

  #[test]
  fn test_pack_unpack_request() {
    let request = create_sample_request();
    let url_bytes = b"https://ftx.us/api/markets/BTC/USD";
    let path_bytes = b"result.price";
    let httpget_tag = [0 as u8; 2];
    let json_tag: [u8; 2] = [1, 0];
    let uint32_tag: [u8; 2] = [2, 0];

    let &mut mut serialized_request = &mut [0 as u8; Request::LEN];
    request.pack_into_slice(&mut serialized_request);
    assert_eq!(serialized_request[0..2], httpget_tag);
    assert_eq!(serialized_request[2..36], *url_bytes);
    assert_eq!(serialized_request[36..38], json_tag);
    assert_eq!(serialized_request[38..50], *path_bytes);
    assert_eq!(serialized_request[72..74], uint32_tag);

    let deserialized_request: Request = Request::unpack_from_slice(&serialized_request).unwrap();
    assert_eq!(deserialized_request, request);
  }

  #[test]
  fn test_pack_unpack_request_queue() {
    let mut empty_request_queue_buffer = [0u8; RequestQueue::LEN];
    let empty_request_queue = RequestQueue::unpack(&empty_request_queue_buffer).unwrap();
    
    assert_eq!(empty_request_queue, RequestQueue {
      requests: Box::new([None, None, None, None, None, None, None, None, None, None])
    });

    RequestQueue::pack(RequestQueue {
      requests: Box::new([None, None, None, None, None, None, None, None, None, None])
    }, &mut empty_request_queue_buffer).unwrap();

    assert_eq!(empty_request_queue_buffer, [0u8; RequestQueue::LEN]);

    let mut expected_single_request_queue_buffer = [0u8; RequestQueue::LEN];
    let request_queue_buffer = array_mut_ref![expected_single_request_queue_buffer, 0, RequestQueue::LEN];
    let (first_request, _rest, last_request) = mut_array_refs![request_queue_buffer, Request::LEN, RequestQueue::LEN - 2 * Request::LEN, Request::LEN];

    let request = create_sample_request();
    Request::pack(request, first_request).unwrap();
    let request = create_sample_request();
    Request::pack(request, last_request).unwrap();

    let single_queue = RequestQueue::unpack(&expected_single_request_queue_buffer).unwrap();
    let request1 = create_sample_request();
    let request2 = create_sample_request();
    assert_eq!(single_queue, RequestQueue {
      requests: Box::new([Some(request1), None, None, None, None, None, None, None, None, Some(request2)]),
    });
    
    let mut single_queue_buffer = [0u8; RequestQueue::LEN];
    let request1 = create_sample_request();
    let request2 = create_sample_request();
    RequestQueue::pack(RequestQueue {
      requests: Box::new([Some(request1), None, None, None, None, None, None, None, None, Some(request2)]),
    }, &mut single_queue_buffer).unwrap();

    assert_eq!(single_queue_buffer, expected_single_request_queue_buffer);
  }
}