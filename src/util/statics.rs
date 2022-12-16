use {
  crate::util::{conf_structs::Config, config::read_config},
  once_cell::sync::Lazy,
};

pub(crate) static CONF: Lazy<Config> = Lazy::new(read_config);

pub(crate) static GREET_ICONS: [&str; 4] = ["", "", "", ""];
pub(crate) static GREET_EMOJIS: [&str; 4] = ["🌇", "🏙️", "🌆", "🌃"];
pub(crate) static TIME_ICONS: [&str; 12] =
  ["", "", "", "", "", "", "", "", "", "", "", ""];
pub(crate) static TIME_EMOJIS: [&str; 12] = [
  "🕛", "🕐", "🕑", "🕒", "🕓", "🕔", "🕕", "🕖", "🕗", "🕘", "🕙", "🕚",
];
pub(crate) static WEATHER_ICONS: [&str; 21] = [
  "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
  "", "",
];
pub(crate) static WEATHER_EMOJIS: [&str; 21] = [
  "☀️", "⛅️", "🌙", "☁️", "🌙", "☁️", "☁️", "☁️", "🌧️", "🌧️", "🌧️", "🌧️", "⛈️", "⛈️", "🌨️", "🌨️", "🌫️", "🌫️", "🌫️",
  "🌫️", "❓",
];
pub(crate) static PACKAGE_ICONS: [&str; 12] =
  ["", "", "", "", "", "", "", "", "", "", "", ""];
pub(crate) static PACKAGE_EMOJIS: [&str; 13] = [
  "☑️", "1️⃣", "2️⃣", "3️⃣", "4️⃣", "5️⃣", "6️⃣", "7️⃣", "8️⃣", "9️⃣", "🔟", "‼️", "📦",
];
pub(crate) static MISC_ICONS: [&str; 6] = ["", "", "", "", "", ""];
pub(crate) static MISC_EMOJIS: [&str; 6] = ["💻", "🫀", "🧠", "💾", "🖥️", "🎵"];
