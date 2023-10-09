struct Iping {
    pong: String,
}

#[juniper::object(description = "GQL: Ping")]
impl Iping {
    pub fn pong(&self) -> &str {
        self.pong.as_str()
    }
}
