use wallet_provider::NostradeWalletStore;
use yew::prelude::*;
mod pages;
pub mod persister;
mod router;
mod wallet_provider;
pub use pages::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    let relays = vec![
        nostr_minions::relay_pool::UserRelay {
            url: "wss://relay.nos.lol".to_string(),
            read: true,
            write: true,
        },
        nostr_minions::relay_pool::UserRelay {
            url: "wss://relay.illuminodes.com".to_string(),
            read: true,
            write: true,
        },
    ];
    html! {
        <yew_router::BrowserRouter>
           <nostr_minions::relay_pool::NostrRelayPoolProvider {relays}>
           <nostr_minions::key_manager::NostrIdProvider>
               <div class="fixed inset-0 z-0 bg-gradient-to-br from-purple-600 via-pink-500 to-orange-400"/>
               <div class="fixed inset-0 font-impact tracking-widest relative z-20">
               <Suspense fallback={html! { <GyatSplash /> }}>
                   <wallet_provider::WalletProvider>
                   <LoginCheck>
                   <WalletLoad>
                         <crate::router::MainPageSwitch />
                   </WalletLoad >
                   </LoginCheck>
                   </wallet_provider::WalletProvider>
               </Suspense>
               </div>
               <img src="./public/blueberrini.png" alt="GYAT Logo" class="fixed -top-20 left-0 size-140 opacity-20 z-10" />
               <img src="./public/borrito.png" alt="GYAT Logo" class="fixed -bottom-20 left-0 size-140 opacity-20 z-10" />
               <img src="./public/avocodo.png" alt="GYAT Logo" class="fixed -bottom-20 right-0 object-cover size-140 opacity-20 z-10" />
               <img src="./public/tungtung.png" alt="GYAT Logo" class="fixed -top-20 right-0 object-cover size-140 opacity-20 z-10" />
           </nostr_minions::key_manager::NostrIdProvider>
           </nostr_minions::relay_pool::NostrRelayPoolProvider>
        </yew_router::BrowserRouter>
    }
}

#[function_component(LoginCheck)]
fn login_check(props: &yew::html::ChildrenProps) -> HtmlResult {
    let ctx = nostr_minions::key_manager::use_nostr_id_ctx();
    let has_key = yew::suspense::use_future_with(ctx.clone(), move |ctx| async move {
        ctx.get_nostr_key().await.is_some()
    })?;
    if *has_key {
        Ok(props.children.clone())
    } else {
        Ok(html! {
            <div class="flex flex-col items-center justify-evenly h-screen w-screen p-4">
                <pages::NostrLogin />
            </div>
        })
    }
}

#[function_component(WalletLoad)]
fn wallet_load(props: &yew::html::ChildrenProps) -> HtmlResult {
    let wallet_ctx = use_context::<NostradeWalletStore>().expect("No wallet context found");
    let key_ctx = nostr_minions::key_manager::use_nostr_key();
    let ctx_clone = wallet_ctx.clone();
    let loaded = yew::suspense::use_future_with(key_ctx, |nostr_key| async move {
        if let Some(mut key) = (*nostr_key).clone() {
            key.set_extractable(true);
            let Ok(mnemonic) = key.mnemonic(nostr_minions::nostro2_signer::Language::English)
            else {
                return false;
            };
            key.set_extractable(false);
            let Some(bdk_key) = bdk_wallet::bip39::Mnemonic::parse(&mnemonic).ok() else {
                return false;
            };
            if !ctx_clone.loaded() {
                ctx_clone.load(&bdk_key.to_seed("")).await.ok();
                ctx_clone.sync().await.ok();
                ctx_clone.dispatch(wallet_provider::NostradeWalletAction::Loaded);
                return true;
            }
            ctx_clone.loaded()
        } else {
            true
        }
    })?;
    match *loaded {
        true => Ok(props.children.clone()),
        false => Ok(html! {
            <GyatSplash />
        }),
    }
}

#[function_component(GyatSplash)]
pub fn splash_screen() -> Html {
    html! {
            <div class="h-screen flex flex-col items-center justify-center text-center space-y-4">
                <img src="./public/pool.png" alt="GYAT Logo" class="w-32 h-32 mx-auto mb-4 animate-bounce" />
                <GyatWalletHeader />
                <GyatWalletFooter />

            </div>
    }
}

#[function_component(GyatWalletHeader)]
pub fn gyat_wallet_header() -> Html {
    html! {
        <header class="text-center space-y-2 pt-4">
          <div class="flex items-center justify-center gap-2">
            <lucide_yew::Bitcoin class="w-6 h-6 text-yellow-300 animate-bounce" />
            <h1 class="text-2xl font-black text-white">{"GYAT WALLET"}</h1>
            <lucide_yew::Bitcoin class="w-6 h-6 text-yellow-300 animate-bounce" />
          </div>
          <p class="text-white/80 text-sm font-medium">{"💎 DIAMOND HANDS ONLY 💎"}</p>
        </header>
    }
}

#[function_component(GyatWalletFooter)]
pub fn gyat_wallet_header() -> Html {
    html! {
        <div class="text-center text-white/80 text-sm font-bold pb-2 my-6">
            <p>
                <img src="./public/emojifire.png" alt="gyat" class="inline-block w-4 h-4 mr-1" />
                    <span>
                        {"STAY SIGMA, KEEP HODLING"}
                    </span>
                <img src="./public/emojifire.png" alt="gyat" class="inline-block w-4 h-4 ml-1" />
            </p>
            <p class="text-xs mt-1">
                <img src="./public/emoji100.png" alt="gyat" class="inline-block w-4 h-4 mr-1" />
                <span>
                    {"NOT FINANCIAL ADVICE FR FR"}
                </span>
                <img src="./public/emoji100.png" alt="gyat" class="inline-block w-4 h-4 ml-1" />
            </p>
        </div>
    }
}
