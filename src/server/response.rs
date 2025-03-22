impl Response {
    pub(crate) fn response_as_string(&self) -> String {
        //self.response.to_string()
        String::from("")
    }
    pub(crate) fn is_stop_request(&self) -> bool {
        match self {
            Response::Stop{id: _}=> true,
            _ => false
        }
    }
}

pub(crate) enum Response {
    Stop{id: String},
    ParseError {error: serde_json::Error},
  	InvalidRequest {error: serde_json::Error},
}

// {
//                     is_stop_request: false,
//                     response: serde_json::json!({
//                         "jsonrpc": "2.0",
//                         "error": {
//                           "code": -32700,
//                           "message": format!("invalid json: {}", e),
//                         }

//                     }),
//                 };

// /**
//  * response::Response {
//                             is_stop_request: false,
//                             response: serde_json::json!({
//                                 "jsonrpc": "2.0",
//                                 "error": {
//                                 "code": -32600,
//                                 "message": format!("invalid request: {}", e),
//                                 }

//                             }),
//                         };
//  */