use crate::states::menu::UiState;
use amethyst::ecs::prelude::Entity;
use amethyst::input::is_key_down;
use amethyst::prelude::*;
use amethyst::ui::UiEvent;
use amethyst::ui::UiEventType;
use amethyst::ui::UiFinder;
use amethyst::winit::VirtualKeyCode;

const ROOT_ID: &str = "quit";
const BUTTON_YES_ID: &str = "quit.yes";
const BUTTON_NO_ID: &str = "quit.no";

pub struct QuitState {
    ui_root: Option<Entity>,
    button_yes: Option<Entity>,
    button_no: Option<Entity>,
}

impl QuitState {
    pub fn new() -> Self {
        return Self {
            ui_root: None,
            button_yes: None,
            button_no: None,
        };
    }
}

impl SimpleState for QuitState {
    fn on_start(&mut self, mut data: StateData<GameData>) {
        self.ui_root = self.find_ui_root(&mut data.world);
        self.on_start_or_resume(&mut data.world);

        data.world.exec(|finder: UiFinder| {
            self.button_yes = finder.find(BUTTON_YES_ID);
            self.button_no = finder.find(BUTTON_NO_ID);
        });
    }

    fn on_pause(&mut self, mut data: StateData<GameData>) {
        self.on_stop_or_pause(&mut data.world);
    }

    fn on_resume(&mut self, mut data: StateData<GameData>) {
        self.on_start_or_resume(&mut data.world);
    }

    fn on_stop(&mut self, mut data: StateData<GameData>) {
        self.button_yes = None;
        self.button_no = None;
        self.on_stop_or_pause(&mut data.world);
    }

    fn handle_event(&mut self, _: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        match event {
            StateEvent::Window(event) => {
                if is_key_down(&event, VirtualKeyCode::Escape) {
                    return Trans::Pop;
                }
            }
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                if Some(target) == self.button_yes {
                    return Trans::Quit;
                }

                if Some(target) == self.button_no {
                    return Trans::Pop;
                }
            }
            _ => {}
        }

        return Trans::None;
    }
}

impl UiState for QuitState {
    fn get_ui_root_id(&self) -> &'static str {
        return ROOT_ID;
    }

    fn get_ui_root(&self) -> Option<Entity> {
        return self.ui_root;
    }
}
