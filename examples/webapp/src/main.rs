//! In this example, the token is saved into a cache file. If you are building a
//! real-world web app, you should store it in a database instead. In that case
//! you can disable `token_cached` in the `Config` struct passed to the client
//! when initializing it to avoid using cache files.

use cookie::{time::Duration, SameSite};
use getrandom::getrandom;
use log::{error, info};
use rocket::{
    catch, catchers,
    config::TlsConfig,
    get,
    http::{Cookie, CookieJar},
    launch,
    response::Redirect,
    routes,
    serde::{
        json::{json, Value},
        Deserialize,
    },
    Request, Responder,
};
use rocket_dyn_templates::Template;
use rspotify::{model::TimeRange, prelude::*, scopes, AuthCodeSpotify, Config, Credentials, OAuth};

use std::{collections::HashMap, env, fs, net, path::PathBuf};

#[derive(Debug, Responder)]
#[allow(clippy::large_enum_variant)]
pub enum AppResponse {
    Template(Template),
    Redirect(Redirect),
    Json(Value),
}

const CACHE_PATH: &str = ".spotify_cache/";
// Taken from the `.env` file from the main repository. Please replace this with
// yours for production usage.
const CLIENT_ID: &str = "e1dce60f1e274e20861ce5d96142a4d3";
const CLIENT_SECRET: &str = "0e4e03b9be8d465d87fc32857a4b5aa3";

fn creds() -> Credentials {
    Credentials::new(CLIENT_ID, CLIENT_SECRET)
}

fn oauth() -> OAuth {
    #[derive(Deserialize)]
    #[serde(crate = "rocket::serde")]
    struct Config {
        address: net::IpAddr,
        port: u16,
        tls: Option<TlsConfig>,
    }

    // Obtaining the configured host details from the Rocket configuration.
    let rocket = rocket::build();
    let config: Config = rocket.figment().extract().expect("no config set by rocket");
    let address = if config.address == net::Ipv4Addr::LOCALHOST {
        "localhost".to_string() // API doesn't work with IPs
    } else {
        config.address.to_string()
    };
    let port = config.port;
    let protocol = config.tls.map_or("http", |_| "https");

    OAuth {
        scopes: scopes!(
            "user-read-currently-playing",
            "playlist-modify-private",
            "user-top-read"
        ),
        redirect_uri: format!("{protocol}://{address}:{port}/callback"),
        ..Default::default()
    }
}

fn config(jar: &CookieJar<'_>) -> Config {
    Config {
        token_cached: true,
        token_refreshing: true,
        cache_path: create_cache_path_if_absent(jar),
        ..Default::default()
    }
}

/// Generate `length` random chars
fn generate_random_uuid(length: usize) -> String {
    let alphanum: &[u8] =
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".as_bytes();
    let mut buf = vec![0u8; length];
    getrandom(&mut buf).unwrap();
    let range = alphanum.len();

    buf.iter()
        .map(|byte| alphanum[*byte as usize % range] as char)
        .collect()
}

/// We store the cache locally within the current directory.
fn get_cache_path(jar: &CookieJar<'_>) -> PathBuf {
    let mut cache_path = env::current_dir().unwrap();
    cache_path.push(CACHE_PATH);
    cache_path.push(jar.get_pending("uuid").unwrap().value());

    cache_path
}

fn create_cache_path_if_absent(jar: &CookieJar<'_>) -> PathBuf {
    let cache_path = get_cache_path(jar);
    if !cache_path.exists() {
        let mut path = cache_path.clone();
        path.pop();
        fs::create_dir_all(path).unwrap();
    }
    cache_path
}

fn is_authenticated(jar: &CookieJar<'_>) -> bool {
    jar.get("uuid").is_some() && cache_path_exists(jar)
}

fn remove_cache_path(jar: &CookieJar<'_>) {
    let cache_path = get_cache_path(jar);
    if cache_path.exists() {
        fs::remove_file(cache_path).unwrap()
    }
    jar.remove(Cookie::named("uuid"))
}

fn cache_path_exists(jar: &CookieJar<'_>) -> bool {
    let cache_path = get_cache_path(jar);
    cache_path.exists()
}

