use crate::{
    core::{
        game_state::GameState,
        perk::{Perk, PerkData, PerkType},
    },
    systems::input::InputState,
};

pub struct DepoState {
    pub perk_choices: Vec<Perk>,
    pub perk_choice_index: usize,
}

impl Default for DepoState {
    fn default() -> Self {
        DepoState::new()
    }
}

impl DepoState {
    pub fn new() -> Self {
        Self {
            perk_choices: vec![
                Perk {
                    kind: PerkType::Shield,
                    duration: None,
                    stacks: 1,
                    data: PerkData::Shield {
                        max_charges: 1,
                        charges: 1,
                        recharge_time: 1.0,
                        recharge_timer: 0.0,
                    },
                },
                Perk {
                    kind: PerkType::Shield,
                    duration: None,
                    stacks: 1,
                    data: PerkData::Shield {
                        max_charges: 1,
                        charges: 1,
                        recharge_time: 1.0,
                        recharge_timer: 0.0,
                    },
                },
            ],
            perk_choice_index: 0,
        }
    }
    pub fn is_selected(&self, i: usize) -> bool {
        i == self.perk_choice_index
    }
}

pub fn update(state: &mut DepoState, input: &InputState) -> Option<GameState> {
    if state.perk_choices.is_empty() {
        return None;
    }

    let h_movement = input.ui_horizontal_axis();

    if h_movement != 0 {
        state.perk_choice_index = (state.perk_choice_index as isize + h_movement)
            .clamp(0, state.perk_choices.len() as isize - 1)
            .try_into()
            .unwrap()
    }

    None
}
