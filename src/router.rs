use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum GyatRoute {
    #[at("/")]
    Home,
    #[at("/address")]
    Receive,
    #[at("/send")]
    SendCoins,
    #[at("/key")]
    Backup,
}

#[function_component(MainPageSwitch)]
pub fn main_page_switch() -> HtmlResult {
    let base_class = classes!(
        "transition-transform",
        "duration-500",
        "ease-in-out",
        "flex",
        "overflow-hidden",
        "h-screen",
        "z-20",
        "max-h-screen",
    );

    Ok(html! {
            <Switch<GyatRoute> render = { move |switch: GyatRoute| {
                html! {
                    <>
                    <div class={if switch == GyatRoute::Home {
                        classes!(base_class.clone(), "translate-y-0")
                    } else {
                        classes!(base_class.clone(), "-translate-y-full" )
                    }}>
                        <HomePage />
                    </div>
                    <div class={if switch == GyatRoute::Receive {
                        classes!(base_class.clone(), "translate-y-0", "absolute", "inset-0")
                    } else {
                        classes!(base_class.clone(), "-translate-y-full", "absolute", "inset-0")
                    }}>
                        <AddressDisplay />
                    </div>
                    <div class={if switch == GyatRoute::SendCoins {
                        classes!(base_class.clone(), "translate-y-0", "absolute", "inset-0")
                    } else {
                        classes!(base_class.clone(), "-translate-y-full", "absolute", "inset-0")
                    }}>
                        <SendCoins />
                    </div>
                    <div class={if switch == GyatRoute::Backup {
                        classes!(base_class.clone(), "translate-y-0", "absolute", "inset-0")
                    } else {
                        classes!(base_class.clone(), "-translate-y-full", "absolute", "inset-0")
                    }}>
                        <NewGyatKey />
                    </div>
                    </>
                }
            } } />
    })
}

#[function_component(HomePage)]
pub fn main() -> HtmlResult {
    Ok(html! {
        <div class="flex-1 justify-evenly items-center flex flex-col">
            <BalanceSummary />
            <GyatActionButtons />
            <ListTransactions />
        </div>
    })
}
#[function_component(GyatActionButtons)]
pub fn gyat_action_buttons() -> Html {
    html! {
        <div class="grid grid-cols-2 gap-4 max-w-xs md:max-w-sm w-full items-stretch justify-items-stretch">
            <yew_router::components::Link<GyatRoute> to={GyatRoute::Receive}>
                <button class="flex gap-1 h-fit w-full px-1 items-center justify-center rounded-xl bg-gradient-to-r from-green-400 to-blue-500 hover:from-green-500 hover:to-blue-600 text-white font-black py-3 border-4 border-white shadow-xl transform hover:scale-105 transition-all">
                        {"SECURE A 💰"}
                </button>
            </yew_router::components::Link<GyatRoute> >
            <yew_router::components::Link<GyatRoute> to={GyatRoute::SendCoins}>
                <button class="flex gap-1 h-fit w-full items-center justify-center px-1 rounded-xl bg-gradient-to-r from-red-400 to-pink-500 hover:from-red-500 hover:to-pink-600 text-white font-black py-3 border-4 border-white shadow-xl transform hover:scale-105 transition-all">
                    <span>
                        {"YEET SATS"}
                        <lucide_yew::Rocket class="w-4 h-4 inline-block ml-1" />
                    </span>
                </button>
            </yew_router::components::Link<GyatRoute> >
        </div>
    }
}

#[function_component(NewGyatKey)]
pub fn new_gyat_key() -> Html {
    let onsubmit = {
        Callback::from(|e: SubmitEvent| {
            e.prevent_default();
            let form = e.target_unchecked_into::<web_sys::HtmlFormElement>();
            let seed = form
                .get_with_name("seed")
                .map(|n| {
                    wasm_bindgen::JsCast::unchecked_into::<web_sys::HtmlInputElement>(n).value()
                })
                .unwrap_or_default();

            yew::platform::spawn_local(async move {});
        })
    };
    html! {
        <form {onsubmit}>
            <input
                type="text"
                id="seed"
                name="seed"
                placeholder="Mnemonic" />
            <input
                type="submit"
                value="Load Wallet"
                class="bg-black text-white" />
        </form>
    }
}

