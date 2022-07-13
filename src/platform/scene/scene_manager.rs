use crate::platform::scene::scene;
use crate::{eng_debug, eng_trace};

pub struct SceneManager {
    scenes: Vec<Box<dyn scene::Scene>>,

    current_scene: Option<Box<dyn scene::Scene>>,
    next_id: u16,
}

impl SceneManager {
    pub(crate) fn new() -> SceneManager {
        let scenes = Vec::new();

        SceneManager {
            scenes,
            current_scene: None,
            next_id: 0,
        }
    }

    pub fn register_scene<S: 'static + scene::Scene>(mut self, scene: S) -> u16 {
        self.scenes.push(Box::new(scene));
        let scene_id = self.next_id;

        self.next_id += 1;

        scene_id
    }

    // transition will switch the current scene to the scene identified by
    // the scene_id (if it exists) -- TODO: don't fail silently
    pub fn transition(mut self, scene_id: u16) {
        eng_trace!("performing a scene transition");

        match self.current_scene {
            None => (),
            _ => {
                let curr_scene = self.current_scene.unwrap();
                eng_debug!("transitioning out of scene {}", curr_scene.get_id());

                curr_scene.transition_out();
            }
        }

        eng_trace!("looking for scene matching id {}", scene_id);
        let mut scene = None;
        for s in &self.scenes {
            if s.get_id() == scene_id {
                scene = Some(s);
                break;
            }
        }

        match scene {
            None => {
                eng_debug!("no scene matched id {}", scene_id);
                return;
            }
            _ => {
                eng_debug!("found scene with id {}, transitioning in", scene_id);
                scene.unwrap().transition_in();
            }
        }
    }

    pub fn update(self) {
        let curr_scene = self.current_scene.unwrap();
        curr_scene.update();
    }
}
