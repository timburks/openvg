extern crate libc;

type Fontinfo = [u8; 2020];

#[link(name = "shapes")]
extern {
    static SansTypeface: Fontinfo;
    fn init(x: &u32, y: &u32);
    fn finish();
    fn Start(x: u32, y: u32);
    fn Background(r:u32, g:u32, b:u32);
    fn Fill(r:u32, g:u32, b:u32, a:f32);
    fn Circle(r:f32, g:f32, b:f32);
    fn TextMid(x:f32, y:f32, s:*const libc::c_char, f:Fontinfo, size:u32);
    fn End();
    fn puts(s:*const libc::c_char);
}

fn main() {
    let width : u32 = 0;
    let height : u32 = 0;
    unsafe { init(&width, &height); }
    println!("{} {}", width, height);
    let c_str = std::ffi::CString::new("hello, world").unwrap();
    let c_ptr = c_str.as_ptr();
    unsafe {
        Start(width, height);                              // Start the picture
        Background(0, 0, 0);                               // Black background
        Fill(44, 77, 232, 1.0);                              // Big blue marble
        Circle(width as f32 / 2.0, 0 as f32, width as f32);                       // The "world"
        Fill(255, 255, 255, 1.0);                            // White text
        puts(c_ptr);
        TextMid(width as f32 / 2.0, 
                height as f32 * 0.7, 
                c_ptr,
                SansTypeface, 
                width/15);
        End();
    }
    loop {
    }
    unsafe { finish(); }
}