#[function_component(AddressDisplay)]
pub fn get_address() -> HtmlResult {
    let copied = use_state(|| false);
    let Some(address) = crate::wallet_provider::use_wallet_btc_address() else {
        return Ok(html! {
            <Spinner />
        });
    };
    let onclick = {
        let copied = copied.setter();
        let addr = address.clone();
        Callback::from(move |_| {
            let address = addr.clone();
            let Some(clipboard) = web_sys::window().map(|win| win.navigator().clipboard()) else {
                web_sys::console::log_1(&"Clipboard API not available".into());
                return;
            };
            let future = clipboard.write_text(&address.to_string());
            let copied = copied.clone();
            yew::platform::spawn_local(async move {
                let promise = wasm_bindgen_futures::JsFuture::from(future);
                if let Err(e) = promise.await {
                    web_sys::console::log_1(&format!("Failed to copy address: {e:?}").into());
                } else {
                    copied.set(true);
                    web_sys::console::log_1(&format!("Copied address: {address}").into());
                    // Reset copied state after 2 seconds
                    gloo::timers::future::sleep(std::time::Duration::from_secs(2)).await;
                    copied.set(false);
                }
            });
        })
    };
    Ok(html! {
        <div class="flex-1 justify-evenly items-center flex flex-col">
          <div class="max-w-xs md:max-w-sm mx-auto space-y-4 sm:space-y-6">
            <div class="flex items-center justify-between pt-4 sm:pt-8">
                <yew_router::components::Link<GyatRoute> to={GyatRoute::Home}>
                <button class="text-white hover:bg-white/20">
                    <lucide_yew::ArrowLeft class="w-6 h-6" />
                </button>
                </yew_router::components::Link<GyatRoute> >
              <div class="text-center">
                <h1 class="text-2xl font-black text-white sm:text-3xl">{"RECEIVE COINS 💰"}</h1>
                <p class="text-white/80 text-sm font-medium">{"GET THAT BAG FR FR 💸"}</p>
              </div>
              <div class="w-10"></div>
            </div>

            <div class="bg-gradient-to-br from-cyan-400 via-purple-500 to-pink-500 border-4 border-white shadow-2xl rounded-2xl p-3 space-y-2">
              <div>
                <div class="text-white font-black text-center flex items-center justify-center gap-2">
                  <lucide_yew::Sparkles class="w-5 h-5" />
                  {"TAKE A (QR)SELFIE 📱"}
                  <lucide_yew::Sparkles class="w-5 h-5" />
                </div>
              </div>
              <div class="flex justify-center">
                <div class="w-64 h-64 border-pink-300  rounded-2xl border-4 bg-white shadow-xl flex items-center justify-center relative overflow-hidden">
                    <bitcoin-qr
                        id="qr"
                        width="240"
                        height="240"
                        bitcoin={ address.to_qr_uri() }
                        //parameters="amount=0.00001&label=sbddesign%3A%20For%20lunch%20Tuesday&message=For%20lunch%20Tuesday"
                        image="./public/blueberrini.png"
                        type="svg"
                        corners-square-color="#f7931a"
                        corners-dot-color="#f7931a"
                        corners-square-type="extra-rounded"
                        dots-type="classy-rounded"
                        dots-color="#f7931a"
                    />
                </div>
              </div>
            </div>

            <div class="bg-gradient-to-br from-cyan-400 via-purple-500 to-pink-500 border-4 border-white shadow-2xl rounded-2xl p-3 space-y-2">
              <div>
                <div class="text-white font-black text-center">{"YOUR SIGMA ADDRESS 🏠"}</div>
              </div>
              <div class="space-y-4">
                <div class="bg-gradient-to-r from-gray-100 to-gray-500 p-4 rounded-xl border-2 border-gray-300">
                  <p class="text-sm font-mono text-gray-800 truncate mas-w-32 text-center">{address.to_string()}</p>
                </div>
                <button
                    {onclick}
                    class={classes!("w-full", "font-black", "py-4", "text-lg", "border-4", "flex", "items-center",
                        "justify-center", "rounded-xl", "text-white", "hover:shadow-2xl",
                        "border-white", "shadow-xl", "transform", "hover:scale-105", "transition-all",
                        if *copied {
                            "bg-gradient-to-r from-green-500 to-emerald-500 hover:from-green-600 hover:to-emerald-600"
                        } else {
                            "bg-gradient-to-r from-yellow-400 to-orange-500 hover:from-blue-600 hover:to-purple-600"
                        }
                      ) }>
                        {if *copied {
                            html! {
                                <>
                                    <lucide_yew::Check class="w-6 h-6 mr-2" />
                                    {"COPIED! BASED 🔥"}
                                </>
                            }
                        } else {
                            html! {
                                <>
                                    <lucide_yew::Copy class="w-6 h-6 mr-2" />
                                    {"COPY ADDRESS 📋"}
                                </>
                            }
                        }}
                </button>
              </div>
            </div>
            </div>
        </div>
    })
}

