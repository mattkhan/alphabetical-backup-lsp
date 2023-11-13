use crate::with_prev::IteratorWithPrev;
use crate::yield_self::YieldSelf;
use tower_lsp::lsp_types::Range;
use tree_sitter::Node;
use tree_sitter::Parser;
use tree_sitter::Range as TSRange;

const DIR_COMMAND: &str = "d";
const FILE_COMMAND: &str = "f";
const COPY_COMMANDS: [&str; 2] = [DIR_COMMAND, FILE_COMMAND];

#[derive(Clone, Debug)]
pub struct Diagnostic {
    pub range: Range,
    pub message: String,
}

#[derive(Clone, Debug)]
struct Data<'a> {
    command: Command<'a>,
    first_arg: FirstArgument<'a>,
}

#[derive(Clone, Debug)]
struct Command<'a> {
    node: Node<'a>,
    name: String,
}

#[derive(Clone, Debug)]
struct FirstArgument<'a> {
    node: Result<Node<'a>, Diagnostic>,
    value: Result<String, Diagnostic>,
}

fn to_lsp(range: TSRange) -> tower_lsp::lsp_types::Range {
    tower_lsp::lsp_types::Range {
        start: tower_lsp::lsp_types::Position {
            line: u32::try_from(range.start_point.row).unwrap(),
            character: u32::try_from(range.start_point.column).unwrap(),
        },
        end: tower_lsp::lsp_types::Position {
            line: u32::try_from(range.end_point.row).unwrap(),
            character: u32::try_from(range.end_point.column).unwrap(),
        },
    }
}

fn sort_error_message(prev: &String, curr: &String) -> String {
    return format!("First argument `{}` should appear before `{}`. First arguments should be sorted alphabetically within the f(ile) and d(irectory) sections.", curr, prev);
}

pub fn parse(text: &String) -> Result<(), Diagnostic> {
    let source = text.as_bytes();

    let mut parser = Parser::new();
    parser.set_language(tree_sitter_bash::language()).expect("Error loading Bash grammar");
    let tree = parser.parse(&source, None).expect("Failed to parse the script");

    let mut cursor = tree.walk();
    let mut child_cursor = tree.walk();
    let root_node = tree.root_node();

    let command_nodes = root_node
        .children(&mut cursor)
        .filter(|node| node.kind() == "command")
        .filter_map(|node| {
            let command_node = node.child_by_field_name("name").expect("Panic!");
            let command_name = command_node
                .utf8_text(source)
                .expect("Panic!")
                .yield_self(|&name| COPY_COMMANDS.contains(&name).then(|| name))?;

            Some((
                node,
                Command {
                    node: command_node,
                    name: command_name.into(),
                },
            ))
        });

    let command_data = command_nodes.filter_map(|(node, command)| {
        let arg_node = node
            .children_by_field_name("argument", &mut child_cursor)
            .find(|child| {
                child.utf8_text(source).expect("Panic!").yield_self(|&name| !name.starts_with("-"))
            })
            .ok_or(Diagnostic {
                range: to_lsp(command.node.range()),
                message: "Missing first argument.".into(),
            });

        let first_arg = arg_node.clone().and_then(|arg_node| {
            (arg_node.kind() == "word")
                .then(|| arg_node.utf8_text(source).expect("Panic!").into())
                .ok_or(Diagnostic {
                    range: to_lsp(arg_node.range()),
                    message: "First argument must be a string literal.".into(),
                })
        });

        Some(Data {
            command,
            first_arg: FirstArgument {
                node: arg_node,
                value: first_arg,
            },
        })
    });

    let bind = command_data.with_prev().try_fold((), |_, (prev, curr)| {
        prev.first_arg.node.clone()?;
        prev.first_arg.value.clone()?;

        curr.first_arg.node.clone()?;
        curr.first_arg.value.clone()?;

        if prev.command.name != curr.command.name {
            return match prev.command.name == FILE_COMMAND {
                true => Err(Diagnostic {
                    range: to_lsp(prev.command.node.range()),
                    message: "The first f command should appear after the last d command.".into(),
                }),
                false => Ok(()),
            };
        }

        let prev_arg = prev.first_arg.value.unwrap();
        let curr_arg = curr.first_arg.value.unwrap();

        match prev_arg <= curr_arg {
            true => Ok(()),
            false => Err(Diagnostic {
                range: to_lsp(curr.first_arg.node.unwrap().range()),
                message: sort_error_message(&prev_arg, &curr_arg),
            }),
        }
    });
    bind
}
