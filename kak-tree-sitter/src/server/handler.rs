use std::{
  path::PathBuf,
  sync::mpsc::{Receiver, Sender, channel},
  thread::{self, JoinHandle},
  time::Instant,
};

use kak_tree_sitter_config::Config;

use crate::{
  error::OhNo,
  kakoune::{selection::Sel, session::Session, text_objects::OperationMode},
  protocol::{
    request::Metadata,
    response::{EnabledLang, Payload, Response},
  },
  tree_sitter::{languages::Languages, nav, state::Trees},
};

use super::triple_buffer::TripleBufferReader;

/// Commands for the handler.
///
/// The handler receives commands from the async IO handler, execute them and eventually replies back to
/// Kakoune by sending responses to the response queue.
#[derive(Debug)]
pub enum Command {
  /// Session initiated.
  SessionInit { metadata: Metadata },

  /// Session closed.
  SessionEnd { metadata: Metadata },

  /// Server shutdown.
  Shutdown,

  /// Buffer metadata changed.
  BufferMetadata {
    metadata: Metadata,
    lang: String,
    fifo_path: PathBuf,
    sentinel: String,
  },

  /// Buffer closed.
  BufferClose { metadata: Metadata },

  /// Buffer updated.
  ///
  /// The `back_buffer_sender` should be used to send back a string of the buffer.
  BufferUpdate {
    metadata: Metadata,
    reader: TripleBufferReader,
  },

  /// Text objects selections.
  TextObjects {
    metadata: Metadata,
    pattern: String,
    selections: Vec<Sel>,
    mode: OperationMode,
  },

  /// Tree navigation.
  Nav {
    metadata: Metadata,
    selections: Vec<Sel>,
    dir: nav::Dir,
  },

  /// Server version.
  Version { metadata: Metadata },
}

/// Send commands to the handler.
#[derive(Debug)]
pub struct CommandSender {
  join_handle: Option<JoinHandle<()>>,
  sender: Option<Sender<Command>>,
}

impl CommandSender {
  pub fn send(&self, cmd: Command) -> Result<(), OhNo> {
    self
      .sender
      .as_ref()
      .unwrap()
      .send(cmd)
      .map_err(|_| OhNo::CannotSendCommand)
  }
}

impl Drop for CommandSender {
  fn drop(&mut self) {
    // this should cause the thread counterpart to exit
    self.sender = None;

    if let Some(join_handle) = self.join_handle.take()
      && join_handle.join().is_err()
    {
      log::error!("handler not properly closed");
    }
  }
}

/// Type responsible for handling tree-sitter changes.
///
/// This type is stateful, as requests might have side-effect (i.e. tree-sitter
/// parsing generates trees/highlighters that can be reused, for instance).
pub struct Handler {
  config: Config,
  trees: Trees,
  langs: Languages,
  with_highlighting: bool,
}

impl Handler {
  /// Create a new [`Handler`] and a [`CommandSender`] to send it commands.
  pub fn create(config: &Config, with_highlighting: bool) -> CommandSender {
    let (sender, cmds) = channel();

    let config = config.clone();
    let join_handle = thread::spawn(move || {
      let langs = Languages::new(&config);
      let handler = Self {
        config,
        trees: Trees::default(),
        langs,
        with_highlighting,
      };

      handler.start(cmds);
    });

    CommandSender {
      join_handle: Some(join_handle),
      sender: Some(sender),
    }
  }

  fn start(mut self, cmds: Receiver<Command>) {
    log::info!("handler started");

    while let Ok(cmd) = cmds.recv() {
      let resp = match cmd {
        Command::SessionInit { metadata } => Ok(Some(self.handle_session_begin(metadata))),

        Command::SessionEnd { metadata } => {
          self.handle_session_end(metadata);
          Ok(None)
        }

        Command::Shutdown => break,

        Command::BufferMetadata {
          metadata,
          lang,
          fifo_path,
          sentinel,
        } => self
          .handle_buffer_metadata(metadata, lang, fifo_path, sentinel)
          .map(Some),

        Command::BufferClose { metadata } => self.handle_buffer_close(metadata).map(|_| None),

        Command::BufferUpdate { metadata, reader } => {
          self.handle_full_buffer_update(metadata, reader)
        }

        Command::TextObjects {
          metadata,
          pattern,
          selections,
          mode,
        } => self
          .handle_text_objects(metadata, pattern, selections, mode)
          .map(Some),

        Command::Nav {
          metadata,
          selections,
          dir,
        } => self.handle_nav(metadata, selections, dir).map(Some),

        Command::Version { metadata } => Ok(Some(Response::from_req_metadata(
          metadata,
          Payload::Version(env!("VERSION").to_owned()),
        ))),
      };

      match resp {
        Ok(Some(resp)) => {
          self.respond_to_kak(resp);
        }

        Err(err) => {
          log::error!("error in handler: {err}");
        }

        _ => (),
      }
    }

    log::debug!("handler loop exiting");
  }

