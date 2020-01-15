use rustler::{SchedulerFlags::*, Term};

mod atoms;
mod enchant_elixir;

rustler::rustler_export_nifs! {
    "Elixir.Enchant",
    [
        ("check", 2, enchant_elixir::check, DirtyIo)
    ],
    Some(enchant_elixir::load)
}
