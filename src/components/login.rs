use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
        <div class="bg-indigo-900 flex w-screen h-screen justify-center items-center font-mono">
            <div class="flex flex-col justify-center items-center bg-black p-10 border-4 border-pink-500 shadow-[8px_8px_0px_rgba(236,72,153,1)]">
                <div class="text-6xl mb-4">{"👾"}</div>
                <h1 class="text-4xl font-extrabold text-transparent bg-clip-text bg-gradient-to-r from-pink-500 to-yellow-500 mb-2 tracking-widest">
                    {"ARCADE LOBBY"}
                </h1>
                <p class="text-green-400 mb-6 animate-pulse">{"INSERT COIN (Enter Player Name)"}</p>
                <form class="flex w-full mt-4">
                    <input {oninput} class="flex-grow p-4 border-2 border-green-500 bg-gray-900 text-green-400 focus:outline-none focus:ring-2 focus:ring-pink-500 placeholder-gray-600" placeholder="Player..."/>
                    <Link<Route> to={Route::Chat}> 
                        <button {onclick} disabled={username.len()<1} class="px-6 bg-pink-600 text-white font-bold p-4 uppercase hover:bg-pink-500 transition disabled:opacity-50 border-y-2 border-r-2 border-pink-500" >
                            {"START 🎮"}
                        </button>
                    </Link<Route>>
                </form>
            </div>
        </div>
    }
}
