use crate::{
    agent::{Agent, AgentId},
    rendering::{TileVisitor, VisitorData},
    RuntimeWorldError, WorldEvent,
};

use super::{Floor, Tile};

pub struct Start {
    floor: Floor,
    agent_id: AgentId,
}

impl Start {
    pub fn new(agent_id: AgentId) -> Self {
        Self {
            floor: Floor::default(),
            agent_id,
        }
    }

    pub fn agent_id(&self) -> AgentId {
        self.agent_id
    }
}

impl Tile for Start {
    fn pre_enter(&mut self, agent: &Agent) -> Result<(), RuntimeWorldError> {
        self.floor.pre_enter(agent)
    }
    fn reset(&mut self) {
        self.floor.reset();
    }

    fn enter(&mut self, agent: &mut Agent) -> Option<WorldEvent> {
        self.floor.enter(agent);
        None
    }

    fn leave(&mut self) -> AgentId {
        self.floor.leave()
    }

    fn agent(&self) -> Option<AgentId> {
        self.floor.agent()
    }

    fn accept(&self, _visitor: &dyn TileVisitor, _data: &mut VisitorData) {
        // Nothing to do
    }
}

#[derive(Default)]
pub struct Exit {
    floor: Floor,
}

impl Tile for Exit {
    fn pre_enter(&mut self, agent: &Agent) -> Result<(), RuntimeWorldError> {
        self.floor.pre_enter(agent)
    }

    fn reset(&mut self) {
        self.floor.reset();
    }

    fn enter(&mut self, agent: &mut Agent) -> Option<WorldEvent> {
        self.floor.enter(agent);
        if !agent.has_arrived() {
            agent.arrive();
            return Some(WorldEvent::AgentExit {
                agent_id: agent.id(),
            });
        }
        None
    }

    fn leave(&mut self) -> AgentId {
        self.floor.leave()
    }

    fn agent(&self) -> Option<AgentId> {
        self.floor.agent()
    }

    fn accept(&self, _visitor: &dyn TileVisitor, _data: &mut VisitorData) {
        // Nothing to do
    }
}
