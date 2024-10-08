// This module contains the types to represent commands from the chess engine to
// a GUI.

use crate::opt::HasOpt;
use crate::pm::Pm;
use std::fmt::{self, Display, Formatter};
use std::time::Duration;

// Represents a command from the engine to the GUI.
// TODO: support copyprotection, registration, and custom commands:
// - copyprotection: Used by copyprotected engines.
// - registration [ok | error]: Needed for engines that need a username and or a
//   code to function
// with all the features.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum EngCmd {
    // id name <x>: The name and version of the chess engine, as response to
    // "uci" command..
    IdName(String),
    // id author <x>: The name of the author of the chess engine, as response to
    // "uci" command.
    IdAuthor(String),
    // uciok: Must be sent after the ID and optional options to tell the GUI
    // that the engine has sent all infos and is ready in uci mode.
    UciOk,
    // readyok: This must be sent when the engine has received an "isready"
    // command and has processed all input and is ready to accept new commands
    // now.
    ReadyOk,
    // bestmove <move1> [ponder <move2>]: The engine has stopped searching and
    // found the move <move1> best in this position. The engine can send the
    // move it likes to ponder on. The engine must not start pondering
    // automatically. This command must always be sent if the engine stops
    // searching, also in pondering mode if there is a "stop" command, so for
    // every "go" command a "bestmove" command is needed. Directly before that,
    // the engine should send a final info command with the final search
    // information.
    BestMove { best: Pm, ponder: Option<Pm> },
    // info [opts]: Used by the engine to send information about the engine and
    // its calculations to the GUI. See below for more details.
    Info(Info),
    // option name <id> [opts..]: To tell the engine which options can be
    // changed.
    HasOpt(HasOpt),
}

impl Display for EngCmd {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            EngCmd::UciOk => formatter.write_str("uciok"),
            EngCmd::ReadyOk => formatter.write_str("readyok"),
            EngCmd::IdName(ref name) => write!(formatter, "id name {}", name),
            EngCmd::IdAuthor(ref author) => write!(formatter, "id author {}", author),
            EngCmd::Info(ref info) => info.fmt(formatter),
            EngCmd::HasOpt(ref has_opt) => has_opt.fmt(formatter),
            EngCmd::BestMove { best, ponder } => {
                write!(formatter, "bestmove {}", best)?;
                if let Some(pm) = ponder {
                    write!(formatter, " ponder {}", pm)?;
                }
                Ok(())
            }
        }
    }
}

// Represents the various options to encode the "info" command, when the engine
// wants to send information to the GUI. This should be done whenever one of the
// info has changed. The engine can send only selected infos or mutliple infos
// with one info command, e.g. "info currmove e2e4 currmovenumber 1", "info
// depth 12 nodes 123456 nps 1000000". All infos belonging to the pv should be
// sent together, e.g. "info depth 2 score cp 214 time 1242 nodes 2124 nps 34928
// pv e2e4 e7e5 g1f3". Suggest to send "currmove", "currmovenumber", "currline",
// and "refutation" only after 1 second to avoid too much traffic.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Info {
    // depth <x>: Search depth in plies.
    depth: Option<u16>,

    // seldepth <x>: Selective search depth in plies. If the engine sends
    // "seldepth", there must also be a "depth" present in the same string.
    sel_depth: Option<u16>,

    // node <x>: x nodes searched. The engine should send this info regularly.
    node: Option<u32>,

    // time <x>: The time searched in ms. This should be sent together with the
    // PV.
    time: Option<Duration>,

    // pv <move1> .. <movei>: The best line found.
    pv: Option<Vec<Pm>>,

    // multipv <num>: This for the multipv mode. For the best move/pv add
    // "multipv 1" in the string when you send the pv. In k-best mode always
    // send the all k variants in k strings together.
    multi_pv: Option<MultiPv>,

    // score [opts]: The score from the engine's point of view.
    score: Option<Score>,

    // currmove <move>: Currently searching this move.
    curr_move: Option<Pm>,

    // hashfull <x>: The hashfull is x permill full. The engine should send this
    // info regularly.
    hash_full: Option<u16>,

    // nps <x>: x nodes per second searched. The engine should send this info
    // regularly.
    nodes_per_sec: Option<u32>,

    // tbhits <x>: x positions where found in the endgame table base.
    tb_hits: Option<u32>,

    // sbhits <x>: x positions where found in the shredder endgame databases.
    sb_hits: Option<u32>,

    // cpuload <x>: The CPU usage of the engine is <x> permill.
    cpu_load: Option<u16>,

    // string <str>: Any string <str> which will be displayed by the engine. If
    // there is a string command the rest of the line will be interpreted as
    // <str>.
    string: Option<String>,

    // refutation <move1> <move2> .. <movei>: move1 is refuted by line.
    refutation: Option<Refutation>,

    // currline <cpunr> <move1> .. <movei>: The current line the engine is
    // calculating. <cpnur> is only relevant if more than one CPU is used. See
    // CurrLine for more detaisl.
    curr_line: Option<CurrLine>,
}

