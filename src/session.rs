use log::{debug, info};

use actix::fut;
use actix::prelude::*;
use actix_broker::BrokerIssue;
use actix_web_actors::ws;

use serde::de;
use serde::{Serialize, Deserialize};

use rmp_serde::{Serializer, Deserializer, Raw, RawRef};
use rmp_serde::decode::{self, Error};

use crate::message::*;
use crate::server::WsChatServer;

#[derive(Debug,Serialize,Deserialize)]
pub struct MPGameAction {
    id: usize,
    string: String
}

#[derive(Debug,Serialize,Deserialize)]
pub struct MPGameResponse {
    id: usize,
    string: String
}

#[derive(Default,Debug)]
pub struct WsChatSession {
    id: usize,
    room: String,
    name: Option<String>,
}

impl WsChatSession {
    pub fn join_room(&mut self, room_name: &str, ctx: &mut ws::WebsocketContext<Self>) {
        let room_name = room_name.to_owned();

        // First send a leave message for the current room
        let leave_msg = LeaveRoom(self.room.clone(), self.id);

        // issue_sync comes from having the `BrokerIssue` trait in scope.
        self.issue_system_sync(leave_msg, ctx);

        // Then send a join message for the new room
        let join_msg = JoinRoom(
            room_name.to_owned(),
            self.name.clone(),
            ctx.address().recipient(),
        );

        WsChatServer::from_registry()
            .send(join_msg)
            .into_actor(self)
            .then(|id, act, _ctx| {
                if let Ok(id) = id {
                    act.id = id;
                    act.room = room_name;
                }

                fut::ready(())
            })
            .wait(ctx);
    }

    pub fn list_rooms(&mut self, ctx: &mut ws::WebsocketContext<Self>) {
        WsChatServer::from_registry()
            .send(ListRooms)
            .into_actor(self)
            .then(|res, _, ctx| {
                if let Ok(rooms) = res {
                    for room in rooms {
                        ctx.text(room);
                    }
                }

                fut::ready(())
            })
            .wait(ctx);
    }

    pub fn send_msg(&self, msg: &str) {
        let content = format!(
            "{}: {}",
            self.name.clone().unwrap_or_else(|| "anon".to_string()),
            msg
        );

        let msg = SendMessage(self.room.clone(), self.id, content);

        // issue_async comes from having the `BrokerIssue` trait in scope.
        self.issue_system_async(msg);
    }

    pub fn start_game(&self, ctx: &mut ws::WebsocketContext<Self>) {
        //
        let msg = StartGame();

        self.issue_system_async(msg);
    }

    pub fn game_action(&self, ctx: &mut ws::WebsocketContext<Self>, action: MPGameAction) {
        info!("game action was hit {:?}", action);
        //
        let ga = GameAction(action.id, action.string);

        //self.issue_system_async(msg);
        WsChatServer::from_registry()
            .send(ga)
            .into_actor(self)
            .then(|res, _, ctx| {
                if let Ok(res) = res {
                    let mut buf = Vec::new();
                    let val = MPGameResponse {
                        id: 1234,
                        string: res
                    };
                    val.serialize(&mut Serializer::new(&mut buf)).unwrap();
                    ctx.binary(buf);
                }

                fut::ready(())
            })
            .wait(ctx);
    }
}

impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!(
            "join: {:?}",
            self
        );
        self.join_room("Main", ctx);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!(
            "WsChatSession closed for {}({}) in room {}",
            self.name.clone().unwrap_or_else(|| "anon".to_string()),
            self.id,
            self.room
        );
    }
}

impl Handler<ChatMessage> for WsChatSession {
    type Result = ();

    fn handle(&mut self, msg: ChatMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsChatSession {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        debug!("WEBSOCKET MESSAGE: {:?}", msg);

        match msg {
            ws::Message::Binary(byt) => {
                info!("binary type message");
                // msgpack
                //let mut buf = Vec::new();
                //byt.read_to_end(&mut buf).unwrap();
                //let vbyt = byt.to_vec();
                //let de = Deserializer::new(&vbyt);
                let mut de = Deserializer::new(&byt[..]);
                let action: MPGameAction = Deserialize::deserialize(&mut de).unwrap();
                self.game_action(ctx, action);
            }
            ws::Message::Text(text) => {
                info!("text type message");
                let msg = text.trim();

                if msg.starts_with('/') {
                    let mut command = msg.splitn(2, ' ');

                    match command.next() {
                        Some("/list") => self.list_rooms(ctx),

                        Some("/join") => {
                            if let Some(room_name) = command.next() {
                                self.join_room(room_name, ctx);
                            } else {
                                ctx.text("!!! room name is required");
                            }
                        }

                        Some("/name") => {
                            if let Some(name) = command.next() {
                                self.name = Some(name.to_owned());
                                ctx.text(format!("name changed to: {}", name));
                            } else {
                                ctx.text("!!! name is required");
                            }
                        }

                        Some("/start") => {
                            if let Some(start) = command.next() {
                                ctx.text("start command");
                                self.start_game(ctx);
                            } else {
                                ctx.text("start takes variables");
                            }
                        }

                        _ => ctx.text(format!("!!! unknown command: {:?}", msg)),
                    }

                    return;
                }
                self.send_msg(msg);
            }
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => {}
        }
    }
}
