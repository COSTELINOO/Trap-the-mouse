#![warn(unused_variables)]
#[derive(Debug, Clone)]

#[derive(PartialEq)]
pub enum Page {
    Menu,
    SinglePlayer,
    TwoPlayers,
    Join,
    Create,
    GameBoard,
    Option,
}

#[derive(Debug,Clone,PartialEq)]
pub enum Role {
    Mouse,
    Hunter,
}

#[derive(Debug,Clone)]
pub struct Model {
   pub  current_page: Page,
   pub  role: Option<Role>,
    pub current_move:Option<Role>,
   pub pin: Option<String>,

}

impl Model {

    pub fn go_to_menu(&mut self) {
        self.current_page = Page::Menu;
        self.role = None;
        self.pin = None;
    }

    pub fn go_to_single_player(&mut self) {
        self.current_page = Page::SinglePlayer;
    }

    pub fn go_to_two_players(&mut self) {
        self.current_page = Page::TwoPlayers;
    }

    pub fn go_to_join(&mut self) {
        self.current_page = Page::Join;
        self.role = Some(Role::Hunter);
    }

    pub fn go_to_create(&mut self) {
        self.current_page = Page::Create;
        self.role = Some(Role::Mouse);
    }

    pub fn go_to_game_board(&mut self) {
        self.current_page = Page::GameBoard;
    }
    pub fn go_to_option(&mut self) {
        self.current_page = Page::Option;
    }

    pub fn set_role(&mut self, role: Role) {
        self.role = Some(role);
    }

    pub fn set_pin(&mut self, pin: String) {
        self.pin = Some(pin);
    }
}
