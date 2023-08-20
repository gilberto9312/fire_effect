use raylib_ffi::{
    InitWindow, 
    CloseWindow, 
    WindowShouldClose,
    BeginDrawing,
    EndDrawing,
    Color,
    Image, 
    enums,
    LoadTextureFromImage,
    Texture2D,
    DrawTexture,
    UpdateTexture
};
use raylib_ffi::colors::WHITE;
use std::ffi::c_void;

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


fn load_texture_from_image(img: Image) -> Texture2D {
    unsafe {
        LoadTextureFromImage(img)
    }
}

fn draw_texture(texture: Texture2D, pos_x: i32, pos_y: i32, tint: Color) {
    unsafe {
        DrawTexture(texture, pos_x, pos_y, tint)
    }
}

fn update_texture(texture: Texture2D, data: &[Color] ) {
    unsafe {
        UpdateTexture(texture, data.as_ptr() as *const c_void)
    }
}

fn draw_next_frame(screen: &mut [Color], pal: &[Color]) {
    for i in 0..pal.len() {
        let init = i*400 + 50;
        let pixel = &mut screen[init..(init+4)];
        pixel[0] = pal[i];
        pixel[1] = pal[i];
        pixel[2] = pal[i];
        pixel[3] = pal[i];

    }
    
}

fn generate_palette() ->[Color; 256] {
    let mut pal = [Color{r:0,g:0,b:0,a:0xFF}; 256]; 
    for i in 0..=84 {
        pal[i].r = (i * (0xFF / 85)) as u8;
    }
    for i in 85..=169 {
        pal[i].r = 255;
        pal[i].g = ((i - 85) * (0xFF / 85)) as u8;
    }
    for i in 170..=255 {
        pal[i].r = 255;
        pal[i].g = 255;
        pal[i].b = ((i - 85) * (0xFF / 85)) as u8;
    }
    pal
}

fn main() {
   
    init_gui();

     let mut screen_buffer_data= [ Color{r:0, g:0, b:0, a: 0xFF}; 400*300 ];

    let screen_buffer = Image{
        data: screen_buffer_data.as_mut_ptr() as *mut c_void,
        width: 400,
        height: 300,
        format: enums::PixelFormat::R8g8b8a8 as i32,
        mipmaps: 1
    };

    let screen_buffer_texture = load_texture_from_image(screen_buffer);
    let pixel = &mut screen_buffer_data[200*400 + 200];
    *pixel = Color{r:0xFF, g:0, b:0, a: 0xFF};

    while ! window_shold_close() {
        begin_drawing();
        update_texture(screen_buffer_texture, &screen_buffer_data);
        draw_texture(screen_buffer_texture, 0,0, WHITE);
        end_drawing()
    }
    println!("hello world");
    close_window()
    
}