#[function_component(BalanceSummary)]
pub fn list_transactions() -> HtmlResult {
    let visible = use_state(|| false);
    let Some(balance) = crate::wallet_provider::use_wallet_btc_balance() else {
        return Ok(html! {
            <Spinner />
        });
    };

    let onclick = {
        let visible = visible.clone();
        Callback::from(move |_| {
            visible.set(!*visible);
        })
    };

    Ok(html! {
        <div class="bg-gradient-to-r from-yellow-400 to-orange-500 border-4 border-white shadow-2xl rounded-xl py-9 px-6 max-w-xs md:max-w-sm w-full">
           <div {onclick} class="flex gap-6 items-center justify-evenly">
                <img src="./public/saturno.png" alt="gyat" class="inline-block size-6 animate-bounce" />
                {if *visible {
                    html! {
                        <div class="text-center flex items-center flex-col">
                            <div class="text-xl font-black text-white mb-2 text-nowrap flex gap-1 items-center">
                                <img src="./public/sats-white.png" alt="gyat" class="inline-block size-5 mr-2" />
                                {format!("{}", balance.total().to_sat())}
                            </div>
                        </div>
                    }
                } else {
                    html! {
                        <div class="text-white font-black text-lg">
                        <p class="text-center mb-2">
                            {"SATURNO SATURNINA SATS"}
                        </p>
                        <p class="text-center mb-2 text-xs">
                            {"CLICK TO REVEAL"}
                        </p>
                        </div>

                    }
                }}
                <img src="./public/saturno.png" alt="gyat" class="inline-block size-6 animate-bounce" />
           </div>
        </div>
    })
}

#[function_component(ListTransactions)]
pub fn list_transactions() -> HtmlResult {
    let transactions = crate::wallet_provider::use_wallet_transactions();
    let inner_html = html! {
        <>
            { for transactions.into_iter().map(|(tx, conf_time)| html! {
                <a href={format!("https://mempool.space/signet/tx/{}", tx.compute_txid())}
                           target="_blank"
                           rel="noopener noreferrer">

                <div
                  key={tx.compute_txid().to_string()}
                  class="flex items-center justify-between p-2 gap-3 bg-gradient-to-r from-purple-100 to-pink-100 rounded-xl border-2 border-purple-200"
                >
                    <div class="text-sm text-gray-600 font-medium max-w-32 truncate">{tx.compute_txid().to_string()}</div>
                    <div class="text-right">
                        {if conf_time.is_confirmed() {
                             html! { "✅ BASED" }
                        } else {
                            html! { "⏳ SUSSY" }
                        }}
                    </div>
                </div>
                </a>
            }) }
        </>
    };
    Ok(html! {
        <div class="bg-gradient-to-r from-yellow-400 to-orange-500 border-4 border-white shadow-2xl rounded-xl max-w-xs md:max-w-sm w-full">
            <div class="py-3 px-6">
                <div class="text-white font-black flex items-center justify-center gap-2">
                    <img src="./public/emojifire.png" alt="gyat" class="inline-block size-6 mr-1 animate-pulse" />
                        {"RIZZLER MOVES"}
                    <img src="./public/emojifire.png" alt="gyat" class="inline-block size-6 ml-1 animate-pulse" />
                </div>
            </div>
            <div class="flex flex-col gap-3 px-6 overflow-y-auto max-h-54 flex-1 min-h-54">
                { inner_html }
            </div>
        </div>
    })
}

#[function_component(Spinner)]
pub fn spinner() -> Html {
    html! {
        <div class="rounded-full w-6 h-6 border-4 border-t-4 border-blue-500 transition-all duration-500 ease-in-out animate-spin" />
    }
}

