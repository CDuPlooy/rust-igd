use std;
use std::error;
use std::fmt;
use std::io;
use std::str;
#[cfg(feature = "aio")]
use std::string::FromUtf8Error;

use attohttpc;
#[cfg(feature = "aio")]
use hyper;

/// Errors that can occur when sending the request to the gateway.
#[derive(Debug)]
pub enum RequestError {
    /// attohttp error
    AttoHttpError(attohttpc::Error),
    /// IO Error
    IoError(io::Error),
    /// The response from the gateway could not be parsed.
    InvalidResponse(String),
    /// The gateway returned an unhandled error code and description.
    ErrorCode(u16, String),
    /// When using the aio feature.
    #[cfg(feature = "aio")]
    HyperError(hyper::Error),

    #[cfg(feature = "aio")]
    /// http crate error type
    HttpError(http::Error),

    #[cfg(feature = "aio")]
    /// Error parsing HTTP body
    Utf8Error(FromUtf8Error),
}

impl From<attohttpc::Error> for RequestError {
    fn from(err: attohttpc::Error) -> RequestError {
        RequestError::AttoHttpError(err)
    }
}

impl From<io::Error> for RequestError {
    fn from(err: io::Error) -> RequestError {
        RequestError::IoError(err)
    }
}

#[cfg(feature = "aio")]
impl From<http::Error> for RequestError {
    fn from(err: http::Error) -> RequestError {
        RequestError::HttpError(err)
    }
}

#[cfg(feature = "aio")]
impl From<hyper::Error> for RequestError {
    fn from(err: hyper::Error) -> RequestError {
        RequestError::HyperError(err)
    }
}

#[cfg(feature = "aio")]
impl From<FromUtf8Error> for RequestError {
    fn from(err: FromUtf8Error) -> RequestError {
        RequestError::Utf8Error(err)
    }
}

#[cfg(feature = "aio")]
impl From<tokio::timer::Error> for RequestError {
    fn from(_err: tokio::timer::Error) -> RequestError {
        RequestError::IoError(io::Error::new(io::ErrorKind::TimedOut, "timer failed"))
    }
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RequestError::AttoHttpError(ref e) => write!(f, "HTTP error {}", e),
            RequestError::InvalidResponse(ref e) => write!(f, "Invalid response from gateway: {}", e),
            RequestError::IoError(ref e) => write!(f, "IO error. {}", e),
            RequestError::ErrorCode(n, ref e) => write!(f, "Gateway response error {}: {}", n, e),
            #[cfg(feature = "aio")]
            RequestError::HyperError(ref e) => write!(f, "Hyper Error: {}", e),
            #[cfg(feature = "aio")]
            RequestError::HttpError(ref e) => write!(f, "Http  Error: {}", e),
            #[cfg(feature = "aio")]
            RequestError::Utf8Error(ref e) => write!(f, "Utf8Error Error: {}", e),
        }
    }
}

impl std::error::Error for RequestError {
    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            RequestError::AttoHttpError(ref e) => Some(e),
            RequestError::InvalidResponse(..) => None,
            RequestError::IoError(ref e) => Some(e),
            RequestError::ErrorCode(..) => None,
            #[cfg(feature = "aio")]
            RequestError::HyperError(ref e) => Some(e),
            #[cfg(feature = "aio")]
            RequestError::HttpError(ref e) => Some(e),
            #[cfg(feature = "aio")]
            RequestError::Utf8Error(ref e) => Some(e),
        }
    }

    fn description(&self) -> &str {
        match *self {
            RequestError::AttoHttpError(..) => "Http error",
            RequestError::InvalidResponse(..) => "Invalid response",
            RequestError::IoError(..) => "IO error",
            RequestError::ErrorCode(_, ref e) => &e[..],
            #[cfg(feature = "aio")]
            RequestError::HyperError(_) => "Hyper Error",
            #[cfg(feature = "aio")]
            RequestError::HttpError(_) => "Http Error",
            #[cfg(feature = "aio")]
            RequestError::Utf8Error(_) => "UTF8 Error",
        }
    }
}

/// Errors returned by `Gateway::get_external_ip`
#[derive(Debug)]
pub enum GetExternalIpError {
    /// The client is not authorized to perform the operation.
    ActionNotAuthorized,
    /// Some other error occured performing the request.
    RequestError(RequestError),
}

/// Errors returned by `Gateway::remove_port`
#[derive(Debug)]
pub enum RemovePortError {
    /// The client is not authorized to perform the operation.
    ActionNotAuthorized,
    /// No such port mapping.
    NoSuchPortMapping,
    /// Some other error occured performing the request.
    RequestError(RequestError),
}

