use crate::service::websocket::socket_manager::SocketManager;

pub struct PingerJob<'a> {
    pub socket_manager: Option<&'a SocketManager<'a>>,
    has_ponged: bool,
}

impl<'a> PingerJob<'a> {
    pub fn set_socket_manager(
        &mut self,
        socket_manager: &'a SocketManager<'a>,
    ) -> &mut PingerJob<'a> {
        self.socket_manager = Some(socket_manager);
        self
    }

    pub fn ping(&mut self) {
        self.has_ponged = true;
        //self.socket_manager.send("ping");
        // wait 5 seconds
        if !self.has_ponged {
            //self.socket_manager.reconnect
        }
    }

    pub fn notify(&mut self) {
        self.has_ponged = true;
    }

    pub fn new(socket_manager: Option<&'a SocketManager<'a>>) -> PingerJob {
        PingerJob {
            socket_manager,
            has_ponged: false,
        }
    }
}
