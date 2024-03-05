use gloo_timers::future::TimeoutFuture;
use leptos::error::Error;
// use leptos::html::{button, div, span};
use leptos::leptos_dom::ev::{Event, MouseEvent, SubmitEvent};
use leptos::leptos_dom::{ErrorKey, Errors};
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

/// struct for NestedSignals example
#[derive(Debug, Clone)]
struct DatabaseEntry {
    key: String,
    value: RwSignal<i32>,
}

/// struct for MemorySlices example
#[derive(Debug, Clone)]
struct DatabaseEntry2 {
    key: String,
    value: i32,
}

#[component]
pub fn App() -> impl IntoView {
    let (name, set_name) = create_signal(String::new());
    let (greet_msg, set_greet_msg) = create_signal(String::new());

    let update_name = move |ev: Event| {
        let v = event_target_value(&ev);
        set_name.set(v);
    };

    let greet = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let name = name.get_untracked();
            if name.is_empty() {
                return;
            }

            let args = to_value(&GreetArgs { name: &name }).unwrap();
            // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
            let new_msg = invoke("greet", args).await.as_string().unwrap();
            set_greet_msg.set(new_msg);
        });
    };

    view! {
        <Router>
            <main class="container">
                <Routes>
                    <Route path="" view=FormExample/>
                </Routes>
                <div class="row">
                    <a href="https://tauri.app" target="_blank">
                        <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
                    </a>
                    <a href="https://docs.rs/leptos/" target="_blank">
                        <img src="public/leptos.svg" class="logo leptos" alt="Leptos logo"/>
                    </a>
                </div>

                <p>"Click on the Tauri and Leptos logos to learn more."</p>

                <p>
                    "Recommended IDE setup: "
                    <a href="https://code.visualstudio.com/" target="_blank">
                        "VS Code"
                    </a> " + " <a href="https://github.com/tauri-apps/tauri-vscode" target="_blank">
                        "Tauri"
                    </a> " + " <a href="https://github.com/rust-lang/rust-analyzer" target="_blank">
                        "rust-analyzer"
                    </a>
                </p>

                <form class="row" on:submit=greet>
                    <input id="greet-input" placeholder="Enter a name..." on:input=update_name/>
                    <button type="submit">"Greet"</button>
                </form>

                <p>
                    <b>{move || greet_msg.get()}</b>
                </p>
                <DynamicList initial_length=3/>
                <NestedSignals/>
                <MemorySlices/>
                <ControlledInputs/>
                <UncontrolledInputs/>
                <TextArea/>
                <SelectBar/>
                <MultipleReturnType/>
                <ShowComponent/>
                <ErrorHandling/>
                <ParentChildCom/>
                <TakesChildren render_prop=|| {
                    view! { <p>"Hello World"</p> }
                }>"Some text" <span>"A span"</span></TakesChildren>
                <WrapsChildren>"A1" "B1" "C1"</WrapsChildren>
                <WatchSignal/>
                <CreateEffect/>
                <CreateResource/>
                <SuspenseComponent/>
                <CreateAction/>
            </main>
        </Router>
    }
    // Counter(0,2)
}