/// Errors returned by `Gateway::add_any_port` and `Gateway::get_any_address`
#[derive(Debug)]
pub enum AddAnyPortError {
    /// The client is not authorized to perform the operation.
    ActionNotAuthorized,
    /// Can not add a mapping for local port 0.
    InternalPortZeroInvalid,
    /// The gateway does not have any free ports.
    NoPortsAvailable,
    /// The gateway can only map internal ports to same-numbered external ports
    /// and this external port is in use.
    ExternalPortInUse,
    /// The gateway only supports permanent leases (ie. a `lease_duration` of 0).
    OnlyPermanentLeasesSupported,
    /// The description was too long for the gateway to handle.
    DescriptionTooLong,
    /// Some other error occured performing the request.
    RequestError(RequestError),
}

impl From<RequestError> for AddAnyPortError {
    fn from(err: RequestError) -> AddAnyPortError {
        AddAnyPortError::RequestError(err)
    }
}

impl From<GetExternalIpError> for AddAnyPortError {
    fn from(err: GetExternalIpError) -> AddAnyPortError {
        match err {
            GetExternalIpError::ActionNotAuthorized => AddAnyPortError::ActionNotAuthorized,
            GetExternalIpError::RequestError(e) => AddAnyPortError::RequestError(e),
        }
    }
}

/// Errors returned by `Gateway::add_port`
#[derive(Debug)]
pub enum AddPortError {
    /// The client is not authorized to perform the operation.
    ActionNotAuthorized,
    /// Can not add a mapping for local port 0.
    InternalPortZeroInvalid,
    /// External port number 0 (any port) is considered invalid by the gateway.
    ExternalPortZeroInvalid,
    /// The requested mapping conflicts with a mapping assigned to another client.
    PortInUse,
    /// The gateway requires that the requested internal and external ports are the same.
    SamePortValuesRequired,
    /// The gateway only supports permanent leases (ie. a `lease_duration` of 0).
    OnlyPermanentLeasesSupported,
    /// The description was too long for the gateway to handle.
    DescriptionTooLong,
    /// Some other error occured performing the request.
    RequestError(RequestError),
}

impl fmt::Display for GetExternalIpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetExternalIpError::ActionNotAuthorized => write!(f, "The client is not authorized to remove the port"),
            GetExternalIpError::RequestError(ref e) => write!(f, "Request Error. {}", e),
        }
    }
}

impl From<io::Error> for GetExternalIpError {
    fn from(err: io::Error) -> GetExternalIpError {
        GetExternalIpError::RequestError(RequestError::from(err))
    }
}

impl std::error::Error for GetExternalIpError {
    fn cause(&self) -> Option<&std::error::Error> {
        None
    }

    fn description(&self) -> &str {
        match *self {
            GetExternalIpError::ActionNotAuthorized => "The client is not authorized to remove the port",
            GetExternalIpError::RequestError(..) => "Request error",
        }
    }
}

impl fmt::Display for RemovePortError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RemovePortError::ActionNotAuthorized => write!(f, "The client is not authorized to remove the port"),
            RemovePortError::NoSuchPortMapping => write!(f, "The port was not mapped"),
            RemovePortError::RequestError(ref e) => write!(f, "Request error. {}", e),
        }
    }
}

impl std::error::Error for RemovePortError {
    fn cause(&self) -> Option<&std::error::Error> {
        None
    }

    fn description(&self) -> &str {
        match *self {
            RemovePortError::ActionNotAuthorized => "The client is not authorized to remove the port",
            RemovePortError::NoSuchPortMapping => "The port was not mapped",
            RemovePortError::RequestError(..) => "Request error",
        }
    }
}

impl fmt::Display for AddAnyPortError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddAnyPortError::ActionNotAuthorized => {
                write!(f, "The client is not authorized to remove the port")
            }
            AddAnyPortError::InternalPortZeroInvalid => {
                write!(f, "Can not add a mapping for local port 0")
            }
            AddAnyPortError::NoPortsAvailable => {
                write!(f, "The gateway does not have any free ports")
            }
            AddAnyPortError::OnlyPermanentLeasesSupported => {
                write!(
                    f,
                    "The gateway only supports permanent leases (ie. a `lease_duration` of 0),"
                )
            }
            AddAnyPortError::ExternalPortInUse => {
                write!(
                    f,
                    "The gateway can only map internal ports to same-numbered external ports and this external port is in use."
                )
            }
            AddAnyPortError::DescriptionTooLong => {
                write!(f, "The description was too long for the gateway to handle.")
            }
            AddAnyPortError::RequestError(ref e) => write!(f, "Request error. {}", e),
        }
    }
}