  /// Send a response back to Kakoune.
  ///
  /// The response is constructed from metadata and payload computed by a handler function.
  fn respond_to_kak(&self, resp: Response) {
    if let Err(err) = Session::send_response(resp) {
      log::error!("sending response to Kakoune failed: {err}");
    }
  }

  /// Initiate languages on session init.
  pub fn handle_session_begin(&mut self, metadata: Metadata) -> Response {
    let enabled_langs = self
      .config
      .languages
      .iter()
      .map(|(name, lang)| EnabledLang {
        name: name.to_owned(),
        remove_default_highlighter: lang.remove_default_highlighter.into(),
        filetype_hook: lang.filetype_hook.into(),
        aliases: lang.aliases.clone(),
      })
      .collect();

    Response::from_req_metadata(metadata, Payload::Init { enabled_langs })
  }

  pub fn handle_session_end(&mut self, metadata: Metadata) {
    self.trees.clean_session(&metadata.session);
  }

  /// Update buffer metadata change.
  pub fn handle_buffer_metadata(
    &mut self,
    metadata: Metadata,
    lang: String,
    fifo_path: PathBuf,
    sentinel: String,
  ) -> Result<Response, OhNo> {
    let lang = self.langs.get(lang)?;
    let id = metadata.to_buffer_id()?;

    // ensure the tree exists
    self.trees.compute(&self.langs, lang, &id)?;

    let payload = Payload::BufferSetup {
      fifo_path,
      sentinel,
    };
    Ok(Response::from_req_metadata(metadata, payload))
  }

  /// Handle buffer close.
  pub fn handle_buffer_close(&mut self, metadata: Metadata) -> Result<(), OhNo> {
    let id = metadata.to_buffer_id()?;
    self.trees.delete_tree(&id);
    Ok(())
  }

  /// Update a full buffer update.
  pub fn handle_full_buffer_update(
    &mut self,
    metadata: Metadata,
    reader: TripleBufferReader,
  ) -> Result<Option<Response>, OhNo> {
    let id = metadata.to_buffer_id()?;
    let tree = self.trees.get_tree_mut(&id)?;

    // update the tree
    let timer = Instant::now();
    if !tree.update_buf(&self.langs, reader)? {
      // nothing changed; abort early
      return Ok(None);
    }

    log::debug!(
      "buffer tree {id:?} was recomputed in {}us",
      timer.elapsed().as_micros()
    );

    // run any additional post-processing on the buffer
    if !self.with_highlighting {
      return Ok(None);
    }

    let timer = Instant::now();
    let ranges = tree.highlight(&self.langs)?;

    log::debug!(
      "highlights were recomputed in {}us",
      timer.elapsed().as_micros()
    );

    // if there is nothing to highlight, we return nothing
    if ranges.is_empty() {
      log::debug!("no highlights to return");
      return Ok(None);
    }

    let resp = Response::new(
      id.session(),
      None,
      id.buffer().to_owned(),
      Payload::Highlights {
        faces: self.langs.faces().clone(),
        ranges,
      },
    );

    Ok(Some(resp))
  }

  pub fn handle_text_objects(
    &mut self,
    metadata: Metadata,
    pattern: String,
    selections: Vec<Sel>,
    mode: OperationMode,
  ) -> Result<Response, OhNo> {
    let id = metadata.to_buffer_id()?;
    log::debug!("text-objects {pattern}, mode {mode:?} for buffer {id:?}");

    let tree_state = self.trees.get_tree(&id)?;
    let lang = self.langs.get(tree_state.lang_name())?;
    let sels = tree_state.text_objects(lang, &pattern, &selections, &mode)?;

    log::trace!("text-objects selections: {sels:?}");

    Ok(Response::new(
      metadata.session,
      metadata.client,
      None,
      Payload::Selections { sels },
    ))
  }

  pub fn handle_nav(
    &mut self,
    metadata: Metadata,
    selections: Vec<Sel>,
    dir: nav::Dir,
  ) -> Result<Response, OhNo> {
    let id = metadata.to_buffer_id()?;
    log::debug!("nav {dir:?} for buffer {id:?}");

    let tree_state = self.trees.get_tree(&id)?;
    let sels = tree_state.tree_sitter_nav_tree(&selections, dir);

    Ok(Response::new(
      metadata.session,
      metadata.client,
      None,
      Payload::Selections { sels },
    ))
  }
}
