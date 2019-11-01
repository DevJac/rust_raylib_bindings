use std::ffi::CString;

fn main() {
    unsafe {
        let screen_width = 800;
        let screen_height = 450;
        let title = CString::new("Basic Window Example").unwrap();
        raylib_sys::InitWindow(screen_width, screen_height, title.as_ptr());
        raylib_sys::SetTargetFPS(60);
        while !raylib_sys::WindowShouldClose() {
            raylib_sys::BeginDrawing();
            raylib_sys::ClearBackground(raylib_sys::RAYWHITE);
            let msg = CString::new("Congrats! You created your first window!").unwrap();
            raylib_sys::DrawText(msg.as_ptr(), 190, 200, 20, raylib_sys::LIGHTGRAY);
            raylib_sys::EndDrawing();
        }
        raylib_sys::CloseWindow();
    }
}
