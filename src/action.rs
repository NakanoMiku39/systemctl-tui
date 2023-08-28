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
  RefreshServicesAndLog,
  SetServices(Vec<UnitStatus>),
  EnterMode(Mode),
  CancelTask,
  ToggleHelp,
  SetLogs { unit_name: String, logs: String },
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
