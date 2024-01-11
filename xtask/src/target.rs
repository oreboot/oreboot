use log::{error, trace};
use std::path::{Component, Path};

use crate::starfive;
use crate::sunxi;
use crate::util::project_root;

pub(crate) struct Target {
    vendor_board: Vendor,
    features: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
enum Vendor {
    StarFive(crate::starfive::Board),
    Sunxi(crate::sunxi::Board),
}

impl Target {
    pub(crate) fn execute_command(self, command: &crate::Cli) {
        match self.vendor_board {
            Vendor::Sunxi(sunxi) => sunxi.execute_command(command, self.features),
            Vendor::StarFive(starfive) => starfive.execute_command(command, self.features),
        };
    }
}

pub(crate) fn parse_target(
    cur_path: &Path,
    param_mainboard: Option<&str>,
    param_variant: Option<&str>,
) -> Option<Target> {
    let features = if let Some(variant) = param_variant {
        vec![variant.to_string()]
    } else {
        vec![]
    };
    if let Some((vendor, board)) = parse_target_str(cur_path, param_mainboard) {
        let vendor_board = match (vendor.as_ref(), board.as_ref()) {
            ("sunxi", "nezha") => Vendor::Sunxi(sunxi::Board::Nezha),
            ("starfive", "visionfive1") => Vendor::StarFive(starfive::Board::VisionFive1),
            ("starfive", "visionfive2") => Vendor::StarFive(starfive::Board::VisionFive2),
            _ => return None,
        };
        return Some(Target {
            vendor_board,
            features,
        });
    };
    None
}

// mainboard format: vendor/board
fn parse_target_str(cur_path: &Path, param_mainboard: Option<&str>) -> Option<(String, String)> {
    trace!("parse target string, mainboard: {:?}", param_mainboard);
    if let Some(mainboard_str) = param_mainboard {
        trace!("try parse from parameter mainboard: {:?}", mainboard_str);
        let mut split = mainboard_str.split('/');
        let vendor = if let Some(vendor) = split.next() {
            vendor
        } else {
            trace!("no input vendor");
            return None;
        };
        let board = if let Some(board) = split.next() {
            board
        } else {
            trace!("no input board");
            return None;
        };
        if split.next().is_some() {
            trace!("there is unexpected remaining string");
            return None;
        }
        let input_mainboard_path = project_root()
            .join("src")
            .join("mainboard")
            .join(vendor)
            .join(board);
        if !input_mainboard_path.exists() {
            trace!("path not exist");
            return None;
        }
        return Some((vendor.to_string(), board.to_string()));
    }
    let relative_path = match pathdiff::diff_paths(cur_path, project_root()) {
        Some(path_buf) => path_buf,
        None => {
            error!("project root is relative, this is unreachable");
            return None;
        }
    };
    trace!("current relative path: {:?}", relative_path);
    let mainboard_base_path = Path::new("src/mainboard");
    // note(unwrap): both paths are relative
    let mainboard = pathdiff::diff_paths(relative_path, mainboard_base_path).unwrap();
    trace!("path diff to mainboard folder: {:?}", mainboard);
    let mut components = mainboard.components();
    let vendor_board_from_path = match components.next() {
        Some(Component::Normal(vendor)) => {
            trace!("vendor from path: {:?}", vendor);
            match components.next() {
                Some(Component::Normal(board)) => {
                    trace!("board from path: {:?}", board);
                    Some((vendor, board))
                }
                Some(Component::ParentDir | Component::CurDir) => {
                    trace!("when reading path, not in any subfolder of mainboard folder");
                    None
                }
                None => {
                    trace!("can't decide board from this path");
                    None
                }
                illegal @ Some(Component::Prefix(_) | Component::RootDir) => {
                    trace!("illegal path diffs: {:?}", illegal);
                    unreachable!()
                }
            }
        }
        Some(Component::ParentDir | Component::CurDir) => {
            trace!("when reading vendor, not in any subfolder of mainboard folder");
            None
        }
        None => {
            trace!("can't decide vendor from this path");
            None
        }
        illegal @ Some(Component::Prefix(_) | Component::RootDir) => {
            trace!("illegal path diffs: {:?}", illegal);
            unreachable!()
        }
    };
    if let Some((vendor, board)) = vendor_board_from_path {
        return Some((
            vendor.to_string_lossy().to_string(),
            board.to_string_lossy().to_string(),
        ));
    }
    None
}
