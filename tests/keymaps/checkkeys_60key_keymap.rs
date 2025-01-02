use seq_macro::seq;
seq!(I in 0..60 {
    type KeyDefinitionsType = tuples::Keys60<
        #(
            Key,
        )*
    >;
});
pub const KEY_DEFINITIONS: KeyDefinitionsType = {
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

    seq!(I in 0..60 {
        tuples::Keys60::new((
            #(
                Key::Simple(simple::Key(codes[I] as u8)),
            )*
        ))
    })
};
