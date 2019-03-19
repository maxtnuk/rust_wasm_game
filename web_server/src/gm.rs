use std::collections::{HashMap};
use crate::user::Player;
use wasm_games::{Message,Board,BoardState,GameList,game_board,Input};

pub struct GameManager{
    pub room: RoomController
}
impl GameManager{
    pub fn new() -> Self{
        GameManager{
            room: RoomController::new()
        }
    }
}

pub struct RoomController
{
    rooms: HashMap<String,Room>,
    roomindex: HashMap<String,String>
}

impl RoomController
{
    pub fn new() -> Self{
        RoomController{
            rooms: HashMap::new(),
            roomindex: HashMap::new()
        }
    }
    pub fn create(&mut self,players: Vec<Player>,game: GameList){
        let roomid = players.iter().fold(String::new(),|acc,x|{
            acc+&x.to_string()
        });
        let game_form=game_board(game);

        let room = Room::new(roomid.clone(),players.clone(),Box::new(game_form));
        self.rooms.insert(roomid.clone(),room);
        self.rooms.get_mut(&roomid).unwrap().ready();

        for i in players.iter(){
            self.roomindex.insert(i.to_string(),roomid.clone());
            i.send_msg(Message::Ready);
        }
    }
    pub fn get_room(&mut self,user_id: String)->Option<&mut Room> {
        if let Some(room_id) =self.roomindex.get(&user_id){
            self.rooms.get_mut(room_id)    
        }else{
            None
        }
    }
    pub fn destroy(&mut self,id: String){
        if self.rooms.get(&id).is_none(){
            println!("there is no room {}",id.clone());
            return;
        }
        for player in self.rooms.get(&id).unwrap().player.iter(){
            self.roomindex.remove(&player.to_string());
            player.send_msg(Message::Destroy)
        }

        self.rooms.remove(&id);
    }
    pub fn gameover(&mut self,room_id: String,winner: Player){
        let room= self.rooms.get(&room_id);
        if room.is_none(){
            println!("there is no {} game",room_id);
            return;
        }
        for player in room.unwrap().player.iter(){
            let _msg = if winner == *player {"you win"} else {"you loose"};
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
pub struct Room
{
    id: String,
    pub player: Vec<Player>,
    status: RoomStatus,
    pub board: Box<dyn Board>
}
impl Room
{
    pub fn new(id: String, players: Vec<Player>,board: Box<dyn Board>)->Self{
        Room{
            id: id,
            player: players,
            status: RoomStatus::Nothing,
            board: board
        }
    }
    pub fn ready(&mut self){
        self.status=RoomStatus::Ready;
        self.board.ready();
    }
    pub fn update(&mut self){
        self.status=RoomStatus::Playing;
        self.board.update();
    }
    pub fn game_loop(&mut self){
        loop {
            self.board.update();
            if self.board.gm_state() != BoardState::Relay{
                break;
            }
        }
    }
    pub fn input(&self,input: Input){
        
    }
    pub fn get_id(&self)-> String{
        self.id.clone()
    }
}