impl Display for Info {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "info")?;
        if let Some(depth) = self.depth {
            write!(formatter, " depth {}", depth)?;
        }
        if let Some(sel_depth) = self.sel_depth {
            write!(formatter, " seldepth {}", sel_depth)?;
        }
        if let Some(node) = self.node {
            write!(formatter, " node {}", node)?;
        }
        if let Some(time) = self.time {
            write!(formatter, " time {}", time.as_millis())?;
        }
        if let Some(ref pv) = self.pv {
            write!(formatter, " pv")?;
            for pm in pv {
                write!(formatter, " {}", pm)?;
            }
        }
        if let Some(ref multi_pv) = self.multi_pv {
            write!(formatter, " {}", multi_pv)?;
        }
        if let Some(score) = self.score {
            write!(formatter, " {}", score)?;
        }
        if let Some(curr_move) = self.curr_move {
            write!(formatter, " currmove {}", curr_move)?;
        }
        if let Some(hash_full) = self.hash_full {
            write!(formatter, " hashfull {}", hash_full)?;
        }
        if let Some(nps) = self.nodes_per_sec {
            write!(formatter, " nps {}", nps)?;
        }
        if let Some(tb_hits) = self.tb_hits {
            write!(formatter, " tbhits {}", tb_hits)?;
        }
        if let Some(sb_hits) = self.sb_hits {
            write!(formatter, " sbhits {}", sb_hits)?;
        }
        if let Some(cpu_load) = self.cpu_load {
            write!(formatter, " cpuload {}", cpu_load)?;
        }
        if let Some(ref string) = self.string {
            write!(formatter, " string {}", string)?;
        }
        if let Some(ref refutation) = self.refutation {
            write!(formatter, " {}", refutation)?;
        }
        if let Some(ref curr_line) = self.curr_line {
            write!(formatter, " {}", curr_line)?;
        }
        Ok(())
    }
}

// currline <cpunr> <move1> .. <movei>: Represents the current line the engine
// is calculating. <cpunr> is the number of the cpu if the   engine is running
// on more than one cpu. <cpunr> = 1, 2, 3, etc. If the engien is just using one
// CPU, <cpunr> can be omitted. If <cpunr> is greater than 1, always send all
// k lines in k strings   together. The engine should only send this if the
// option "UCI_ShowCurrLine" is set to true.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CurrLine {
    cpu_id: Option<u16>,
    line: Vec<Pm>,
}

impl Display for CurrLine {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "currline")?;
        if let Some(cpu_id) = self.cpu_id {
            write!(formatter, " {}", cpu_id)?;
        }
        for pm in &self.line {
            write!(formatter, " {}", *pm)?;
        }
        Ok(())
    }
}

// refutation <move1> <move2> .. <movei>: Represents the refutation command.
// Move <move1> is refuted by the line <move2> .. <movei>. The engine should
// only send this if the option "UCI_ShowRefutations" is set to true.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Refutation {
    refuted_move: Pm,

    // The line of moves that refute refuted_move.
    moves: Vec<Pm>,
}

impl Display for Refutation {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "refutation {}", self.refuted_move)?;
        for pm in &self.moves {
            write!(formatter, " {}", *pm)?;
        }
        Ok(())
    }
}

// score cp <x> [mate <y>] [lowerbound] [upperbound]: Represents the score
// option to the info command.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Score {
    // cp <x>: The score from the engine's point of view in centipawns.
    cp: i32,

    // mate <y>: Mate in y moves, not plies. If the engine is getting mated, use
    // negative values for y.
    mate: Option<i16>,

    // If provided, then the score is either a lower or an upper bound.
    bound: Option<ScoreBound>,
}

impl Display for Score {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "score cp {}", self.cp)?;
        if let Some(mate) = self.mate {
            write!(formatter, " {}", mate)?;
        }
        if let Some(bound) = self.bound {
            write!(formatter, " {}", bound)?;
        }
        Ok(())
    }
}

// Represents a lower or an upper score bound.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScoreBound {
    Lower,
    Upper,
}

impl ScoreBound {
    pub fn as_str(&self) -> &'static str {
        match self {
            ScoreBound::Lower => "lowerbound",
            ScoreBound::Upper => "upperbound",
        }
    }
}

impl Display for ScoreBound {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// multipv <num>: Used for representing the multipv command in the multipv mode.
// For the best move/pv add "multipv 1" in the string when you send the pv. In
// k-best mode, should always send the all k variants in k strings together.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MultiPv {
    rank: u64,
    moves: Vec<Pm>,
}

impl Display for MultiPv {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "multipv {}", self.rank)?;
        for pm in &self.moves {
            write!(formatter, " {}", *pm)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn engcmd_id_name() {
        let cmd = EngCmd::IdName("funnychess".into());
        assert_eq!(cmd.to_string().as_str(), "id name funnychess");
    }

    #[test]
    fn engcmd_id_author() {
        let cmd = EngCmd::IdAuthor("Omar S".into());
        assert_eq!(cmd.to_string().as_str(), "id author Omar S");
    }

    #[test]
    fn engcmd_uciok() {
        assert_eq!(EngCmd::UciOk.to_string().as_str(), "uciok");
    }

    #[test]
    fn engcmd_readyok() {
        assert_eq!(EngCmd::ReadyOk.to_string().as_str(), "readyok");
    }

    #[test]
    fn engcmd_bestmove() {
        let best = Pm::from_str("e2e4").unwrap();

        let best_move = EngCmd::BestMove {
            best: best,
            ponder: None,
        };

        assert_eq!(best_move.to_string().as_str(), "bestmove e2e4");

        let best_move = EngCmd::BestMove {
            best: best,
            ponder: Some(Pm::from_str("e7e6").unwrap()),
        };

        assert_eq!(best_move.to_string().as_str(), "bestmove e2e4 ponder e7e6");
    }
}
