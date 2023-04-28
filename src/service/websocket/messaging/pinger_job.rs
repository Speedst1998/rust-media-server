use crate::service::websocket::socket_manager::{self, SocketManager};

pub struct PingerJob<'a> {
    pub socket_manager: Option<SocketManager<'a>>,
    hasPonged: bool,
}

impl<'a> PingerJob<'a> {
    pub fn set_socket_manager(&mut self, socket_manager: SocketManager<'a>) -> &mut PingerJob<'a> {
        self.socket_manager = Some(socket_manager);
        self
    }

    pub fn ping(&mut self) {
        self.hasPonged = true;
        //self.socket_manager.send("ping");
        // wait 5 seconds
        if !self.hasPonged {
            //self.socket_manager.reconnect
        }
    }

    pub fn notify(&mut self) {
        self.hasPonged = true;
    }

    pub fn new(socket_manager: Option<SocketManager>) -> PingerJob {
        PingerJob {
            socket_manager: socket_manager,
            hasPonged: false,
        }
    }
}
