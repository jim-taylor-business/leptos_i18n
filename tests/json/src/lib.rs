#![deny(warnings)]

leptos_i18n::load_locales!();

#[cfg(test)]
mod defaulted;
#[cfg(test)]
mod foreign;
#[cfg(test)]
mod plurals;
#[cfg(test)]
mod scoped;
#[cfg(test)]
mod subkeys;
#[cfg(test)]
mod tests;
