use macroquad::prelude::*;

#[macroquad::main("Cubo RGB Interactivo")]
async fn main() {
    // Variables de estado del cubo
    let mut rotacion_x: f32 = 0.0;
    let mut rotacion_y: f32 = 0.0;
    let mut velocidad_rotacion: f32 = 0.0;

    // Configuración física
    let desaceleracion: f32 = 0.985;
    let impulso: f32 = 0.15;

    // Definimos los 8 vértices de un cubo de tamaño 1.0 centrado en el origen (0,0,0)
    let vertices_locales = [
        vec3(-0.5, -0.5, -0.5), // 0
        vec3( 0.5, -0.5, -0.5), // 1
        vec3( 0.5,  0.5, -0.5), // 2
        vec3(-0.5,  0.5, -0.5), // 3
        vec3(-0.5, -0.5,  0.5), // 4
        vec3( 0.5, -0.5,  0.5), // 5
        vec3( 0.5,  0.5,  0.5), // 6
        vec3(-0.5,  0.5,  0.5), // 7
    ];

    // Definimos las 12 aristas que conectan los vértices para formar el cubo
    let aristas = [
        (0, 1), (1, 2), (2, 3), (3, 0), // Cara trasera
        (4, 5), (5, 6), (6, 7), (7, 4), // Cara delantera
        (0, 4), (1, 5), (2, 6), (3, 7), // Conexiones entre caras
    ];

    // Un color base distinto para cada una de las 12 aristas.
    // Generado repartiendo el matiz (hue) uniformemente en la rueda de color.
    let colores_aristas: [Color; 12] = {
        let mut colores = [WHITE; 12];
        for i in 0..12 {
            let hue = i as f32 / 12.0; // 0.0 a 1.0
            colores[i] = color_from_hue(hue);
        }
        colores
    };

    loop {
        let dir_y = rand::gen_range(-1.0, 1.0);
        let dir_ac = rand::gen_range(-0.2, 0.2);
        clear_background(BLACK);

        // 1. Configurar la cámara 3D estándar
        set_camera(&Camera3D {
            position: vec3(0.0, 1.5, 3.0),
            up: vec3(0.0, 1.0, 0.0),
            target: vec3(0.0, 0.0, 0.0),
            ..Default::default()
        });

        // 2. Detectar interacción (Clic izquierdo)
        if is_mouse_button_pressed(MouseButton::Left) {
            velocidad_rotacion = impulso;
        }

        // 3. Actualizar física de rotación
        if velocidad_rotacion > 0.001 {
            rotacion_x += velocidad_rotacion;
            rotacion_y += velocidad_rotacion * dir_y;
            velocidad_rotacion *= desaceleracion + dir_ac;
        } else {
            velocidad_rotacion = 0.0;
        }

        // 4. Intensidad de brillo basada en la velocidad (para mezclar con el color de cada arista)
        let intensidad: f32 = (velocidad_rotacion / impulso).clamp(0.0, 1.0);

        // 5. Calcular la rotación usando Quaternions
        let q_rotacion = Quat::from_rotation_x(rotacion_x) * Quat::from_rotation_y(rotacion_y);

        // Rotamos todos los vértices localmente antes de dibujarlos
        let mut vertices_rotados = [Vec3::ZERO; 8];
        for i in 0..8 {
            vertices_rotados[i] = q_rotacion * vertices_locales[i];
        }

        // Dibujamos cada arista con su propio color, atenuado por un brillo base
        // + un extra proporcional a la velocidad de rotación
        let brillo_base = 0.4;
        for (idx, &(inicio, fin)) in aristas.iter().enumerate() {
            let c = colores_aristas[idx];
            let factor = brillo_base + (1.0 - brillo_base) * intensidad;
            let color_final = Color::new(c.r * factor, c.g * factor, c.b * factor, 1.0);
            draw_line_3d(vertices_rotados[inicio], vertices_rotados[fin], color_final);
        }

        // 6. Volver a la cámara 2D para la interfaz de texto
        set_default_camera();
        draw_text("Haz CLIC para hacer girar el cubo", 20.0, 30.0, 24.0, WHITE);
        draw_text(&format!("Velocidad: {:.4}", velocidad_rotacion), 20.0, 60.0, 20.0, GRAY);

        if dir_y < -0.9 {
            draw_text(&format!("{:.2}", dir_y), 200.0, 20.0, 20.0, Color::new(1.0, 0.0, 0.0, 1.0));
        }

        next_frame().await
    }
}

/// Convierte un matiz (hue) en [0.0, 1.0] a un color RGB saturado y brillante.
fn color_from_hue(hue: f32) -> Color {
    let h = hue * 6.0;
    let x = 1.0 - (h % 2.0 - 1.0).abs();
    let (r, g, b) = match h as i32 {
        0 => (1.0, x, 0.0),
        1 => (x, 1.0, 0.0),
        2 => (0.0, 1.0, x),
        3 => (0.0, x, 1.0),
        4 => (x, 0.0, 1.0),
        _ => (1.0, 0.0, x),
    };
    Color::new(r, g, b, 1.0)
}