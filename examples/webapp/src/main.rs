//! In this example, the token is saved into a cache file. If you are building a real-world web
//! app, it's easy to save token into database, by calling the function
//! `util::get_token_without_cache()`, instead of `util::get_token()`, which saves token by
//! default.

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::http::{Cookie, Cookies};
use rocket::response::Redirect;
use rocket_contrib::json;
use rocket_contrib::json::JsonValue;
use rocket_contrib::templates::Template;
use rspotify::blocking::client::Spotify;

use rspotify::blocking::oauth2::{SpotifyClientCredentials, SpotifyOAuth};
use rspotify::blocking::util;

use std::env;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Responder)]
pub enum AppResponse {
    Template(Template),
    Redirect(Redirect),
    Json(JsonValue),
}

const CACHE_PATH: &str = ".spotify_cache/";

fn cache_path(cookies: Cookies) -> PathBuf {
    let project_dir_path = env::current_dir().unwrap();
    let mut cache_path = PathBuf::from(project_dir_path);
    cache_path.push(CACHE_PATH);
    let cache_dir = cache_path.display().to_string();
    cache_path.push(cookies.get("uuid").unwrap().value());
    println!("cache_path: {:?}", cache_path);
    if !Path::new(cache_dir.as_str()).exists() {
        fs::create_dir_all(cache_dir).unwrap();
    }
    cache_path
}

fn remove_cache_path(mut cookies: Cookies) -> () {
    let project_dir_path = env::current_dir().unwrap();
    let mut cache_path = PathBuf::from(project_dir_path);
    cache_path.push(CACHE_PATH);
    let cache_dir = cache_path.display().to_string();
    if Path::new(cache_dir.as_str()).exists() {
        fs::remove_dir_all(cache_dir).unwrap()
    }
    cookies.remove(Cookie::named("uuid"))
}

fn spotify(mut auth_manager: SpotifyOAuth) -> Spotify {
    let token_info = util::get_token(&mut auth_manager).unwrap();
    let client_credential = SpotifyClientCredentials::default()
        .token_info(token_info)
        .build();
    Spotify::default()
        .client_credentials_manager(client_credential)
        .build()
}

fn auth_manager(cookies: Cookies) -> SpotifyOAuth {
    // Please notice that protocol of redirect_uri, make sure it's http(or https). It will fail if you mix them up.
    SpotifyOAuth::default()
        .client_id("your-client-id")
        .client_secret("your-client-secret")
        .redirect_uri("http://localhost:8000/callback")
        .cache_path(cache_path(cookies))
        .scope("user-read-currently-playing playlist-modify-private")
        .build()
}

#[get("/callback?<code>")]
fn callback(cookies: Cookies, code: String) -> AppResponse {
    let auth_manager = auth_manager(cookies);
    return match auth_manager.get_access_token(code.as_str()) {
        Some(_) => AppResponse::Redirect(Redirect::to("/")),
        _ => {
            let mut context = HashMap::new();
            context.insert("err_msg", "Can not get code!");
            AppResponse::Template(Template::render("error", context))
        }
    };
}

#[get("/")]
fn index(mut cookies: Cookies) -> AppResponse {
    let cookie = cookies.get("uuid");
    if let None = cookie {
        cookies.add(Cookie::new("uuid", util::generate_random_string(64)));
    }
    let mut auth_manager = auth_manager(cookies);
    let mut context = HashMap::new();
    match auth_manager.get_cached_token() {
        Some(token) => token,
        None => {
            let state = util::generate_random_string(16);
            let auth_url = auth_manager.get_authorize_url(Some(&state), None);
            context.insert("auth_url", auth_url);
            return AppResponse::Template(Template::render("authorize", context));
        }
    };
    let token_info = util::get_token(&mut auth_manager).unwrap();
    let client_credential = SpotifyClientCredentials::default()
        .token_info(token_info)
        .build();
    let spotify = Spotify::default()
        .client_credentials_manager(client_credential)
        .build();
    let me = spotify.me();
    println!("me: {:?}", me);
    context.insert(
        "display_name",
        me.unwrap().display_name.unwrap_or(String::from("Dear")),
    );
    AppResponse::Template(Template::render("index", context.clone()))
}

#[get("/sign_out")]
fn sign_out(cookies: Cookies) -> AppResponse {
    remove_cache_path(cookies);
    AppResponse::Redirect(Redirect::to("/"))
}

#[get("/playlists")]
fn playlist(cookies: Cookies) -> AppResponse {
    let mut auth_manager = SpotifyOAuth::default()
        .cache_path(cache_path(cookies))
        .build();
    if let None = auth_manager.get_cached_token() {
        return AppResponse::Redirect(Redirect::to("/"));
    }
    let spotify = spotify(auth_manager);
    let playlists = spotify.current_user_playlists(Some(20), Some(0)).unwrap();
    AppResponse::Json(json!(playlists))
}

#[get("/me")]
fn me(cookies: Cookies) -> AppResponse {
    let mut auth_manager = SpotifyOAuth::default()
        .cache_path(cache_path(cookies))
        .build();
    if let None = auth_manager.get_cached_token() {
        return AppResponse::Redirect(Redirect::to("/"));
    }
    let spotify = spotify(auth_manager);
    let user_info = spotify.me().unwrap();
    AppResponse::Json(json!(user_info))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, callback, sign_out, me, playlist])
        .attach(Template::fairing())
        .launch();
}