#[component]
fn DynamicList(
    /// The number of counters to begin with.
    initial_length: usize,
) -> impl IntoView {
    // This dynamic list will use the <For/> component.
    // <For/> is a keyed list. This means that each row
    // has a defined key. If the key does not change, the row
    // will not be re-rendered. When the list changes, only
    // the minimum number of changes will be made to the DOM.

    // `next_counter_id` will let us generate unique IDs
    // we do this by simply incrementing the ID by one
    // each time we create a counter
    let mut next_counter_id = initial_length;

    // we generate an initial list as in <StaticList/>
    // but this time we include the ID along with the signal
    let initial_counters = (0..initial_length)
        .map(|id| (id, create_rw_signal(id + 1)))
        .collect::<Vec<_>>();

    // now we store that initial list in a signal
    // this way, we'll be able to modify the list over time,
    // adding and removing counters, and it will change reactively
    let counters = create_rw_signal(initial_counters);

    let add_counter = move |_| {
        // create a signal for the new counter
        let sig = create_rw_signal(next_counter_id + 1);
        // add this counter to the list of counters
        counters.update(move |counters| {
            // since `.update()` gives us `&mut T`
            // we can just use normal Vec methods like `push`
            counters.push((next_counter_id, sig))
        });
        // increment the ID so it's always unique
        next_counter_id += 1;
    };

    view! {
        <div>
            <button on:click=add_counter>"Add Counter"</button>
            <ul>
                // The <For/> component is central here
                // This allows for efficient, key list rendering
                <For
                    // `each` takes any function that returns an iterator
                    // this should usually be a signal or derived signal
                    // if it's not reactive, just render a Vec<_> instead of <For/>
                    each=move || counters.get()
                    // the key should be unique and stable for each row
                    // using an index is usually a bad idea, unless your list
                    // can only grow, because moving items around inside the list
                    // means their indices will change and they will all rerender
                    key=|counter| counter.0
                    // `children` receives each item from your `each` iterator
                    // and returns a view
                    children=move |(id, count)| {
                        view! {
                            <li>
                                <button on:click=move |_| {
                                    count.update(|n| *n += 1)
                                }>{count}</button>
                                <button on:click=move |_| {
                                    counters
                                        .update(|counters| {
                                            counters
                                                .retain(|(counter_id, signal)| {
                                                    if counter_id == &id {
                                                        signal.dispose();
                                                    }
                                                    counter_id != &id
                                                })
                                        });
                                }>

                                    "Remove"
                                </button>
                            </li>
                        }
                    }
                />

            </ul>
        </div>
    }
}

// nested signal
#[component]
pub fn NestedSignals() -> impl IntoView {
    let default_data = vec![
        DatabaseEntry {
            key: "foo".to_owned(),
            value: create_rw_signal(10),
        },
        DatabaseEntry {
            key: "bar".to_owned(),
            value: create_rw_signal(20),
        },
        DatabaseEntry {
            key: "baz".to_owned(),
            value: create_rw_signal(15),
        },
    ];

    let data = create_rw_signal(default_data);
    view! {
        <button on:click=move |_| {
            data.with(|data| {
                for row in data {
                    row.value.update(|value| *value *= 2);
                }
            });
            leptos_dom::log!("{:?}", data.get());
        }>"Update Values"</button>
        <For each=move || data.get() key=|state| state.key.to_owned() let:child>
            <p>{child.value}</p>
        </For>
    }
}

#[component]
pub fn MemorySlices() -> impl IntoView {
    let default_data = vec![
        DatabaseEntry2 {
            key: "foo".to_owned(),
            value: 10,
        },
        DatabaseEntry2 {
            key: "bar".to_owned(),
            value: 20,
        },
        DatabaseEntry2 {
            key: "baz".to_owned(),
            value: 15,
        },
    ];
    let data = create_rw_signal(default_data);
    view! {
        <button on:click=move |_| {
            data.update(|data| {
                for row in data {
                    row.value *= 3;
                }
            });
            leptos_dom::log!("{:?}", data.get());
        }>

            "Update MemorySlices"
        </button>

        <For
            each=move || data.get().into_iter().enumerate()
            key=|(_, state)| state.key.to_owned()
            children=move |(index, _)| {
                let value = create_memo(move |_| {
                    data.with(|data| data.get(index).map(|d| d.value).unwrap_or(0))
                });
                view! { <p>{value}</p> }
            }
        />
    }
}

#[component]
pub fn ControlledInputs() -> impl IntoView {
    let name = create_rw_signal("Controlled".to_owned());
    view! {
        <input
            type="text"
            on:input=move |ev| {
                name.update(|name| *name = event_target_value(&ev));
            }

            value=name
        />
        <p>"Name is: " {name}</p>
    }
}

