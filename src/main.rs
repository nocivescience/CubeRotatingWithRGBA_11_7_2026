use macroquad::prelude::*;

#[macroquad::main("Cubo RGB Interactivo")]
async fn main() {
    // Variables de estado del cubo
    let mut rotacion_x = 0.0;
    let mut rotacion_y = 0.0;
    let mut velocidad_rotacion = 0.0;
    
    // Configuración física
    let desaceleracion = 0.985; 
    let impulso = 0.15;         
    let mut color_actual = WHITE;

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

    loop {
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
            rotacion_y += velocidad_rotacion * 0.7;
            velocidad_rotacion *= desaceleracion;
        } else {
            velocidad_rotacion = 0.0;
        }

        // 4. Modificar el RGB dinámicamente basado en la velocidad
        let intensidad = velocidad_rotacion / impulso; 
        color_actual = Color::new(
            0.3 + (intensidad * 0.7), 
            0.2 + (intensidad * 0.5), 
            0.5 + (intensidad * 0.5), 
            1.0,
        );

        // 5. Calcular la rotación usando Quaternions (Álgebra de glam pura, 100% estable)
        let q_rotacion = Quat::from_rotation_x(rotacion_x) * Quat::from_rotation_y(rotacion_y);

        // Rotamos todos los vértices localmente antes de dibujarlos
        let mut vertices_rotados = [Vec3::ZERO; 8];
        for i in 0..8 {
            vertices_rotados[i] = q_rotacion * vertices_locales[i];
        }

        // Dibujamos las líneas del cubo usando los vértices ya rotados en el espacio 3D
        for &(inicio, fin) in &aristas {
            draw_line_3d(vertices_rotados[inicio], vertices_rotados[fin], color_actual);
        }

        // 6. Volver a la cámara 2D para la interfaz de texto
        set_default_camera();
        draw_text("Haz CLIC para hacer girar el cubo", 20.0, 30.0, 24.0, WHITE);
        draw_text(&format!("Velocidad: {:.4}", velocidad_rotacion), 20.0, 60.0, 20.0, GRAY);

        next_frame().await
    }
}