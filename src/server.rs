use log::{debug, info};

use actix::prelude::*;
use actix_broker::BrokerSubscribe;

use std::collections::HashMap;
use std::mem;

use crate::message::*;

type Client = Recipient<ChatMessage>;
type Room = HashMap<usize, Client>;

#[derive(Default,Debug)]
pub struct WsChatServer {
    rooms: HashMap<String, Room>,
}

impl WsChatServer {
    fn take_room(&mut self, room_name: &str) -> Option<Room> {
        let room = self.rooms.get_mut(room_name)?;
        let room = mem::replace(room, HashMap::new());
        Some(room)
    }

    fn add_client_to_room(
        &mut self,
        room_name: &str,
        id: Option<usize>,
        client: Client,
    ) -> usize {
        let mut id = id.unwrap_or_else(rand::random::<usize>);

        if let Some(room) = self.rooms.get_mut(room_name) {
            loop {
                if room.contains_key(&id) {
                    id = rand::random::<usize>();
                } else {
                    break;
                }
            }

            room.insert(id, client);
            return id;
        }

        // Create a new room for the first client
        let mut room: Room = HashMap::new();

        room.insert(id, client);
        self.rooms.insert(room_name.to_owned(), room);

        id
    }

    fn send_chat_message(
        &mut self,
        room_name: &str,
        msg: &str,
        _src: usize,
    ) -> Option<()> {
        let mut room = self.take_room(room_name)?;

        for (id, client) in room.drain() {
            if client.do_send(ChatMessage(msg.to_owned())).is_ok() {
                self.add_client_to_room(room_name, Some(id), client);
            }
        }

        Some(())
    }
}

impl Actor for WsChatServer {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("WsChatServer started");
        self.subscribe_system_async::<LeaveRoom>(ctx);
        self.subscribe_system_async::<SendMessage>(ctx);
        self.subscribe_system_async::<StartGame>(ctx);
        //self.subscribe_system_async::<GameAction>(ctx);
    }
}

impl Handler<JoinRoom> for WsChatServer {
    type Result = MessageResult<JoinRoom>;

    fn handle(&mut self, msg: JoinRoom, _ctx: &mut Self::Context) -> Self::Result {
        info!("WsChatServer handled JoinRoom");
        let JoinRoom(room_name, client_name, client) = msg;

        let id = self.add_client_to_room(&room_name, None, client);
        let join_msg = format!(
            "{} joined {}",
            client_name.unwrap_or_else(|| "anon".to_string()),
            room_name
        );

        self.send_chat_message(&room_name, &join_msg, id);
        MessageResult(id)
    }
}

impl Handler<LeaveRoom> for WsChatServer {
    type Result = ();

    fn handle(&mut self, msg: LeaveRoom, _ctx: &mut Self::Context) {
        info!("WsChatServer handled LeaveRoom");
        info!("LeaveRoom: {:?}", &_ctx);
        info!("someone left {}", &msg.0);
        if let Some(room) = self.rooms.get_mut(&msg.0) {
            room.remove(&msg.1);
        }
    }
}

impl Handler<ListRooms> for WsChatServer {
    type Result = MessageResult<ListRooms>;

    fn handle(&mut self, _: ListRooms, _ctx: &mut Self::Context) -> Self::Result {
        info!("WsChatServer handled ListRoom");
        MessageResult(self.rooms.keys().cloned().collect())
    }
}

impl Handler<SendMessage> for WsChatServer {
    type Result = ();

    fn handle(&mut self, msg: SendMessage, _ctx: &mut Self::Context) {
        info!("WsChatServer handled SendMessage");
        let SendMessage(room_name, id, msg) = msg;
        self.send_chat_message(&room_name, &msg, id);
    }
}

impl Handler<StartGame> for WsChatServer {
    type Result = ();

    fn handle(&mut self, msg: StartGame, _ctx: &mut Self::Context) {
        info!("StartGame was handled by the WsChatServer");
    }
}

impl Handler<GameAction> for WsChatServer {
    type Result = MessageResult<GameAction>;

    fn handle(&mut self, msg: GameAction, _ctx: &mut Self::Context) -> Self::Result {
        info!("PlayGame was handled by the WsChatServer");
        info!("msg: {:?}", msg);
        MessageResult("test".to_string())
    }
}

impl SystemService for WsChatServer {}
impl Supervised for WsChatServer {}
