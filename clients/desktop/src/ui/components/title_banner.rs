use crate::TextKeys::*;
use dioxus::prelude::*;
use common::ports::i18n_driven_port::I18nDrivenPort;

const KDRIVE_LOGO: Asset = asset!("/assets/kdrive.svg");

#[component]
pub fn TitleBanner<I18nPort: I18nDrivenPort + 'static>(i18n: I18nPort) -> Element {
    rsx! (
        div {
            class: "-ml-20 flex items-center gap-x-6",
            img { src: KDRIVE_LOGO, class: "w-16 h-16 object-contain", alt: i18n.t(KDriveLogoAlt)}
            h1 { class: "text-5xl font-bold tracking-tighter", {i18n.t(KDriveProductName)} }
        }
    )
}