pub enum Status {
    OPEN,
    INPROGRESS,
    CLOSED,
}

pub struct Epic {
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<Story>,
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            status: Status::OPEN,
            stories: Vec::new(),
        }
    }
}

pub struct Story {
    pub name: String,
    pub description: String,
    pub status: Status,
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            status: Status::OPEN,
        }
    }
}

pub struct DBState {
    pub last_item_id: u64,
    pub epics: Vec<Epic>,
    pub stories: Vec<Story>,
}