#[get("/callback?<code>")]
fn callback(jar: &CookieJar<'_>, code: String) -> AppResponse {
    if jar.get("uuid").is_none() {
        let mut context = HashMap::new();
        context.insert("err_msg", "The uuid in cookie is empty!");
        return AppResponse::Template(Template::render("error", context));
    }

    let spotify = AuthCodeSpotify::with_config(creds(), oauth(), config(jar));

    match spotify.request_token(&code) {
        Ok(_) => {
            info!("Requested user token successfully");
            AppResponse::Redirect(Redirect::to("/"))
        }
        Err(err) => {
            error!("Failed to get user token: {:?}", err);
            let mut context = HashMap::new();
            context.insert("err_msg", "Failed to get token!");
            AppResponse::Template(Template::render("error", context))
        }
    }
}

fn show_index(spotify: &AuthCodeSpotify) -> Template {
    let mut context = HashMap::new();
    match spotify.me() {
        Ok(user_info) => {
            context.insert(
                "display_name",
                user_info
                    .display_name
                    .unwrap_or_else(|| String::from("Dear")),
            );
            Template::render("index", context.clone())
        }
        Err(err) => {
            context.insert("err_msg", format!("Failed to fetch `me` endpoint: {err}"));
            Template::render("error", context)
        }
    }
}

#[get("/")]
fn index(jar: &CookieJar<'_>) -> Template {
    // If it's the user's first log in, we need to authorize our Spotify client.
    // The next time, the cache will already be initialized, so we can read from
    // it. Even if it's expired, we'll have the refresh token, which can be used
    // to authorize again seamlessly.
    if !is_authenticated(jar) {
        let uuid = Cookie::build("uuid", generate_random_uuid(64))
            .path("/")
            .secure(true)
            .max_age(Duration::minutes(30))
            .same_site(SameSite::Lax)
            .finish();
        jar.add(uuid);

        let spotify = AuthCodeSpotify::with_config(creds(), oauth(), config(jar));
        let auth_url = spotify.get_authorize_url(true).unwrap();
        let mut context = HashMap::new();
        context.insert("auth_url", auth_url);
        return Template::render("authorize", context);
    }

    // Already authenticated Spotify client.
    let spotify = AuthCodeSpotify::with_config(creds(), oauth(), config(jar));
    show_index(&spotify)
}

#[get("/topartists")]
fn top_artists(jar: &CookieJar<'_>) -> AppResponse {
    if !is_authenticated(jar) {
        return AppResponse::Redirect(Redirect::to("/"));
    }

    let spotify = AuthCodeSpotify::with_config(creds(), oauth(), config(jar));

    let top_artists = spotify
        .current_user_top_artists(Some(TimeRange::LongTerm))
        .take(10)
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    AppResponse::Json(json!(top_artists))
}

#[get("/sign_out")]
fn sign_out(jar: &CookieJar<'_>) -> AppResponse {
    remove_cache_path(jar);
    AppResponse::Redirect(Redirect::to("/"))
}

#[get("/playlists")]
fn playlist(jar: &CookieJar<'_>) -> AppResponse {
    if !is_authenticated(jar) {
        return AppResponse::Redirect(Redirect::to("/"));
    }

    let spotify = AuthCodeSpotify::with_config(creds(), oauth(), config(jar));
    let playlists = spotify
        .current_user_playlists()
        .take(50)
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    if playlists.is_empty() {
        return AppResponse::Redirect(Redirect::to("/"));
    }

    AppResponse::Json(json!(playlists))
}

#[get("/me")]
fn me(jar: &CookieJar<'_>) -> AppResponse {
    if !is_authenticated(jar) {
        return AppResponse::Redirect(Redirect::to("/"));
    }

    let spotify = AuthCodeSpotify::with_config(creds(), oauth(), config(jar));
    match spotify.me() {
        Ok(user_info) => AppResponse::Json(json!(user_info)),
        Err(_) => AppResponse::Redirect(Redirect::to("/")),
    }
}

#[catch(500)]
pub fn server_error(_req: &Request) -> Template {
    let mut context = HashMap::new();
    context.insert(
        "err_msg",
        "Ooops, there is something wrong with the server".to_owned(),
    );
    Template::render("error", context)
}

#[launch]
fn rocket() -> _ {
    env_logger::init();
    rocket::build()
        .mount(
            "/",
            routes![index, callback, sign_out, me, playlist, top_artists],
        )
        .attach(Template::fairing())
        .register("/", catchers![server_error])
}