impl std::error::Error for AddAnyPortError {
    fn cause(&self) -> Option<&std::error::Error> {
        None
    }

    fn description(&self) -> &str {
        match *self {
            AddAnyPortError::ActionNotAuthorized => {
                "The client is not authorized to remove the port"
            }
            AddAnyPortError::InternalPortZeroInvalid => "Can not add a mapping for local port 0.",
            AddAnyPortError::NoPortsAvailable => "The gateway does not have any free ports",
            AddAnyPortError::OnlyPermanentLeasesSupported => {
                "The gateway only supports permanent leases (ie. a `lease_duration` of 0),"
            }
            AddAnyPortError::ExternalPortInUse => {
                "The gateway can only map internal ports to same-numbered external ports and this external port is in use."
            }
            AddAnyPortError::DescriptionTooLong => {
                "The description was too long for the gateway to handle."
            }
            AddAnyPortError::RequestError(..) => "Request error",
        }
    }
}

impl fmt::Display for AddPortError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddPortError::ActionNotAuthorized => write!(f, "The client is not authorized to map this port."),
            AddPortError::InternalPortZeroInvalid => write!(f, "Can not add a mapping for local port 0"),
            AddPortError::ExternalPortZeroInvalid => write!(
                f,
                "External port number 0 (any port) is considered invalid by the gateway."
            ),
            AddPortError::PortInUse => write!(
                f,
                "The requested mapping conflicts with a mapping assigned to another client."
            ),
            AddPortError::SamePortValuesRequired => write!(
                f,
                "The gateway requires that the requested internal and external ports are the same."
            ),
            AddPortError::OnlyPermanentLeasesSupported => write!(
                f,
                "The gateway only supports permanent leases (ie. a `lease_duration` of 0),"
            ),
            AddPortError::DescriptionTooLong => write!(f, "The description was too long for the gateway to handle."),
            AddPortError::RequestError(ref e) => write!(f, "Request error. {}", e),
        }
    }
}

impl std::error::Error for AddPortError {
    fn cause(&self) -> Option<&std::error::Error> {
        None
    }

    fn description(&self) -> &str {
        match *self {
            AddPortError::ActionNotAuthorized => "The client is not authorized to map this port.",
            AddPortError::InternalPortZeroInvalid => "Can not add a mapping for local port 0",
            AddPortError::ExternalPortZeroInvalid => {
                "External port number 0 (any port) is considered invalid by the gateway."
            }
            AddPortError::PortInUse => "The requested mapping conflicts with a mapping assigned to another client.",
            AddPortError::SamePortValuesRequired => {
                "The gateway requires that the requested internal and external ports are the same."
            }
            AddPortError::OnlyPermanentLeasesSupported => {
                "The gateway only supports permanent leases (ie. a `lease_duration` of 0),"
            }
            AddPortError::DescriptionTooLong => "The description was too long for the gateway to handle.",
            AddPortError::RequestError(..) => "Request error",
        }
    }
}

/// Errors than can occur while trying to find the gateway.
#[derive(Debug)]
pub enum SearchError {
    /// Http/Hyper error
    HttpError(attohttpc::Error),
    /// Unable to process the response
    InvalidResponse,
    /// IO Error
    IoError(io::Error),
    /// UTF-8 decoding error
    Utf8Error(str::Utf8Error),
    /// XML processing error
    XmlError(xmltree::ParseError),
    /// When using the aio feature.
    #[cfg(feature = "aio")]
    HyperError(hyper::Error),
    /// Error parsing URI
    #[cfg(feature = "aio")]
    InvalidUri(hyper::http::uri::InvalidUri),
}

impl From<attohttpc::Error> for SearchError {
    fn from(err: attohttpc::Error) -> SearchError {
        SearchError::HttpError(err)
    }
}

impl From<io::Error> for SearchError {
    fn from(err: io::Error) -> SearchError {
        SearchError::IoError(err)
    }
}

impl From<str::Utf8Error> for SearchError {
    fn from(err: str::Utf8Error) -> SearchError {
        SearchError::Utf8Error(err)
    }
}

impl From<xmltree::ParseError> for SearchError {
    fn from(err: xmltree::ParseError) -> SearchError {
        SearchError::XmlError(err)
    }
}

#[cfg(feature = "aio")]
impl From<hyper::Error> for SearchError {
    fn from(err: hyper::Error) -> SearchError {
        SearchError::HyperError(err)
    }
}