#[component]
pub fn UncontrolledInputs() -> impl IntoView {
    let name = create_rw_signal("Uncontrolled".to_owned());
    let input_el: NodeRef<html::Input> = create_node_ref();
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let value = input_el.get().expect("<input> to exist").value();
        name.set(value);
    };
    view! {
        <form on:submit=on_submit>
            <input type="text" value=name node_ref=input_el/>
            <input type="submit" value="Submit"/>
        </form>
        <p>"Name is: " {name}</p>
    }
}

#[component]
fn TextArea() -> impl IntoView {
    let some_value = create_rw_signal("TextArea".to_owned());
    let input_el: NodeRef<html::Textarea> = create_node_ref();
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let value = input_el.get().expect("<input> to exist").value();
        some_value.set(value);
    };
    view! {
        <form on:submit=on_submit>
            <textarea
                type="text"
                prop:value=some_value
                on:input=move |ev| some_value.update(|v| *v = event_target_value(&ev))
                node_ref=input_el
            >// {untrack(move||some_value.get())}
            </textarea>
            <input type="submit" value="Submit"/>
        </form>
        <p>"Name is: " {some_value}</p>
    }
}

#[component]
fn SelectOption(is: &'static str, value: RwSignal<String>) -> impl IntoView {
    view! {
        <option value=is selected=move || value.get() == is>
            {is}
        </option>
    }
}

#[component]
fn SelectBar() -> impl IntoView {
    let value = create_rw_signal("B".to_owned());
    view! {
        <select on:change=move |ev| {
            value.update(|v| *v = event_target_value(&ev));
        }>
            <SelectOption value is="A"/>
            <SelectOption value is="B"/>
            <SelectOption value is="C"/>
        </select>
        <p>{move || value.get()}</p>
    }
}

#[component]
fn MultipleReturnType() -> impl IntoView {
    let value = create_rw_signal(2);
    let is_odd = move || value.get() & 1 == 1;
    view! {
        <div>
            {move || match is_odd() {
                true if value.get() == 1 => view! { <pre>"One"</pre> }.into_any(),
                false if value.get() == 2 => view! { <p>"Two"</p> }.into_any(),
                _ => view! { <textarea>{value.get()}</textarea> }.into_any(),
            }}

        </div>
    }
}

#[component]
fn ShowComponent() -> impl IntoView {
    let value = create_rw_signal(6);

    view! {
        <Show when=move || { value.get() > 5 } fallback=|| view! { <Small/> }>
            <Big/>
        </Show>
    }
}

#[component]
fn Big() -> impl IntoView {
    view! { "Big" }
}

#[component]
fn Small() -> impl IntoView {
    view! { "Small" }
}

#[component]
fn ErrorHandling() -> impl IntoView {
    let value = create_rw_signal(Ok(1));
    let on_input =
        move |ev: Event| value.update(move |val| *val = event_target_value(&ev).parse::<i32>());
    let on_fallback = |errors: RwSignal<Errors>| {
        view! {
            <div class="error">
                <p>"Not a number! Errors: "</p>
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(idex, e): (ErrorKey, Error)| {
                                leptos_dom::log!("{:?}", idex);
                                view! { <li>{e.to_string()}</li> }
                            })
                            .collect_view()
                    }}

                </ul>
            </div>
        }
    };
    view! {
        <h1>"Error Handling"</h1>
        <label>
            <input type="number" on:input=on_input/>
            <ErrorBoundary fallback=on_fallback>
                <p>"You entered: " <strong>{value}</strong></p>
            </ErrorBoundary>
        </label>
    }
}

#[component]
fn ParentChildCom() -> impl IntoView {
    let toggle = create_rw_signal(false);
    provide_context(toggle);
    view! {
        <p>"Toggled? " {toggle}</p>
        <ToggleButton on_click=move |_| toggle.update(|v| *v = !*v)/>
        <Layout/>
    }
}

#[component]
fn ToggleButton<F>(on_click: F) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    view! { <button on:click=on_click>"Toggle"</button> }
}

