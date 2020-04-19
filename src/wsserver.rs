// import external crates
extern crate rmp_serde as rmps;

// import standard libraries
use std::collections::HashMap;
use std::io::Cursor;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use std::string::ToString;

// import external libraries
use futures::{FutureExt, StreamExt};
use rmps::{Deserializer};
use serde::Deserialize;
use tokio::sync::{mpsc, Mutex};
use warp::ws::{Message, WebSocket};
use warp::Filter;

// import local libraries
use crate::rooms::{Room};
use crate::ohhell::{Player, Round};

// Our global unique user id counter.
static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);
static NEXT_ROOM_ID: AtomicUsize = AtomicUsize::new(1);

// Our state of currently connected users.
//
// - Key is their id
// - Value is a sender of `warp::ws::Message`
type Users = Arc<Mutex<HashMap<usize, mpsc::UnboundedSender<Result<Message, warp::Error>>>>>;

type UsersRooms = Arc<Mutex<HashMap<usize, usize>>>;

type Rooms = Arc<Mutex<HashMap<usize, Room>>>;

// type Regx = Arc<Mutex<RegexSet>>;

#[tokio::main]
pub async fn run_websocket_server() {
    // pretty_env_logger::init();

    // Keep track of all connected users, key is usize, value
    // is a websocket sender.
    let users = Arc::new(Mutex::new(HashMap::new()));
    let gamerooms = Arc::new(Mutex::new(HashMap::new()));
    let room_from_user = Arc::new(Mutex::new(HashMap::new()));

    // Turn our "state" into a new Filter...
    let users = warp::any().map(move || users.clone());
    let gamerooms = warp::any().map(move || gamerooms.clone());
    let room_from_user = warp::any().map(move || room_from_user.clone());

    // GET /game -> websocket upgrade
    let game = warp::path("game")
        // The `ws()` filter will prepare Websocket handshake...
        .and(warp::ws())
        .and(users)
        .and(gamerooms)
        .and(room_from_user)
        .map(|ws: warp::ws::Ws, users, gamerooms, room_from_user| {
            // This will call our function if the handshake succeeds.
            ws.on_upgrade(move |socket| user_connected(socket, users, gamerooms, room_from_user))
        });

    // GET / -> index html
    let index = warp::path::end().map(|| warp::reply::html(INDEX_HTML));

    let routes = index.or(game);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn user_connected(ws: WebSocket, users: Users, gamerooms: Rooms, room_from_user: UsersRooms) {
    // Use a counter to assign a new unique ID for this user.
    let my_id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);

    eprintln!("new chat user: {}", my_id);

    // Split the socket into a sender and receive of messages.
    let (user_ws_tx, mut user_ws_rx) = ws.split();

    // Use an unbounded channel to handle buffering and flushing of messages
    // to the websocket...
    let (tx, rx) = mpsc::unbounded_channel();
    tokio::task::spawn(rx.forward(user_ws_tx).map(|result| {
        if let Err(e) = result {
            eprintln!("websocket send error: {}", e);
        }
    }));

    // Save the sender in our list of connected users.
    users.lock().await.insert(my_id, tx);

    // Return a `Future` that is basically a state machine managing
    // this specific user's connection.

    // Make an extra clone to give to our disconnection handler...
    let users2 = users.clone();
    let gamerooms2 = gamerooms.clone();
    let room_from_user2 = room_from_user.clone();

    // Every time the user sends a message
    while let Some(result) = user_ws_rx.next().await {
        let msg = match result {
            Ok(msg) => {
                println!("got msg");
                msg
            },
            Err(e) => {
                eprintln!("websocket error(uid={}): {}", my_id, e);
                break;
            }
        };
        println!("asdf");
        user_message(my_id, msg, &users, &gamerooms, &room_from_user).await;
    }

    // user_ws_rx stream will keep processing as long as the user stays
    // connected. Once they disconnect, then...
    user_disconnected(my_id, &users2, &gamerooms2, &room_from_user2).await;
}

