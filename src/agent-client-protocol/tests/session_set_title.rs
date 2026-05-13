use agent_client_protocol::{
    JsonRpcMessage, JsonRpcRequest, JsonRpcResponse,
    schema::{ClientRequest, SetSessionTitleRequest, SetSessionTitleResponse},
};

#[test]
fn set_session_title_request_has_json_rpc_method() {
    let request = SetSessionTitleRequest::new("sess_1", "Readable title");

    assert!(SetSessionTitleRequest::matches_method("session/setTitle"));
    assert_eq!(request.method(), "session/setTitle");
    assert_eq!(
        <SetSessionTitleRequest as JsonRpcRequest>::Response::new(),
        SetSessionTitleResponse::new()
    );
}

#[test]
fn client_request_dispatches_session_set_title() {
    let request = SetSessionTitleRequest::new("sess_1", "Readable title");
    let message = ClientRequest::SetSessionTitleRequest(request);

    assert_eq!(message.method(), "session/setTitle");

    let parsed = ClientRequest::parse_message(
        "session/setTitle",
        &serde_json::json!({
            "sessionId": "sess_1",
            "title": "Readable title"
        }),
    )
    .expect("session/setTitle should parse through ClientRequest");

    assert!(matches!(parsed, ClientRequest::SetSessionTitleRequest(_)));
}

#[test]
fn set_session_title_response_roundtrips_json_rpc_response() {
    let response = SetSessionTitleResponse::new();
    let json = response
        .clone()
        .into_json("session/setTitle")
        .expect("response should serialize");

    assert_eq!(json, serde_json::json!({}));
    assert_eq!(
        SetSessionTitleResponse::from_value("session/setTitle", json)
            .expect("response should deserialize"),
        response
    );
}
