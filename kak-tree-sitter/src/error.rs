use std::{fmt, io, path::PathBuf};

use kak_tree_sitter_config::error::ConfigError;
use log::SetLoggerError;
use mio::Token;
use thiserror::Error;

use crate::kakoune::buffer::BufferId;

#[derive(Debug, Error)]
pub enum OhNo {
  #[error("nothing to do; please either use --server or --request")]
  NothingToDo,

  #[error("no runtime directory")]
  NoRuntimeDir,

  #[error("cannot initialize logging: {err}")]
  LoggerInit {
    #[from]
    err: SetLoggerError,
  },

  #[error("configuration error: {err}")]
  ConfigError {
    #[from]
    err: ConfigError,
  },

  #[error("cannot create directory {dir}: {err}")]
  CannotCreateDir { dir: PathBuf, err: io::Error },

  #[error("cannot create file {file}: {err}")]
  CannotCreateFile { file: PathBuf, err: io::Error },

  #[error("cannot write to file {file}: {err}")]
  CannotWriteFile { file: PathBuf, err: io::Error },

  #[error("cannot start daemon: {err}")]
  CannotStartDaemon { err: String },

  #[error("cannot set SIGINT handler: {err}")]
  SigIntHandlerError {
    #[from]
    err: ctrlc::Error,
  },

  #[error("cannot create FIFO: {err}")]
  CannotCreateFifo { err: String },

  #[error("cannot open non-blocking FIFO: {err}")]
  CannotOpenFifo { err: io::Error },

  #[error("cannot read FIFO: {err}")]
  CannotReadFifo { err: io::Error },

  #[error("poll error: {err}")]
  PollError { err: io::Error },

  #[error("cannot start server: {err}")]
  CannotStartServer { err: io::Error },

  #[error("cannot load grammar for language {lang}: {err}")]
  CannotLoadGrammar2 { lang: String, err: String },

  #[error("cannot load queries for language {lang}: {err}")]
  CannotLoadQueries { lang: String, err: String },

  #[error("cannot parse queries: {err}")]
  LangConfig {
    #[from]
    err: tree_house_bindings::query::ParseError,
  },

  #[error("error while operating on tree-sitter tree: {err}")]
  TreeHouse { err: String },

  #[cfg(feature = "direct-unix-socket")]
  #[error("Kakoune Unix socket error: {err:?}")]
  KakouneUnixSocketError { err: io::Error },

  #[error("Unix socket connection error: {err:?}")]
  UnixSocketConnectionError { err: io::Error },

  #[error("Unix socket read error: {err:?}")]
  UnixSocketReadError { err: io::Error },

  #[error("invalid request {req}: {err}")]
  InvalidRequest { req: String, err: String },

  #[error("cannot connect to server; is it running?: {err}")]
  CannotConnectToServer { err: io::Error },

  #[error("cannot send request: {err}")]
  CannotSendRequest { err: String },

  #[error("cannot send command")]
  CannotSendCommand,

  #[error("unknown language: {lang}")]
  UnknownLang { lang: String },

  #[error("language already failed to load: {lang}")]
  TriedLoadingOnceLang { lang: String },

  #[error("unknown buffer: {id:?}")]
  UnknownBuffer { id: BufferId },

  #[error("unknown buffer token: {tkn:?}")]
  UnknownToken { tkn: Token },

  #[error("text-objects not supported")]
  UnsupportedTextObjects,
  //
  #[error("no such {pattern} text-object query")]
  UnknownTextObjectQuery { pattern: String },

  #[error("response serialization error: {err}")]
  ResponseSerialization { err: fmt::Error },
}
