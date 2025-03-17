use crate::broker::Broker;

#[derive(Clone, Copy)]
enum Command {
  Connect,
  Disconnect,
  Subscribe,
  Unsubscribe,
  Publish,
  None
}

#[derive(Clone)]
struct Message {
  command: Command,
  path: String,
  body: Option<String>
}

impl From<&[u8]> for Message {
  fn from(buf: &[u8]) -> Self {
    let commands = vec![Command::Subscribe, Command::Unsubscribe, Command::Publish];
    let command = usize::from(buf[0]);
    if command >= commands.len() {
      return Message{ command: Command::None, body: None, path: String::from("")}
    }
    let command = commands[command];
    let mut path_length = (buf[1] << 8) | buf[2] + 3;
    let path = String::from_utf8(buf[3..path_length.into()].to_vec()).unwrap();

    let mut body: Option<String> = None;

    if u16::from(path_length) < buf.len() as u16 {
      let start: usize = path_length.into();
      let end: usize = start+1;
      body = Some( String::from_utf8(buf[start..end].to_vec()).unwrap());
    }

    Message { command, body, path }
  }
}

impl Message {
  pub fn execute(&mut self) {
    let mut broker = Broker::get();
    match self.command {
      Command::Connect => {},
      Command::Subscribe => {
        
      },
      Command::None => {},
      Command::Publish => {},
      _ => {}
    }
  }
}