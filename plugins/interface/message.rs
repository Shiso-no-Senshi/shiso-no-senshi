pub struct Header {
	pub content_length: u64,
	pub content_type: String
}

pub struct ClientCapabilities {}

pub enum ClientCapability {
	RegularExpressions(engine: RegexEngine, version: Option<String>)
}

pub enum RegexEngine {
	ECMAScript,
	Rust,
}

pub struct ServerCapability {}

pub struct RegistrationOptions {}

pub struct Request {
	header: Header,
	id: String,
	idIsNumber: bool,
	method: String,
	params: Option<String>,
}

pub struct Response {
	header: Header,
	id: String,
	isIsNumber: bool,
	result: Option<String>,
	error: Option<ResponseError>,
}

pub struct ResponseError {
	code: ResponseErrorCode,
	message: String,
	data: Option<String>
}

pub enum ResponseErrorCode {
	ParseError=-32700,
	InvalidRequest = -32600,
	MethodNotFound = -32601,
	InvalidParams = -32603,
	ServerNotInitialized = -32002,
	UnknownErrorCode = -32001
	RequestFailed = -32803,
	ServerCancelled = -32802,
	ContentModified = -32801,
	RequestCancelled = -32800,
}

pub struct Notification {
	header: Header,
	method: String,
	params: Option<String>
}

pub struct CancelRequest {
	header: Header,
	id: String,
}

pub struct ProgressNotification {
	header: Header,
	token: String,
	value: u8,
}