use std::sync::Arc;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;

use anyhow::Result;

use crate::control::message::{Action, Call, Message, System};
use crate::control::state::State;

#[derive(Clone)]
pub struct Controller {
    sender: Sender<Message>,
    join: Arc<Option<JoinHandle<()>>>,
}

impl Controller {
    pub fn new() -> Controller {
        let (sender, receiver) = channel();
        let handler = thread::spawn(move || Self::inner_loop(receiver));

        Controller {
            sender,
            join: Arc::new(Some(handler)),
        }
    }

    pub fn call(&self, call: Box<Call>) -> Result<()> {
        self.send_msg(Message::Action(Action::Call(call)))
    }

    pub fn read<C, R>(&self, cl: C) -> Result<R>
        where C: Fn(&mut State) -> R + Send + Sync + 'static,
              R: Send + Sync + 'static {
        let (sender, receive) = channel();

        self.call(Box::new(move |state| {
            let res = cl(state);
            sender.send(res)?;
            Ok(())
        }))?;

        Ok(receive.recv()?)
    }

    pub fn send_msg(&self, msg: Message) -> Result<()> {
        Ok(self.sender.send(msg)?)
    }

    fn inner_loop(receiver: Receiver<Message>) {
        let mut state: Option<State> = None;
        loop {
            match receiver.recv() {
                Ok(msg) => {
                    match msg {
                        Message::Action(action) => {
                            if let Some(state) = &mut state {
                                match action {
                                    Action::Call(call) => {
                                        if let Err(err) = call(state) {
                                            log::warn!("Failed to perform action:{}", err);
                                        }
                                    }
                                }
                            } else {
                                log::info!("State is none.")
                            }
                        }
                        Message::System(msg) => {
                            match msg {
                                System::Shutdown => {
                                    log::info!("Shutdown controller.");
                                    break;
                                }
                                System::Reload(config) => {
                                    log::info!("Reload state.");
                                    state = Some(State::load(config))
                                }
                                System::Store(config) => {
                                    log::info!("Load state.");
                                    if let Some(state) = &mut state {
                                        state.store(config);
                                    }
                                }
                            }
                        }
                    }
                }
                Err(err) => {
                    log::warn!("Failed to receive message. {}", err);
                }
            }
        }
    }
}

impl Drop for Controller {
    fn drop(&mut self) {
        if let Err(err) = self.sender.send(Message::System(System::Shutdown)) {
            log::warn!("Failed to drop controller. {}", err);
        }
        if let Some(join) = Arc::get_mut(&mut self.join) {
            if let Some(join) = join.take() {
                if let Err(err) = join.join() {
                    log::warn!("Failed to drop controller. {:?}", err);
                }
            }
        }
    }
}
