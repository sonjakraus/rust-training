mod sub_sub_mod;

pub fn sub_mod_func() {
    hidden_inline_mod::hidden_sub_fn();
}

mod hidden_inline_mod {
    pub fn  hidden_sub_fn() {}
}