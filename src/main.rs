use raylib_ffi::{
    InitWindow, 
    CloseWindow, 
    WindowShouldClose,
    BeginDrawing,
    EndDrawing,
    DrawPixel,
    Color
};

fn init_gui() {
    let width: i32 = 400;
    let height = 300;
    let title = "fire".as_ptr() as *const i8;
    unsafe {
        InitWindow(width, height, title);
    }
}

fn window_shold_close() -> bool {
    unsafe { WindowShouldClose() }
}

fn begin_drawing() {
    unsafe {
        BeginDrawing();
    }
}

fn end_drawing() {
    unsafe {
        EndDrawing()
    }
}

fn close_window() {
    unsafe {
        CloseWindow();
    }
}

fn draw_pixel(x: i32, y: i32, color: Color) {
    unsafe {
        DrawPixel(x, y, color);
    }
}


fn main() {
   
    init_gui();
    while ! window_shold_close() {
        begin_drawing();
        draw_pixel(200, 200, Color{r: 0xFF, g: 0, b: 0, a: 0xFF});
        end_drawing()
    }
    println!("hello world");
    close_window()
    
}
