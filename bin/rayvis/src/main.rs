use maak_rico::prelude::*;

fn main() {
    let mut engine = Engine::new().with_scene(MainScene);
    engine.run();
}

struct MainScene;

impl Scene for MainScene {
    fn setup(&mut self, cx: &mut SceneData) {
        let _entity =
            cx.spawn(modname::Transform::default().with_translation(Vec3A::new(0.0, 0.0, 0.0)));
        let _entity =
            cx.spawn(modname::Transform::default().with_translation(Vec3A::new(1.0, 0.0, 0.0)));
        let _entity =
            cx.spawn(modname::Transform::default().with_translation(Vec3A::new(2.0, 0.0, 0.0)));
        let _entity =
            cx.spawn(modname::Transform::default().with_translation(Vec3A::new(3.0, 0.0, 0.0)));
    }
}
