//! Client to Spotify API endpoint

use derive_builder::Builder;
use log::error;
use maybe_async::maybe_async;
use serde::Deserialize;
use serde_json::{json, map::Map, Value};
use thiserror::Error;

use std::collections::HashMap;
use std::path::PathBuf;
use std::time;

use crate::http::{HTTPClient, Query};
use crate::macros::{build_json, build_map};
use crate::model::{
    idtypes::{IdType, PlayContextIdType},
    *,
};
use crate::oauth2::{Credentials, OAuth, Token};
use crate::pagination::{paginate, Paginator};
