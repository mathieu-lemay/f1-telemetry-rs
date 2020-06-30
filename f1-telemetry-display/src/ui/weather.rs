use crate::models::WeatherInfo;
use crate::ui::fmt;
use f1_telemetry::packet::session::Weather;
use ncurses::{mvwaddstr, WINDOW};
use std::time::{SystemTime, UNIX_EPOCH};

const OVERCAST: &str = "
               _           
           .:(`  )`.
     .--  `.  (    ) )    .--
  .+(   )   ` _`  ) )  .+(   )
  (   .  ) `_` (   )   (   .  )
 (   (   ))     `-'.  (   (   ))
  `- __.' __.'__.'__.' __.' __-`
";

const LIGHT_RAIN_1: &str = "
 (   (   ))     `-'.  (   (   ))
  `- __.' __.'__.'__.' __.' __-`
  \\        \\         \\        \\
       \\         \\        \\\t\t
";
const LIGHT_RAIN_2: &str = "
 (   (   ))     `-'.  (   (   ))
  `- __.' __.'__.'__.' __.' __-`
       \\         \\        \\\t\t
  \\        \\         \\        \\
";

const HEAVY_RAIN_1: &str = "
 (   (   ))     `-'.  (   (   ))
  `- __.' __.'__.'__.' __.' __-`
   \\ \\ \\ \\ \\ \\ \\ \\ \\ \\ \\ \\ \\ \\ \\
  \\ \\ \\ \\ \\ \\ \\ \\ \\ \\ \\ \\ \\ \\ \\\t\t
";

const HEAVY_RAIN_2: &str = "
 (   (   ))     `-'.  (   (   ))
  `- __.' __.'__.'__.' __.' __-`
  \\ \\ \\ \\ \\ \\ \\ \\ \\ \\ \\ \\ \\ \\ \\\t\t
   \\ \\ \\ \\ \\ \\ \\ \\ \\ \\ \\ \\ \\ \\ \\
";

const CLEAR_1: &str = "
           |
       \\   |   /
         ,d8b,
     --- 88888 ---
          98P'
       /   |   \\
           |
";

const CLEAR_2: &str = "
           |
           |\t\t
         ,d8b,
     --- 88888 ---
          98P'
           |\t\t
           |
";

const LIGHT_CLOUD: &str = "
           |
       \\   |   /
         ,d8b,           .,
 (')-')_ 88888 ---   ;';'  ';'.
('-  (. ')98p'      ';.,;    ,;
 '-.(/' )'     \\       '.';.'
           |
";

const STORM: &str = "
 (   (   ))     `-'.  (   (   ))
  `- __.' __.'__.'__.' __.' __-`
   _/ /       _/ /       _/ /
  /__/       /__/       /__/
 //         //         //
/          /          /
";

pub fn render_weather(w: WINDOW, weather_info: &WeatherInfo, y: i32, x: i32) {
    let weather_icon: &str = match weather_info.weather {
        Weather::Clear => {
            if blink() {
                CLEAR_1
            } else {
                CLEAR_2
            }
        }
        Weather::Overcast => OVERCAST,
        Weather::LightCloud => LIGHT_CLOUD,
        Weather::LightRain => {
            if blink() {
                LIGHT_RAIN_1
            } else {
                LIGHT_RAIN_2
            }
        }
        Weather::HeavyRain => {
            if blink() {
                HEAVY_RAIN_1
            } else {
                HEAVY_RAIN_2
            }
        }
        Weather::Storm => STORM,
    };

    for (i, l) in weather_icon.split('\n').enumerate() {
        mvwaddstr(w, y + i as i32, x, l);
    }
}

fn blink() -> bool {
    (SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        % 2)
        == 0
}