#[component]
pub fn Layout() -> impl IntoView {
    view! {
        <header>
            <h1>"My Page"</h1>
        </header>
        <main>
            <Content/>
        </main>
    }
}

#[component]
pub fn Content() -> impl IntoView {
    view! {
        <div class="content">
            <ButtonD/>
        </div>
    }
}

#[component]
pub fn ButtonD() -> impl IntoView {
    // use_context searches up the context tree, hoping to
    // find a `WriteSignal<bool>`
    // in this case, I .expect() because I know I provided it
    let setter = use_context::<RwSignal<bool>>().expect("to have found the setter provided");
    leptos_dom::log!("{:?}", setter);

    view! { <button on:click=move |_| setter.update(|value| *value = !*value)>"Toggle"</button> }
}

#[component]
pub fn TakesChildren<F, IV>(
    /// Takes a function (type F) that returns anything that can be
    /// converted into a View (type IV)
    render_prop: F,
    /// `children` takes the `Children` type
    children: Children,
) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! {
        <h2>"Render Prop"</h2>
        {render_prop()}

        <h2>"Children"</h2>
        {children()}
    }
}

#[component]
pub fn WrapsChildren(children: Children) -> impl IntoView {
    // Fragment has `nodes` field that contains a Vec<View>
    let children = children()
        .nodes
        .into_iter()
        .map(|child| {
            view! { <li>{child}</li> }
        })
        .collect_view();

    view! { <ul>{children}</ul> }
}

// fn Counter(initial_value: i32, step: u32) -> impl IntoView {
//     let count = create_rw_signal(0);
//     leptos_dom::log!("{} {}", initial_value, step);
//     div().child((
//         button().on(ev::click, move |_| count.set(0)).child("Clear"),
//         button().on(ev::click, move |_| count.update(|c| *c -= 1)).child("-1"),
//         span().child(("Value: ", move || count.get(), "!")),
//         button().on(ev::click, move |_| count.update(|c| *c += 1)).child("+1"),
//     ))
// }
//

#[component]
fn WatchSignal() -> impl IntoView {
    let num = create_rw_signal(0);
    provide_context(num);
    view! {
        <SetButton/>
        <StopButton/>
        <div>{num}</div>
    }
}

#[component]
fn StopButton() -> impl IntoView {
    let num = use_context::<RwSignal<i32>>().expect("to get num signal");
    let stop = watch(
        move || num.get(),
        move |num, prev_num, _| {
            log::debug!("Number: {:?}; Prev: {:?}", num, prev_num);
            leptos_dom::logging::console_log("hey");
        },
        false,
    );
    let stop_watching = move |_| {
        stop();
        leptos_dom::logging::console_log("hello");
    };
    view! { <button on:click=stop_watching>"Stop"</button> }
}

#[component]
fn SetButton() -> impl IntoView {
    let num = use_context::<RwSignal<i32>>().expect("to get num signal");
    let increment = move |_| num.update(|n| *n += 1);
    view! { <button on:click=increment>"Increment"</button> }
}

#[component]
fn CreateEffect() -> impl IntoView {
    let t_a = create_rw_signal(0);
    let t_b = create_rw_signal(0);
    create_effect(move |_| {
        leptos_dom::log!("value of a: {:?}", t_a.get());
    });
    view! {
        <div>"Value a:" {t_a}</div>
        <div>"Value b:" {t_b}</div>
        <button on:click=move |_| t_a.update(move |a| *a += 1)>"Increment a"</button>
        <button on:click=move |_| t_b.update(move |b| *b += 1)>"Increment b"</button>
    }
}

async fn load_data(value: i32) -> i32 {
    TimeoutFuture::new(1000).await;
    value * 10
}

#[component]
fn CreateResource() -> impl IntoView {
    let count = create_rw_signal(1);
    let async_data = create_resource(
        move || count.get(),
        |value: i32| async move { load_data(value).await },
    );
    let increase_count = move |_| count.update(move |count| *count += 1);
    view! {
        <h1>"My data"</h1>
        <button on:click=increase_count>"Increment Count"</button>
        {move || match async_data.get() {
            None => view! { <p>"Loading ..."</p> }.into_view(),
            Some(data) => view! { <p>{data}</p> }.into_view(),
        }}
    }
}

