use leptos::*;

fn main() {
    let (poos, set_poos) = create_signal(1 as i128);
    let (farts, set_farts) = create_signal(1.0 as f64);
    mount_to_body(move || {
        view! {
            <h1>"A card on the ground"</h1>
            <h1>"Sausage dog on a rainbow on a monster"</h1>
            <h1>"Hello Liam, do your best at school and swimming!"</h1>
            <h1>"CUBBY HOUSES! Chloe is awesome at cubby houses!"</h1>
            <button on:click=move |_| {
                set_poos.update(|n| *n += 1)
            }>"Click me to add poop"</button>
            <button on:click=move |_| {
                set_poos.update(|n| *n -= 1)
            }>"Click me to subtract poop"</button>
            <button on:click=move |_| {
                set_poos.update(|n| *n /= 2)
            }>"Click me to half poop"</button>
            <button on:click=move |_| {
                set_poos.update(|n| *n *= 2)
            }>"Click me to double poop"</button>
            <h1
                style:color="tan"
                style:background-color="black"
            >
                "I have pooped "
                {move || poos.get()}
                " times 💩"
            </h1>
            <button on:click=move |_| {
                set_farts.update(|n| *n += 1.0)
            }>"Click me to add fart"</button>
            <button on:click=move |_| {
                set_farts.update(|n| *n -= 1.0)
            }>"Click me to subtract fart"</button>
            <button on:click=move |_| {
                set_farts.update(|n| *n /= 2.0)
            }>"Click me to half fart"</button>
            <button on:click=move |_| {
                set_farts.update(|n| *n *= 2.0)
            }>"Click me to double fart"</button>
            <h1
                style:color="lime"
                style:background-color="black"
            >
                "I have farted "
                {move || farts.get()}
                " times 💨"
            </h1>
            <h1>
                "The largest i32 integer is "
                {i32::MAX}
            </h1>
            <h1>
                "The largest u64 integer is "
                {u64::MAX}
            </h1>
            <h1>
                "The largest u128 integer is "
                {u128::MAX}
            </h1>
        }
    })
}
