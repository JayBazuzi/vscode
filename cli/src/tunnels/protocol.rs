/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/
use std::collections::HashMap;

use crate::options::Quality;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[serde(tag = "method", content = "params")]
#[allow(non_camel_case_types)]
pub enum ServerRequestMethod {
	/// Request from the client to start the VS Code server. It will download the
	/// requested version, if necessary.
	serve(ServeParams),
	/// Prunes unused servers on the CLI.
	prune,
	/// Empty ping/pong method used for liveness check.
	ping(EmptyResult),
	/// Forwards a port from the machine the CLI is running on.
	forward(ForwardParams),
	/// Stops forwarding a port from the machine the CLI is running on.
	unforward(UnforwardParams),
	/// Gets the hostname of the machine the CLI is running on.
	gethostname(EmptyResult),
	/// Checks for or applies an update to the CLI.
	update(UpdateParams),
	/// Sent when the remote instance of VS Code has a message for the server.
	servermsg(ServerMessageParams),
	/// Sent to make an http call on the local VS Code server.
	callserverhttp(CallServerHttpParams),
	/// Sent once with data in response to an `makehttpreq` from the server.
	httpheaders(HttpHeadersParams),
	/// Sent (repeatedly) with data in response to an `makehttpreq` from the server.
	httpbody(HttpBodyParams),
}

#[derive(Serialize, Debug)]
#[serde(tag = "method", content = "params", rename_all = "camelCase")]
#[allow(non_camel_case_types)]
pub enum ClientRequestMethod<'a> {
	servermsg(RefServerMessageParams<'a>),
	serverlog(ServerLog<'a>),
	makehttpreq(HttpRequestParams<'a>),
	version(VersionParams),
}

#[derive(Deserialize, Debug)]
pub struct HttpBodyParams {
	#[serde(with = "serde_bytes")]
	pub segment: Vec<u8>,
	pub complete: bool,
	pub req_id: u32,
}

#[derive(Serialize, Debug)]
pub struct HttpRequestParams<'a> {
	pub url: &'a str,
	pub method: &'static str,
	pub req_id: u32,
}

#[derive(Deserialize, Debug)]
pub struct HttpHeadersParams {
	pub status_code: u16,
	pub headers: Vec<(String, String)>,
	pub req_id: u32,
}

#[derive(Deserialize, Debug)]
pub struct ForwardParams {
	pub port: u16,
}

#[derive(Deserialize, Debug)]
pub struct UnforwardParams {
	pub port: u16,
}

#[derive(Serialize)]
pub struct ForwardResult {
	pub uri: String,
}

#[derive(Deserialize, Debug)]
pub struct ServeParams {
	pub socket_id: u16,
	pub commit_id: Option<String>,
	pub quality: Quality,
	pub extensions: Vec<String>,
	#[serde(default)]
	pub use_local_download: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct EmptyResult {}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateParams {
	pub do_update: bool,
}

#[derive(Deserialize, Debug)]
pub struct ServerMessageParams {
	pub i: u16,
	#[serde(with = "serde_bytes")]
	pub body: Vec<u8>,
}

#[derive(Serialize, Debug)]
pub struct RefServerMessageParams<'a> {
	pub i: u16,
	#[serde(with = "serde_bytes")]
	pub body: &'a [u8],
}

#[derive(Serialize)]
pub struct UpdateResult {
	pub up_to_date: bool,
	pub did_update: bool,
}

#[derive(Deserialize, Debug)]
pub struct ToServerRequest {
	pub id: Option<u32>,
	#[serde(flatten)]
	pub params: ServerRequestMethod,
}

#[derive(Serialize, Debug)]
pub struct ToClientRequest<'a> {
	pub id: Option<u32>,
	#[serde(flatten)]
	pub params: ClientRequestMethod<'a>,
}

#[derive(Serialize, Deserialize)]
pub struct SuccessResponse<T>
where
	T: Serialize,
{
	pub id: u32,
	pub result: T,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
	pub id: u32,
	pub error: ResponseError,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseError {
	pub code: i32,
	pub message: String,
}

#[derive(Debug, Default, Serialize)]
pub struct ServerLog<'a> {
	pub line: &'a str,
	pub level: u8,
}

#[derive(Serialize)]
pub struct GetHostnameResponse {
	pub value: String,
}

#[derive(Deserialize, Debug)]
pub struct CallServerHttpParams {
	pub path: String,
	pub method: String,
	pub headers: HashMap<String, String>,
	pub body: Option<Vec<u8>>,
}

#[derive(Serialize)]
pub struct CallServerHttpResult {
	pub status: u16,
	#[serde(with = "serde_bytes")]
	pub body: Vec<u8>,
	pub headers: HashMap<String, String>,
}

#[derive(Serialize, Debug)]
pub struct VersionParams {
	pub version: &'static str,
	pub protocol_version: u32,
}
