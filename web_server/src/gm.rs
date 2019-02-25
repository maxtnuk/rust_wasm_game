use std::collections::{HashMap,HashSet};
use crate::user::Player;
use wasm_games::{Message,Board,BoardState};

pub struct GameManager<T>
where T: Board{
    pub room: RoomController<T>
}
impl <T> GameManager<T>
where T: Board{
    pub fn new() -> Self{
        GameManager{
            room: RoomController::new::<T>()
        }
    }
}
pub struct RoomController<T>
where T: Board{
    rooms: HashMap<String,Room<T>>,
    roomindex: HashMap<String,String>
}

impl <T> RoomController<T>
where T: Board{
    fn new() -> Self{
        RoomController{
            rooms: HashMap::new(),
            roomindex: HashMap::new()
        }
    }
    fn create(&mut self,players: Vec<Player>){
        let roomid = players.iter().fold(String::new(),|acc,x|{
            acc+&x.to_string()
        });
        
        let room = Room::new::<T>(roomid.clone(),players);
        self.rooms.insert(roomid.clone(),room);
        self.rooms[roomid].ready();
        
        for i in players.iter(){
            self.roomindex.insert(i.to_string(),roomid.clone());
            i.send_msg(Message::Ready);
        }
    }
    fn get_room(&self,user_id: String)->Option<&Room<T>> {
        let room_id=self.roomindex.get(&user_id);
        self.rooms.get(&room_id)
    }
    fn destroy(&mut self,id: String){
        
        for player in self.rooms[id].players.iter(){
            self.roomindex.remove(&player.to_string());
            player.send(Message::Destroy)
        }
        
        self.rooms.remove(id);
    }
    fn gameover(&mut self,room_id: String,winner: Player){
        let room= self.rooms[room_id];
        for player in room.player.iter(){
            let msg = if winner == player {"you win"} else {"you loose"};
            self.roomindex.remove(&player.to_string());
            player.send_msg(Message::Destroy);
        }
        
    }
}
enum RoomStatus{
    Nothing,
    Ready,
    Playing,
    GameOver,
}
pub struct Room<T>
where T: Board{
    id: String,
    pub player: Vec<Player>,
    status: RoomStatus,
    pub board: T
}
impl <T> Room<T>
where T: Board{
    fn new(id: String, players: Vec<Player>,board: T)->Self{
        Room{
            id: id,
            player: players,
            status: RoomStatus::Nothing,
            board: board
        }
    }
    fn ready(&mut self){
        self.status=RoomStatus::Ready;
        self.board.ready();
    }
    fn update(&mut self){
        self.status=RoomStatus::Playing;
        self.board.update();
    }
    fn game_loop(&self){
        loop {
            self.board.update();
            if self.board.gm_state() != BoardState::Relay{
                break;
            }
        }
    }
    pub fn get_id(&self)-> String{
        self.id.clone()
    }
}