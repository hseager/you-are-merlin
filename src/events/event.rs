use crate::actions::Action;

pub trait Event {
    fn prompt(&self) -> String;
    fn actions(&self) -> Vec<Action>;
}