#[component]
fn SuspenseComponent() -> impl IntoView {
    let count_a = create_rw_signal(2);
    let count_b = create_rw_signal(5);
    let a = create_resource(
        move || count_a.get(),
        |count_a| async move { load_data(count_a).await },
    );
    let b = create_resource(
        move || count_b.get(),
        |count_b| async move { load_data(count_b).await },
    );
    let increase_AB = move |_| {
        count_a.update(move |c| *c += 1);
        count_b.update(move |c| *c += 2);
    };
    view! {
        <h1>"My Data"</h1>
        <button on:click=increase_AB>"Increment AB"</button>
        <Suspense fallback=move || view! { <p>"Loading..."</p> }>
            <h2>"My Data a: "</h2>
            {move || a.get()}
            <h2>"My Data b: "</h2>
            {move || { b.get() }}
        </Suspense>
    }
}

async fn add_todo(text: &str) -> uuid::Uuid {
    _ = text;
    TimeoutFuture::new(1000).await;
    uuid::Uuid::new_v4()
}

#[component]
fn CreateAction() -> impl IntoView {
    let action1 = create_action(|input: &String| {
        let input = input.to_owned();
        async move { add_todo(&input).await }
    });
    let submitted = action1.input();
    let pending = action1.pending();
    let todo_id = action1.value();

    let input_ref = create_node_ref::<html::Input>();

    view! {
        <form on:submit=move |ev| {
            ev.prevent_default();
            let input = input_ref.get().expect("input to exist");
            action1.dispatch(input.value());
        }>

            <label>"What do you need to do?" <input type="text" node_ref=input_ref/></label>
            <button type="submit">"Add Todo"</button>
        </form>
        <p>{move || pending.get().then(|| "Loading...")}</p>
        <p>"Submitted: " <code>{move || format!("{:#?}", submitted.get())}</code></p>
        <p>"Pending: " <code>{move || format!("{:#?}", pending.get())}</code></p>
        <p>"Todo ID: " <code>{move || format!("{:#?}", todo_id.get())}</code></p>
    }
}

#[component]
fn FormExample() -> impl IntoView {
    let query = use_query_map();
    let name = move || query.get().get("name").cloned().unwrap_or_default();
    let number = move || query.get().get("number").cloned().unwrap_or_default();
    let select = move || query.get().get("select").cloned().unwrap_or_default();
    view! {
        <table>
            <tr>
                <td>
                    <code>"name"</code>
                </td>
                <td>{name}</td>
            </tr>
            <tr>
                <td>
                    <code>"number"</code>
                </td>
                <td>{number}</td>
            </tr>
            <tr>
                <td>
                    <code>"select"</code>
                </td>
                <td>{select}</td>
            </tr>
        </table>
        <h2>"Manual Submission"</h2>
        <Form method="GET" action="">
            <input type="text" name="name" value=name/>
            <input type="number" name="number" value=number/>
            <select name="select">
                <option selected=move || select() == "A">"A"</option>
                <option selected=move || select() == "B">"B"</option>
                <option selected=move || select() == "C">"C"</option>
            </select>

            <input type="submit"/>
        </Form>
        <h2>"Automatic Submission"</h2>
        <Form method="GET" action="">
            <input
                type="text"
                name="name"
                value=name
                // this oninput attribute will cause the
                // form to submit on every input to the field
                oninput="this.form.requestSubmit()"
            />
            <input type="number" name="number" value=number oninput="this.form.requestSubmit()"/>
            <select name="select" onchange="this.form.requestSubmit()">
                <option selected=move || select() == "A">"A"</option>
                <option selected=move || select() == "B">"B"</option>
                <option selected=move || select() == "C">"C"</option>
            </select>
            // submitting should cause a client-side
            // navigation, not a full reload
            <input type="submit"/>
        </Form>
    }
}
