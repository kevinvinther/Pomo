use crate::domain::session_kind::SessionKind;
use crate::domain::time::Seconds;

pub enum SessionState {
    Idle, 
    Running {
        kind: SessionKind,
        remaining: Seconds,
    },
    Paused {
        kind: SessionKind,
        remaining: Seconds,
    },
    Finished {
        kind: SessionKind,
    },
    Stopped,
}

impl SessionState {
    pub fn start(kind: SessionKind, duration: Seconds) -> Self {
        SessionState::Running {
            kind,
            remaining: duration,
        }
    }
    
    pub fn tick(self) -> Self {
        match self {
            SessionState::Running { kind, remaining } => {
                if remaining.as_seconds() > 1 {
                    SessionState::Running {
                        kind,
                        remaining: Seconds(remaining.as_seconds() -1),
                    }
                } else {
                    SessionState::Finished { kind }
                }
            }
            other => other,
        }
    }
    
    pub fn pause(self) -> Self {
        match self {
            SessionState::Running { kind, remaining } => {
                SessionState::Paused { kind, remaining }
            },
            other => other,
        }
    }
    
    pub fn resume(self) -> Self {
        match self {
            SessionState::Paused { kind, remaining } => {
                SessionState::Running { kind, remaining }
            },
            other => other,
        }
    }
    
    pub fn skip(self) -> Self {
        match self {
            SessionState::Running { kind, .. } | SessionState::Paused { kind, ..} => SessionState::Finished { kind },
            other => other,
        }
    }
    
    pub fn stop(self) -> Self {
        SessionState::Stopped
    }
    
    pub fn is_active(&self) -> bool {
        matches!(self, SessionState::Running { .. } | SessionState::Paused { .. })
    }
    
    pub fn remaining(&self) -> Option<Seconds> {
        match self {
            SessionState::Running { remaining, .. }
            | SessionState::Paused { remaining, .. } => Some(*remaining),
            _ => None,
        }
    }
}