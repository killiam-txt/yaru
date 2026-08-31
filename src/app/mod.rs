pub mod search;
pub mod list;

#[derive(Clone)]
pub enum Screen {
    Search,
    Detail,
    List,
}

pub struct App {
    pub input: String,
    pub results: Vec<String>,
    pub result_ids: Vec<i64>,
    pub list_ids: Vec<i64>,
    pub error: Option<String>,
    pub selected: usize,
    pub list_selected: usize,
    pub detail: Option<String>,
    pub screen: Screen,
    pub previous_screen: Screen,
    pub list_items: Vec<String>,
    pub insert_mode: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            results: Vec::new(),
            result_ids: Vec::new(),
            list_ids: Vec::new(),
            error: None,
            selected: 0,
            list_selected: 0,
            detail: None,
            screen: Screen::Search,
            previous_screen: Screen::Search,
            list_items: Vec::new(),
            insert_mode: false,
        }
    }
}