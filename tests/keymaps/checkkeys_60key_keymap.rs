#[rustfmt::skip]
pub const KEY_DEFINITIONS: [Key; 60] = {
    #[cfg(not(feature = "usbd-human-interface-device"))]
    compile_error!("usbd-human-interface-device feature is not enabled");

    use usbd_human_interface_device::page::Keyboard::*;

    let codes = [
        Grave, Keyboard1, Keyboard2, Keyboard3, Keyboard4, Keyboard5, Keyboard6,   Keyboard7, Keyboard8, Keyboard9, Keyboard0,  DeleteBackspace,
        LeftBrace, Q, W, E, R, T, Y, U, I, O, P, RightBrace,
        Escape, A, S, D, F, G, H,  J, K, L, Semicolon, ReturnEnter,
        LeftShift, Z, X, C, V, B, N, M, Comma, Dot, ForwardSlash, RightShift,
        A, B, C, D, E, F, G, H, I, J, K, L,
    ];

    let mut key_codes = [Key::Simple(simple::Key(0x00)); 60];
    let mut i = 0;
    while i < 60 {
        key_codes[i] = Key::Simple(simple::Key(codes[i] as u8));
        i += 1;
    }
    key_codes
};
