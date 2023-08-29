use crate::{components::home::Mode, systemd::UnitStatus};

#[derive(Debug, Clone)]
pub enum Action {
  Quit,
  Resume,
  Suspend,
  Render,
  SpinnerTick,
  Resize(u16, u16),
  ToggleShowLogger,
  RefreshServices,
  SetServices(Vec<UnitStatus>),
  EnterMode(Mode),
  EnterError { err: String },
  CancelTask,
  ToggleHelp,
  SetLogs { unit_name: String, logs: Vec<String> },
  AppendLogLine { unit_name: String, line: String },
  StartService(String),
  StopService(String),
  RestartService(String),
  ReloadService(String),
  EnableService(String),
  DisableService(String),
  ScrollUp(u16),
  ScrollDown(u16),
  ScrollToTop,
  ScrollToBottom,
  Noop,
}
