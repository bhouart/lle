use crate::AgentId;
use pyo3::prelude::*;

#[derive(Clone, Debug, PartialEq)]
#[pyclass(name = "EventType", module = "lle")]
pub enum PyEventType {
    #[pyo3(name = "AGENT_EXIT")]
    AgentExit,
    #[pyo3(name = "GEM_COLLECTED")]
    GemCollected,
    #[pyo3(name = "AGENT_DIED")]
    AgentDied,
}

#[derive(Clone)]
#[pyclass(name = "WorldEvent", module = "lle")]
pub struct PyWorldEvent {
    #[pyo3(get, set)]
    event_type: PyEventType,
    // pos: Position,
    #[pyo3(get, set)]
    agent_id: AgentId,
}

impl PyWorldEvent {
    pub fn new(event_type: PyEventType, agent_id: AgentId) -> Self {
        Self {
            event_type,
            agent_id,
        }
    }
}

#[pymethods]
impl PyWorldEvent {
    fn __str__(&self) -> String {
        format!("{:?}, agent id: {}", self.event_type, self.agent_id)
    }

    fn __repr__(&self) -> String {
        self.__str__()
    }
}
