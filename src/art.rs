use crate::{Frame, Result};

pub const NAMES: &[&str] = &[
    "ghost", "cat", "rocket", "mountain", "tree", "owl", "heart", "coffee", "fish", "robot",
    "castle", "flower",
];

/// Original small templates; Ghostty artwork is not copied.
pub fn template(name: &str) -> Result<Frame> {
    let text = match name {
        "ghost" => {
            r"       .------.
     .'        '.
    /   o    o   \
   |      __      |
   |              |
   |              |
   /_/\__/\__/\__/\"
        }
        "cat" => {
            r"     /\   /\
    (  . .  )
     \  ^  /
    /|     |\
   (_|_____|_)~~"
        }
        "rocket" => {
            r"       /\
      /  \
     | () |
     |    |
    /|    |\
   /_|____|_\
      /\/\
       \/ "
        }
        "mountain" => {
            r"              /\
       /\    /  \      /\
      /  \  / /\ \    /  \
  ___/____\/_/__\_\__/____\___
 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"
        }
        "tree" => {
            r"        /\
       /**\
      /****\
     /******\
    /********\
       |  |
    ___|__|___"
        }
        "owl" => {
            r"     ,___,
    / o o \
   (   V   )
   /|     |\
  (_|_____|_)
     / | \"
        }
        "heart" => {
            r"    __    __
  /    \/    \
 |            |
  \          /
   \        /
    \      /
     \    /
      \  /
       \/"
        }
        "coffee" => {
            r"      (  )
       )(
    .------.
    |      |__
    |      |  )
    |______|_/
   ==========="
        }
        "fish" => {
            r"       .------.
  ><  /  o     \
   \\/       __/
   /\  _____/
  /  \/ "
        }
        "robot" => {
            r"       _|_
    .-------.
    | [] [] |
    |  ===  |
  __|_______|__
 /  |       |  \
    |_______|
     |     |
    _|     |_"
        }
        "castle" => {
            r"   _   _       _   _
  | |_| |     | |_| |
  |     |_____|     |
  |  []         []  |
  |       ___       |
  |      /   \      |
 _|______|___|______|_"
        }
        "flower" => {
            r"      .-.
   .-(   )-.
  (   \ /   )
   '-.( ).-'
      /|\
       |
   \   |   /
    \__|__/
       |"
        }
        _ => {
            return Err(format!(
                "Unknown artwork {name}; choose {}",
                NAMES.join(", ")
            ));
        }
    };
    Frame::from_text_padded(text)
}
