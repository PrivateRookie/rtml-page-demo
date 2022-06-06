use rtml::{
    mount_body, ref_subs, ref_update, t_attr,
    tags::{a, button, div, hr, p, span, strong},
    EventKind::Click,
    IntoReactive,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    let count = 0usize.reactive();

    let display = ref_subs!(count => {
        let count = count.val();
        let content = if *count >= 1 {
            format!("count {count} times")
        } else {
            format!("count {count} time")
        };
        content
    });

    let incr = ref_update!(count => |_| {
        *count.val_mut() += 1;
        true
    });

    let dec = ref_update!(count => |_| {
        if *count.val() > 0 {
            *count.val_mut() -= 1;
            true
        } else {
            false
        }
    });

    let counter = div((
        p(display),
        button("+1").on(Click, incr),
        button("-1").on(Click, dec),
        hr(()),
        strong((
            span("Written by "),
            a((
                t_attr! {href="https://github.com/PrivateRookie/rtml"},
                "rtml",
            )),
        )),
    ));

    mount_body(counter).unwrap();
}
