use std::collections;
use std::collections::{HashMap, LinkedList, VecDeque};
use std::env::args;
use std::iter::Iterator;
use crate::grep::args::GrepOptions::*;

struct GrepOrder {
    recursively_search: bool,
    pattern : Vec<str>,
    filter_operations : Vec<F>,
    files : Vec<String>
}

pub enum GrepOptions {
    Default,
    DisplayLineCountMatchingPattern,
    DisplayMatchedLinesNoFilenames,
    IgnoreCase,
    DisplayFilenameListOnly,
    DisplayMatchedLinesAndLineNumbers,
    PrintLinesNotMatchingPattern,
    ExpressionSpecification,
    TakePatternsFromFile,
    MatchRegex,
    MatchWholeWord,
    PrintOnlyMatchedPartsOfMatchingLine,
    PrintSearchedLineNLineAfter,
    PrintSearchedLineNLineBefore,
    SearchRecursively
}

const POSS_GREP_IDS : HashMap<&str, GrepOptions> = HashMap::from( [
    ("c", DisplayLineCountMatchingPattern),
    ("h", DisplayMatchedLinesNoFilenames),
    ("i", IgnoreCase),
    ("l", DisplayFilenameListOnly),
    ("n", DisplayMatchedLinesAndLineNumbers),
    ("v", PrintLinesNotMatchingPattern),
    ("e", ExpressionSpecification),
    ("f", TakePatternsFromFile),
    ("E", MatchRegex),
    ("w", MatchWholeWord),
    ("o", PrintOnlyMatchedPartsOfMatchingLine),
    ("A", PrintSearchedLineNLineAfter),
    "B",
    "C",
    "R"]);


pub(super) fn parse_args(args: &mut VecDeque<String>) -> Result<GrepOrder, Err> {
    // collect
    // get first arg, parse to recursive function if an option
    let &mut ops = LinkedList::new();
    let results = match args[1] {
        String::from("-") => parse_options_rec(args, false, ops),
        Some(..) => Ok((false, LinkedList::from([Default])))
    };

    // after parsing options parse for file or directory location

    // if recursive.. check for directory existence and grab all files
    // if not recursive check for existence of file and then
    todo!()
}

fn is_option(args : &mut VecDeque<String>) -> Result<(bool, String), Err> {
    let option = match args.pop_front() {
        Some(token) => token,
        None => Err("Expected token where there was none during options parsing.")
    };
    // if it is not an option, reach end of options parsing
    if option[0] != "-" {
        Ok((false, option))
    }
    Ok((true, option.clone()))
}

fn parse_options<'a>(args : &mut VecDeque<String>, mut recursive : bool, ops: &'a mut LinkedList<GrepOptions>)
    -> Result<(bool, &'a LinkedList<GrepOptions>), Err> {
    if !is_option(args)?.0 {
        Ok((false, ops))
    }
    parse_options_rec(args, recursive, ops)
}

fn parse_options_rec<'a>(args : &mut VecDeque<String>, mut recursive : bool, ops: &'a mut LinkedList<GrepOptions>)
                     -> Result<(bool, &'a LinkedList<GrepOptions>), Err> {
    // if it is not an option, reach end of options parsing
    let is_option_result = is_option(args)?;
    if !is_option_result.0 {
        Ok((recursive, ops))
    }
    let option = is_option_result.1;
    let &option_enumerated = match POSS_GREP_IDS.get(&*option).clone() {
        Some(token_response) => token_response,
        None => Err(String::from("Couldn't find a corresponding option for token: ").push_str(&*option))
    };
    // if it is an option, match it to an enum using hash table
    ops.push_front(option_enumerated.clone());
    // if it is recursive, tie the recursive latch high
    recursive = option_enumerated == SearchRecursively;
    parse_options_rec(args, recursive, ops)
}