#[cfg(feature = "aio")]
impl From<hyper::http::uri::InvalidUri> for SearchError {
    fn from(err: hyper::http::uri::InvalidUri) -> SearchError {
        SearchError::InvalidUri(err)
    }
}
#[cfg(feature = "aio")]
impl From<tokio::timer::timeout::Error<SearchError>> for SearchError {
    fn from(err: tokio::timer::timeout::Error<SearchError>) -> SearchError {
        if err.is_inner() {
            err.into_inner().unwrap()
        } else {
            SearchError::IoError(io::Error::new(io::ErrorKind::TimedOut, "search timed out"))
        }
    }
}

impl fmt::Display for SearchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SearchError::HttpError(ref e) => write!(f, "HTTP error {}", e),
            SearchError::InvalidResponse => write!(f, "Invalid response"),
            SearchError::IoError(ref e) => write!(f, "IO error: {}", e),
            SearchError::Utf8Error(ref e) => write!(f, "UTF-8 error: {}", e),
            SearchError::XmlError(ref e) => write!(f, "XML error: {}", e),
            #[cfg(feature = "aio")]
            SearchError::HyperError(ref e) => write!(f, "Hyper Error: {}", e),
            #[cfg(feature = "aio")]
            SearchError::InvalidUri(ref e) => write!(f, "InvalidUri Error: {}", e),
        }
    }
}

impl error::Error for SearchError {
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            SearchError::HttpError(ref e) => Some(e),
            SearchError::InvalidResponse => None,
            SearchError::IoError(ref e) => Some(e),
            SearchError::Utf8Error(ref e) => Some(e),
            SearchError::XmlError(ref e) => Some(e),
            #[cfg(feature = "aio")]
            SearchError::HyperError(ref e) => Some(e),
            #[cfg(feature = "aio")]
            SearchError::InvalidUri(ref e) => Some(e),
        }
    }

    fn description(&self) -> &str {
        match *self {
            SearchError::HttpError(..) => "HTTP error",
            SearchError::InvalidResponse => "Invalid response",
            SearchError::IoError(..) => "IO error",
            SearchError::Utf8Error(..) => "UTF-8 error",
            SearchError::XmlError(..) => "XML error",
            #[cfg(feature = "aio")]
            SearchError::HyperError(..) => "Hyper Error",
            #[cfg(feature = "aio")]
            SearchError::InvalidUri(_) => "Invalid URI Error"
        }
    }
}

/// An error type that emcompasses all possible errors.
#[derive(Debug)]
pub enum Error {
    /// `AddAnyPortError`
    AddAnyPortError(AddAnyPortError),
    /// `AddPortError`
    AddPortError(AddPortError),
    /// `GetExternalIpError`
    GetExternalIpError(GetExternalIpError),
    /// `RemovePortError`
    RemovePortError(RemovePortError),
    /// `RequestError`
    RequestError(RequestError),
    /// `SearchError`
    SearchError(SearchError),
}

/// A result type where the error is `igd::Error`.
pub type Result<T = ()> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::AddAnyPortError(ref e) => e.fmt(f),
            Error::AddPortError(ref e) => e.fmt(f),
            Error::GetExternalIpError(ref e) => e.fmt(f),
            Error::RemovePortError(ref e) => e.fmt(f),
            Error::RequestError(ref e) => e.fmt(f),
            Error::SearchError(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::AddAnyPortError(ref e) => Some(e),
            Error::AddPortError(ref e) => Some(e),
            Error::GetExternalIpError(ref e) => Some(e),
            Error::RemovePortError(ref e) => Some(e),
            Error::RequestError(ref e) => Some(e),
            Error::SearchError(ref e) => Some(e),
        }
    }

    fn description(&self) -> &str {
        match *self {
            Error::AddAnyPortError(ref e) => e.description(),
            Error::AddPortError(ref e) => e.description(),
            Error::GetExternalIpError(ref e) => e.description(),
            Error::RemovePortError(ref e) => e.description(),
            Error::RequestError(ref e) => e.description(),
            Error::SearchError(ref e) => e.description(),
        }
    }
}

impl From<AddAnyPortError> for Error {
    fn from(err: AddAnyPortError) -> Error {
        Error::AddAnyPortError(err)
    }
}

impl From<AddPortError> for Error {
    fn from(err: AddPortError) -> Error {
        Error::AddPortError(err)
    }
}

impl From<GetExternalIpError> for Error {
    fn from(err: GetExternalIpError) -> Error {
        Error::GetExternalIpError(err)
    }
}

impl From<RemovePortError> for Error {
    fn from(err: RemovePortError) -> Error {
        Error::RemovePortError(err)
    }
}

impl From<RequestError> for Error {
    fn from(err: RequestError) -> Error {
        Error::RequestError(err)
    }
}

impl From<SearchError> for Error {
    fn from(err: SearchError) -> Error {
        Error::SearchError(err)
    }
}

