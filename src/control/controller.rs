use std::sync::Arc;
use std::sync::mpsc::{channel, Receiver, Sender, sync_channel};
use std::thread;
use std::thread::JoinHandle;

use anyhow::Result;

use crate::context::Context;
use crate::control::message::{Action, Call, Message, System};
use crate::state::State;
use crate::control::io::IO;

#[derive(Clone)]
pub struct Controller {
    sender: Sender<Message>,
}

impl Controller {
    pub fn new() -> (Handle, Controller) {
        let (sender, receiver) = channel();
        let controller = Controller {
            sender: sender.clone(),
        };

        let inner_controller = controller.clone();
        let handler = thread::spawn(move || Self::inner_loop(inner_controller, receiver));
        (
            Handle {
                sender: sender.clone(),
                join: Arc::new(Some(handler)),
            },
            controller
        )
    }

    pub fn call(&self, call: Box<Call>) -> Result<()> {
        self.send_msg(Message::Action(Action::Call(call)))
    }

    pub fn read<C, R>(&self, cl: C) -> Result<R>
        where
            C: Fn(&mut State, &mut IO) -> R + Send + Sync + 'static,
            R: Send + Sync + 'static,
    {
        let (sender, receive) = sync_channel(1);

        self.call(Box::new(move |state, ctx| {
            let res = cl(state, ctx);
            sender.send(res)?;
            Ok(())
        }))?;

        Ok(receive.recv()?)
    }

    pub fn send_msg(&self, msg: Message) -> Result<()> {
        Ok(self.sender.send(msg)?)
    }

    fn inner_loop(controller: Controller, receiver: Receiver<Message>) {
        let mut state: Option<State> = None;
        let mut context: Option<IO> = None;
        loop {
            match receiver.recv() {
                Ok(msg) => match msg {
                    Message::Action(action) => {
                        if let Some(state) = &mut state {
                            if let Some(context) = &mut context {
                                match action {
                                    Action::Call(call) => {
                                        if let Err(err) = call(state, context) {
                                            log::warn!("Failed to perform action:{}", err);
                                        }
                                    }
                                }
                            } else {
                                log::info!("Context is none.")
                            }
                        } else {
                            log::info!("State is none.")
                        }
                    }
                    Message::System(msg) => match msg {
                        System::Shutdown => {
                            log::info!("Shutdown controller.");
                            break;
                        }
                        System::Reload(config) => {
                            log::info!("Reload state.");
                            match State::load(config) {
                                Ok(st) => {
                                    state = Some(st);
                                }
                                Err(err) => {
                                    log::warn!("Failed to load state.{}", err);
                                }
                            }
                        }
                        System::Store(config) => {
                            log::info!("Load state.");
                            if let Some(state) = &mut state {
                                if let Err(err) = state.store(config) {
                                    log::warn!("Failed to store state.{}", err);
                                }
                            }
                        }
                        System::SetContext(cxt) => {
                            log::info!("Set context.");
                            context = Some(IO {
                                context: cxt,
                                controller: controller.clone()
                            });
                        }
                    },
                },
                Err(err) => {
                    log::warn!("Failed to receive message. {}", err);
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Handle {
    sender: Sender<Message>,
    join: Arc<Option<JoinHandle<()>>>,
}

impl Drop for Handle {
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
