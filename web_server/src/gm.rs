use std::collections::{HashMap};
use crate::user::Player;
use wasm_games::{Message,Board,BoardState,GameList,game_board};

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

        let room = Room::new(roomid.clone(),players,Box::new(game_form));
        self.rooms.insert(roomid.clone(),room);
        self.rooms[&roomid].ready();

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
pub struct Room
{
    id: String,
    pub player: Vec<Player>,
    status: RoomStatus,
    pub board: Box<dyn Board>
}
impl Room
{
    fn new<'a,T>(id: String, players: Vec<Player>,board: Box<dyn Board>)->Self
    where T: Board + Clone+'a{
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
