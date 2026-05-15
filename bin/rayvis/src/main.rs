use maak_rico::glam;
use maak_rico::prelude::*;

fn main() {
    Engine::new(RaylibBackend::default(), Scene::new().on_setup(setup)).run();
}

fn setup(scene: &mut Scene<RaylibBackend>, _cx: &mut RaylibBackend) {
    let camera =
        scene.spawn(&[Transform::default(), Camera::default(), CameraController::default()]);
    scene.set_active_camera(Some(camera));

    let _entity = scene.spawn(&[Transform::default()]);
}
