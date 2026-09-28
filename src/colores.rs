use macroquad::prelude::*;
#[macroquad::main("Cubo RGB Interactivo")]
async fn main() {
    let mut rotacion_x = 0.0 as f32;
    let mut rotacion_y = 0.0 as f32;
    let mut velocidad_rotacion = 0.0 as f32;
    let desaceleracion = 0.985 as f32;
    let impulso = 0.15 as f32;
    loop {
        let dir_y =rand::gen_range(-1.0, 1.0) as f32;
        let dir_ac = rand::gen_range(-1.0, 1.0);
        clear_background(BLACK);
        set_camera(&Camera3D {
            position: vec3(0.0, 1.5, 3.0),
            target: vec3(0.0, 0.0, 0.0), // Recomendado: indica hacia dónde mira la cámara
            up: vec3(0.0, 1.0, 0.0),     // Corregido: vector unitario hacia arriba
            ..Default::default()
        });
        if is_mouse_button_pressed(MouseButton::Left) {
            velocidad_rotacion = impulso;
        }
        if velocidad_rotacion > 0.001{
            rotacion_x += velocidad_rotacion;
            rotacion_y += velocidad_rotacion * dir_y;
            velocidad_rotacion *= desaceleracion +dir_ac;
        } else {
            velocidad_rotacion = 0.0;
        }
        let intensidad = (velocidad_rotacion/ impulso).clamp(0.0, 0.0);
        draw_line_3d(vec3(-1.0, 0.0, 0.0), vec3(1.0, 1.0, 0.0), RED);
        // set_default_camera();
        draw_text("Cubo Rotando", 50.0, 30.0, 24.0, WHITE);
        next_frame().await
    }
}