#[function_component(SendCoins)]
pub fn send_coins() -> Html {
    let wallet_ctx = use_context::<crate::wallet_provider::NostradeWalletStore>()
        .expect("Wallet context not found");
    let address = use_state(|| None::<bitcoin::Address>);
    let sent_tx = use_state(|| None::<bitcoin::Txid>);
    let address_checked = classes!(
        "absolute",
        "inset-y-2",
        "right-2",
        "flex",
        "items-center",
        "justify-center",
        "w-8",
        "h-8",
        "transition-opacity",
        "duration-300",
        if address.is_some() {
            "text-green-300"
        } else {
            "text-red-300"
        }
    );
    let address_clone = address.clone();
    let tx_clone = sent_tx.clone();
    let onsubmit = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();
        let form = e.target_unchecked_into::<web_sys::HtmlFormElement>();
        let value = form
            .get_with_name("asset_amount")
            .map(|n| wasm_bindgen::JsCast::unchecked_into::<web_sys::HtmlInputElement>(n).value())
            .unwrap_or_default()
            .parse::<u64>()
            .unwrap_or(0);

        let Some(address) = address_clone.as_ref().cloned() else {
            web_sys::console::log_1(&"No valid address provided".into());
            return;
        };
        let amount = bitcoin::Amount::from_sat(value);
        let ctx_clone = wallet_ctx.clone();
        let tx_clone = tx_clone.clone();
        yew::platform::spawn_local(async move {
            match ctx_clone.send_coins(address, amount).await {
                Ok(txid) => {
                    tx_clone.set(Some(txid.compute_txid()));
                    // Optionally, navigate to a confirmation page or show a success message
                }
                Err(e) => {
                    web_sys::console::error_1(&format!("Failed to send coins: {e:#?}",).into());
                    // Optionally, show an error message to the user
                }
            }
        });
    });
    let can_send = address.is_some();
    html! {
        <div class="flex-1 justify-evenly items-center flex flex-col">
          <div class="max-w-xs md:max-w-sm mx-auto space-y-4 sm:space-y-6">
            <div class="flex items-center justify-between pt-4 sm:pt-8">
                <yew_router::components::Link<GyatRoute> to={GyatRoute::Home}>
                <button class="text-white hover:bg-white/20">
                    <lucide_yew::ArrowLeft class="w-6 h-6" />
                </button>
                </yew_router::components::Link<GyatRoute> >
              <div class="text-center">
                <h1 class="text-2xl font-black text-white sm:text-3xl">{"SEND COINS 💰"}</h1>
                <p class="text-white/80 text-sm font-medium">{"BURN THAT CASH 💸"}</p>
              </div>
              <div class="w-10"></div>
            </div>
            <div class="bg-gradient-to-br from-cyan-400 via-purple-500 to-pink-500 border-4 border-white shadow-2xl rounded-2xl p-3 space-y-2">
                <form {onsubmit}
                    class="space-y-4">
                    <img
                        src="./public/tungtung.png"
                        class="w-16 h-16 mx-auto mb-4"
                        alt="Asset Icon" />
                    <h2 class="text-center font-semibold mb-2 text-white">{"Yeet those coins fam"}</h2>
                    <div class="relative w-full min-h-12">
                        <input
                            type="text"
                            name="recipient_address"
                            placeholder="Recipient Address"
                            onchange={
                                let address = address.clone();
                                Callback::from(move |e: Event| {
                                    let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                    address.set(input.value().parse::<bitcoin::Address<bitcoin::address::NetworkUnchecked>>()
                                        .ok()
                                        .and_then(|addr| addr.require_network(bitcoin::Network::Signet).ok()));
                                })
                            }
                            class="absolute inset-0 p-2 border border-gray-200 rounded-lg truncate"
                            required=true />
                        {match *address {
                            Some(_) => html!(
                                <lucide_yew::Check class={address_checked} />
                            ),
                            None => html!(
                                <lucide_yew::X class={address_checked} />
                            ),
                        }}
                    </div>
                    <input
                        type="number"
                        name="asset_amount"
                        placeholder="Enter Amount"
                        class="w-full p-2 border border-gray-200 rounded-lg"
                        required=true
                        />
                    <input
                        type="submit"
                        value="Send"
                        disabled={!can_send}
                        class={classes!("w-full", "p-2", "bg-gradient-to-r", "from-orange-300", "to-red-500", "border-4", "border-white", "text-white", "rounded-lg", "cursor-pointer", "hover:bg-blue-600", "mt-16",
                            if can_send {
                                "opacity-100"
                            } else {
                                "opacity-50 cursor-not-allowed"
                            }
                            )}
                        />
                </form>
                {if let Some(tx_id) = sent_tx.as_ref() {
                    html! {
                    <a href={format!("https://mempool.space/signet/tx/{}", tx_id)}
                           target="_blank"
                           rel="noopener noreferrer">
                        <div class="mt-4 p-3 bg-gradient-to-r from-green-400 to-slate-200 text-green-800 rounded-lg">
                            <p class="font-semibold">{"Transaction Sent!"}</p>
                            <p class="text-sm">{"Transaction ID: "}</p>
                            <p class="font-mono max-w-32 truncate">{tx_id.to_string()}</p>
                            <p class="font-xs">
                            {"View Transaction on Mempool"}
                           </p>
                        </div>
                    </a>
                    }
                } else {
                    html! {}
                }}
        </div>
        </div>
        </div>
    }
}
