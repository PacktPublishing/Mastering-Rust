#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate lazy_static;
extern crate rocket;
extern crate rocket_contrib;
extern crate rand;
extern crate time;
extern crate serde;
#[macro_use] extern crate serde_derive;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use rocket::request::{Outcome, Form, FromRequest, Request, FlashMessage};
use rocket::Outcome::Success;
use rocket::response::{Flash, Redirect, NamedFile};
use rocket_contrib::Template;
use rocket::http::{Cookie, Cookies};
use time::get_time;

use std::path::{Path, PathBuf};
use std::io;

use rand::Rng;

lazy_static! {
    pub static ref CHARACTERS: Characters = Arc::new(Mutex::new(HashMap::new()));
}

#[derive(Serialize, Clone)]
pub struct Character {
    name: String,
    strength: u8,
    dexterity: u8,
    hitpoints: u8
}
#[derive(FromForm, Debug)]
struct CharacterForm {
    name: String
}

type Characters = Arc<Mutex<HashMap<String, Character>>>;
struct GameState {
    players: Characters
}

impl<'a, 'r> FromRequest<'a, 'r> for GameState {
    type Error = std::fmt::Error;
    fn from_request(_: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        Success(GameState{ players: CHARACTERS.clone() })
    }
}

#[derive(Serialize)]
pub struct GameViewData {
    flash: String,
    characters: HashMap<String, Character>
}
#[get("/")]
fn index(cookies: &Cookies) -> Redirect {
    match cookies.find("character_id") {
        Some(_) => Redirect::to("/game"),
        None => Redirect::to("/new")
    }
}

#[get("/new")]
fn new() -> Template {
    Template::render("index", &())
}

#[post("/new", data = "<character_form>")]
fn post_new(cookies: &Cookies, character_form: Form<CharacterForm>, state: GameState) -> Result<Redirect, String> {
    let character = character_form.get();
    println!("char: {:?}", character);

    let mut rng = rand::thread_rng();
    let new_character_id: String = rng.gen::<u64>().to_string();

    let ref mut players = *state.players.lock().unwrap();
    players.insert(new_character_id.clone(), Character {
        name: character.name.clone(),
        strength: rng.gen::<u8>(),
        dexterity: rng.gen::<u8>(),
        hitpoints: rng.gen::<u8>()
    });

    cookies.add(Cookie::new("character_id", new_character_id));
    cookies.add(Cookie::new("throttle", get_time().sec.to_string()));
    Ok(Redirect::to("/game"))
}


#[get("/game")]
fn game(state: GameState, flash: Option<FlashMessage>) -> Template {
    let players = state.players.clone();
    let characters = players.lock().unwrap();
    let flash: String = match flash {
        Some(f) => f.msg().into(),
        None => "".into()
    };

    let g_v_d = GameViewData { flash: flash, characters: characters.clone() };
    Template::render("game", &g_v_d)
}

#[post("/attack/<id>")]
fn attack(cookies: &Cookies, state: GameState, id: &str) -> Flash<Redirect> {
    let ref mut players = *state.players.lock().unwrap();

    let throttle: i64 = cookies.find("throttle").unwrap().value().parse().unwrap();
    println!("throttle is {}, current time is {}", throttle, get_time().sec);
    if throttle >= get_time().sec {
        return Flash::error(Redirect::to("/game"), "Attacking too fast!");
    }

    let attacker_cookie = cookies.find("character_id").unwrap();
    let attacker_id = attacker_cookie.value();
    let attacker = players.get(attacker_id).unwrap().clone();
    let defender = players.get(id).unwrap().clone();

    let mut rng = rand::thread_rng();
    let damage: i16 = attacker.strength as i16 - defender.dexterity as i16 + rng.gen::<i8>() as i16;

    let message = if damage < 1 {
        format!("{} missed {}", attacker.name, defender.name)
    } else if defender.hitpoints as i16 - damage < 1 {
        players.remove(id);
        format!("{} hits {}. {} is slain!", attacker.name, defender.name, defender.name)
    } else {
        let new_defender = Character {
            name: defender.name.clone(),
            strength: defender.strength,
            dexterity: defender.dexterity,
            hitpoints: defender.hitpoints - damage as u8
        };
        players.insert(id.into(), new_defender);
        format!("{} hits {}.", attacker.name, defender.name)
    };

    cookies.add(Cookie::new("throttle", get_time().sec.to_string()));
    Flash::error(Redirect::to("/game"), message)
}

#[get("/<path..>", rank=1)]
fn static_files(path: PathBuf) -> io::Result<NamedFile> {
    NamedFile::open(Path::new("static/").join(path))
}

fn main() {
    rocket::ignite().
        mount("/", routes![index, new, post_new, game, attack, static_files]).
        launch();
}
