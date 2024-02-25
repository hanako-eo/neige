use std::collections::HashMap;

use napi::bindgen_prelude::Reference;
use napi::{Env, JsObject};
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_while1};
use nom::character::complete::{char, space1};
use nom::combinator::opt;
use nom::error::ParseError;
use nom::{IResult, InputLength, Parser};

use super::socket::{JsSocket, Socket};

#[napi]
pub enum HTTPVersion {
    V1_1,
    V2,
    V3,
}

#[derive(Debug)]
pub enum RequestError {
    RequestHeaderBadlyFormated(String),
    RequestTargetBadlyFormated,
    NoTargetLine,
}

#[napi(js_name = "Request")]
pub struct JsRequest {
    inner: Request
}

pub struct Request {
    method: String,
    url: String,
    version: HTTPVersion,
    headers: HashMap<String, String>,
    socket: Socket,
}

impl Request {
    pub(super) fn parse(mut socket: Socket) -> Result<Self, RequestError> {
        let mut lines = socket.lines().map(|l| l.unwrap());
        let Some((method, url, version)) = lines
            .next()
            .map(
                |line| -> Result<(String, String, HTTPVersion), RequestError> {
                    Ok(separated3(
                        space1,
                        parse_method.map(String::from),
                        parse_url.map(String::from),
                        parse_version.map(HTTPVersion::from),
                    )
                    .parse(line.as_str())
                    .map_err(|_| RequestError::RequestTargetBadlyFormated)?
                    .1)
                },
            )
            .transpose()?
        else {
            return Err(RequestError::NoTargetLine);
        };

        let headers = lines
            .take_while(|l| !l.is_empty())
            .map(|line| {
                let Ok((name, content)) = parse_header(line.as_str()) else {
                    return Err(RequestError::RequestHeaderBadlyFormated(line));
                };

                Ok((name.to_lowercase(), content.to_string()))
            })
            .collect::<Result<HashMap<String, String>, _>>()?;

        Ok(Self {
            method,
            url,
            version,
            headers,
            socket,
        })
    }
}

impl From<Request> for JsRequest {
    fn from(req: Request) -> Self {
        Self { inner: req }
    }
}

#[napi]
impl JsRequest {
    #[napi(constructor)]
    pub fn new(env: Env) -> napi::Result<Self> {
        Err(unsafe { env.throw(env.create_string("The request cannot be built from 0.")?).unwrap_err_unchecked() })
    }

    #[napi]
    pub fn headers(&self, env: Env) -> napi::Result<JsObject> {
        let mut obj = env.create_object()?;
        for (key, value) in self.inner.headers.iter() {
            obj.set(key, value)?;
        }
        Ok(obj)
    }

    #[napi]
    pub fn socket(&self, reference: Reference<JsRequest>, env: Env) -> napi::Result<JsSocket> {
        Ok(JsSocket {
            inner: reference.share_with(env, |repo| Ok(&repo.inner.socket))?,
        })
    }
}

impl From<&str> for HTTPVersion {
    fn from(value: &str) -> Self {
        match value {
            "HTTP/1.1" => Self::V1_1,
            "HTTP/2" => Self::V2,
            "HTTP/3" => Self::V3,
            _ => unreachable!(),
        }
    }
}

fn parse_method(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_ascii_uppercase() && c.is_ascii_alphabetic()).parse(input)
}

fn parse_url(input: &str) -> IResult<&str, &str> {
    is_not(" ").parse(input)
}

fn parse_version(input: &str) -> IResult<&str, &str> {
    alt((tag("HTTP/1.1"), tag("HTTP/2"), tag("HTTP/3"))).parse(input)
}

fn parse_header(input: &str) -> IResult<&str, &str> {
    let (input, name) = take_while1(|c: char| c.is_alphanumeric() || c == '-').parse(input)?;
    let (rest, _) = tag(":").and(opt(char(' '))).parse(input)?;

    Ok((name, rest))
}

fn separated3<I, E, S, O, A, O2, B, O3, C, O4>(
    mut sep: S,
    mut first: A,
    mut second: B,
    mut third: C,
) -> impl FnMut(I) -> IResult<I, (O2, O3, O4), E>
where
    I: Clone + InputLength,
    S: Parser<I, O, E>,
    A: Parser<I, O2, E>,
    B: Parser<I, O3, E>,
    C: Parser<I, O4, E>,
    E: ParseError<I>,
{
    move |input: I| {
        let (input, a) = first.parse(input)?;
        let (input, _) = sep.parse(input)?;
        let (input, b) = second.parse(input)?;
        let (input, _) = sep.parse(input)?;
        let (input, c) = third.parse(input)?;

        Ok((input, (a, b, c)))
    }
}