async fn user_message(my_id: usize, msg: Message, users: &Users, gamerooms: &Rooms, room_from_user: &UsersRooms) {
    println!("user_message");

    let msg = msg.as_bytes();

    if msg.len() <= 0 {
        println!("no msg");
        return;
    }

    println!("msg.as_bytes");

    // {
    //  "action": ("new room", "join room", "set players", "set rounds", "play"),
    //  "count": "int",
    //  "room_id": "int",
    //  "card_value": "13", // King of hearts
    //  "card_suit": "hearts"
    // }

    let cur = Cursor::new(&msg[..]);

    let mut deserializer = Deserializer::new(cur);

    let request:HashMap<String, String> = Deserialize::deserialize(&mut deserializer).unwrap();

    if !request.contains_key("action") {
        eprintln!("action required");
    }

    let action = request.get("action").unwrap();

    println!("action: {:?}", action);

    if action == "new room" {
        println!("new room");

        let mut room = Room::new();

        room.game.add_player(Player::new(my_id.to_string()));

        let room_id = NEXT_ROOM_ID.fetch_add(1, Ordering::Relaxed);

        gamerooms.lock().await.insert(room_id, room);

        room_from_user.lock().await.insert(my_id, room_id);

        let users_waited = users.lock().await;

        let message = format!("{{'user_id':{}}}", my_id);

        let tx = users_waited.get(&my_id).expect("failed to get user");

        if let Err(_disconnected) = tx.send(Ok(Message::text(message.clone()))) {}
    }
    else if action == "join room" {
        println!("join room");

        if !request.contains_key("room_id") {
            eprintln!("missing room_id");
        }

        let room_id = request.get("room_id").unwrap().parse::<usize>().unwrap();

        room_from_user.lock().await.insert(my_id, room_id);
    }
    else if action == "set players" {
        println!("set players");

        if !request.contains_key("count") {
            eprintln!("missing count");
        }

        let count = request.get("count").unwrap().parse::<usize>().unwrap();

        let room_from_user_waited = room_from_user.lock().await;

        let room_id = room_from_user_waited.get(&my_id).unwrap();

        let mut gamerooms_waited = gamerooms.lock().await;

        let room = gamerooms_waited.get_mut(&room_id).unwrap();

        let ref mut game = room.game;

        game.set_players(count);
    }
    else if action == "set rounds" {
        println!("set rounds");

        if !request.contains_key("count") {
            eprintln!("missing count");
        }

        let count = request.get("count").unwrap();
        let count: usize = count.parse::<usize>().unwrap();

        let mut gamerooms_waited = gamerooms.lock().await;

        let room = gamerooms_waited.get_mut(&my_id).unwrap();

        let ref mut game = room.game;

        let ref _game = game.set_rounds(count);
    }
    else if action == "set hands" {
        println!("set hands");

        if !request.contains_key("count") {
            eprintln!("missing count");
        }

        let count = request.get("count").unwrap().parse::<usize>().unwrap();

        let room_from_user_waited = room_from_user.lock().await;

        let room_id = room_from_user_waited.get(&my_id).unwrap();

        let mut gamerooms_waited = gamerooms.lock().await;

        let room = gamerooms_waited.get_mut(&room_id).unwrap();

        let ref mut game = room.game;

        game.set_hands(count);
    }
    else if action == "add round" {
        println!("add round");

        let room_from_user_waited = room_from_user.lock().await;

        let room_id = room_from_user_waited.get(&my_id).unwrap();

        let mut gamerooms_waited = gamerooms.lock().await;

        let room = gamerooms_waited.get_mut(&room_id).unwrap();

        room.game.add_round(Round::new());
    }
    else if action == "play" {
        //
        if !request.contains_key("card_value") {
            eprintln!("missing card_value");
        }

        let card_value = request.get("card_value").unwrap().parse::<i8>().unwrap();

        if !request.contains_key("card_suit") {
            eprintln!("missing card_suit");
        }

        let card_suit = request.get("card_suit").unwrap().as_str();

        // TODO: play card
        // TODO: validate card choice is possible
        // TODO: eval if the hand is over
        // TODO: eval who won the hand
        // TODO: eval if the round is over
        // TODO: distribute points to "winners" of round
        // TODO: eval if the game is over
        // TODO: eval who won the game
    }
    else {
        println!("unknown action");
    }

    // let new_msg = format!("<User#{}>: {}", my_id, msg);

    // // New message from this user, send it to everyone else (except same uid)...
    // //
    // // We use `retain` instead of a for loop so that we can reap any user that
    // // appears to have disconnected.
    // for (&uid, tx) in users.lock().await.iter_mut() {
    //     if my_id != uid {
    //         if let Err(_disconnected) = tx.send(Ok(Message::text(new_msg.clone()))) {
    //             // The tx is disconnected, our `user_disconnected` code
    //             // should be happening in another task, nothing more to
    //             // do here.
    //         }
    //     }
    // }
}

async fn user_disconnected(my_id: usize, users: &Users, gamerooms: &Rooms, room_from_user: &UsersRooms) {
    eprintln!("good bye user: {}", my_id);

    // Stream closed up, so remove from the user list
    users.lock().await.remove(&my_id);
    gamerooms.lock().await.remove(&my_id);
    room_from_user.lock().await.remove(&my_id);
}

// async fn get_room_from_user(my_id: usize, gamerooms: &'static Rooms, room_from_user: &UsersRooms) -> &'static Room {
//     let room_from_user_waited = room_from_user.lock().await;

//     let room_id = room_from_user_waited.get(&my_id).unwrap();

//     let mut gamerooms_waited = gamerooms.lock().await;

//     let room = gamerooms_waited.get_mut(&room_id).unwrap();

//     room
// }

static INDEX_HTML: &str = r#"
<!DOCTYPE html>
<html>
    <head>
        <title>Warp Chat</title>
    </head>
    <body>
        <h1>warp chat</h1>
        <div id="chat">
            <p><em>Connecting...</em></p>
        </div>
        <input type="text" id="text" />
        <button type="button" id="new_room">New Room</button>
        <button type="button" id="set_players">Set Players</button>
        <button type="button" id="set_rounds">Set Rounds</button>
        <script type="text/javascript"
            src="https://rawgit.com/kawanet/msgpack-lite/master/dist/msgpack.min.js">
        </script>
        <script type="text/javascript">
        var uri = 'ws://' + location.host + '/game';
        var ws = new WebSocket(uri);

        function message(data) {
            var line = document.createElement('p');
            line.innerText = data;
            chat.appendChild(line);
        }

        ws.onopen = function() {
            chat.innerHTML = "<p><em>Connected!</em></p>";
        }

        ws.onmessage = function(msg) {
            message(msg.data);
        };

        new_room.onclick = function() {
            var msg = text.value;
            ws.send(msgpack.encode({action:"new room"}));
            text.value = '';
            message(msg);
        };
        set_players.onclick = function() {
            var msg = text.value;
            ws.send(msgpack.encode({action:"set players", count: msg}));
            text.value = '';
            message(msg);
        };
        set_rounds.onclick = function() {
            var msg = text.value;
            ws.send(msgpack.encode({action:"set rounds", count: msg}));
            text.value = '';
            message(msg);
        };
        </script>
    </body>
</html>
"#;
