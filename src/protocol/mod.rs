//! The data structures used by the XML protocol.

mod data;
mod game_result;
mod joined;
mod left;
mod player_score;
mod room;
mod score_aggregation;
mod score_cause;
mod score_definition;
mod score_fragment;

pub use data::*;
pub use game_result::*;
pub use joined::*;
pub use left::*;
pub use data::*;
pub use player_score::*;
pub use room::*;
pub use score_definition::*;
pub use score_fragment::*;
pub use score_aggregation::*;
pub use score_cause::*;
