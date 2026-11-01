//! Headless-browser tests of the editor. Each test loads the real `web/index.html` and
//! `web/style.css` once, then drives the page only through DOM events and controls.

use super::document;
use std::sync::Once;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
use web_sys::{
    ClipboardEvent, ClipboardEventInit, DataTransfer, Element, HtmlElement, HtmlInputElement,
    HtmlSelectElement, HtmlTextAreaElement, KeyboardEvent, KeyboardEventInit, PointerEvent,
    PointerEventInit,
};

wasm_bindgen_test_configure!(run_in_browser);

const PAGE: &str = include_str!("../../web/index.html");
const STYLE: &str = include_str!("../../web/style.css");

fn element<T: JsCast>(id: &str) -> T {
    document()
        .get_element_by_id(id)
        .unwrap_or_else(|| panic!("missing #{id}"))
        .dyn_into()
        .unwrap_or_else(|_| panic!("#{id} has the wrong type"))
}
fn set_value(id: &str, value: &str) {
    element::<HtmlInputElement>(id).set_value(value);
}
fn click(id: &str) {
    let button = element::<HtmlElement>(id);
    assert!(!button.has_attribute("disabled"), "#{id} is disabled");
    button.click();
}
/// Click a checkbox only when it must change, so its change listener runs as for a user.
fn check(id: &str, on: bool) {
    let input = element::<HtmlInputElement>(id);
    if input.checked() != on {
        input.click();
    }
}
fn tool(name: &str) {
    let input: HtmlInputElement = document()
        .query_selector(&format!("input[name=tool][value={name}]"))
        .unwrap()
        .unwrap_or_else(|| panic!("missing tool {name}"))
        .dyn_into()
        .unwrap();
    input.click();
    assert!(input.checked());
}
fn status() -> String {
    element::<Element>("status")
        .text_content()
        .unwrap_or_default()
}
fn rows(lines: &[&str]) -> String {
    lines.join("\n")
}

/// The artwork text, checked against the ASCII-only contract on every read: printable bytes
/// 32..=126 in equal-width rows separated by LF.
fn art() -> String {
    let text = element::<Element>("art").text_content().unwrap_or_default();
    assert!(
        text.bytes().all(|b| b == b'\n' || (32..=126).contains(&b)),
        "#art holds a byte outside printable ASCII: {text:?}"
    );
    let widths: Vec<usize> = text.split('\n').map(str::len).collect();
    assert!(
        widths.iter().all(|w| *w == widths[0]),
        "#art rows differ in width: {widths:?}"
    );
    text
}
fn grid_size() -> (usize, usize) {
    let text = art();
    (
        text.split('\n').next().unwrap().len(),
        text.split('\n').count(),
    )
}

/// Load the page once, start the editor, then reset to a blank canvas with default tool
/// options. History and the area clipboard carry over between tests, as in one session.
fn blank(width: usize, height: usize) {
    static LOAD: Once = Once::new();
    LOAD.call_once(|| {
        let body = PAGE
            .split_once("<body>")
            .and_then(|(_, rest)| rest.rsplit_once("</body>"))
            .expect("index.html has a body")
            .0;
        let host = document().create_element("div").unwrap();
        // Scripts inserted as HTML do not run, so only `start` loads the editor.
        host.set_inner_html(&format!("<style>{STYLE}</style>{body}"));
        document().body().unwrap().append_child(&host).unwrap();
        super::start().expect("editor starts");
    });
    tool("select-text");
    check("diagram", false);
    check("arrow", true);
    check("onion-skin", false);
    check("border", false);
    set_value("brush", "#");
    set_value("stamp", "");
    element::<HtmlSelectElement>("mode").set_value("blank");
    set_value("width", &width.to_string());
    set_value("height", &height.to_string());
    click("generate");
    assert_eq!(art(), rows(&vec![" ".repeat(width).as_str(); height]));
}

fn pointer(kind: &str, (x, y): (usize, usize)) {
    let (columns, lines) = grid_size();
    let rect = element::<Element>("art").get_bounding_client_rect();
    let init = PointerEventInit::new();
    init.set_bubbles(true);
    init.set_cancelable(true);
    init.set_pointer_id(1);
    init.set_pointer_type("mouse");
    init.set_is_primary(true);
    init.set_button(if kind == "pointermove" { -1 } else { 0 });
    // Aim at the centre of the cell.
    init.set_client_x(
        (rect.left() + (x as f64 + 0.5) * rect.width() / columns as f64).round() as i32,
    );
    init.set_client_y(
        (rect.top() + (y as f64 + 0.5) * rect.height() / lines as f64).round() as i32,
    );
    let event = PointerEvent::new_with_event_init_dict(kind, &init).unwrap();
    element::<Element>("art").dispatch_event(&event).unwrap();
}
/// Press at the first cell, move through the others and release at the last.
fn drag(path: &[(usize, usize)]) {
    pointer("pointerdown", path[0]);
    for point in &path[1..] {
        pointer("pointermove", *point);
    }
    pointer("pointerup", *path.last().unwrap());
}
fn key_with(key: &str, ctrl: bool, shift: bool) -> bool {
    let init = KeyboardEventInit::new();
    init.set_bubbles(true);
    init.set_cancelable(true);
    init.set_key(key);
    init.set_ctrl_key(ctrl);
    init.set_shift_key(shift);
    let event = KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &init).unwrap();
    let art = element::<HtmlElement>("art");
    art.focus().unwrap();
    art.dispatch_event(&event).unwrap();
    event.default_prevented()
}
fn keys(sequence: &[&str]) {
    for key in sequence {
        key_with(key, false, false);
    }
}
fn repeat(key: &str, times: usize) {
    for _ in 0..times {
        key_with(key, false, false);
    }
}
/// Move the keyboard cursor to a cell from the top-left corner.
fn cursor_to((x, y): (usize, usize)) {
    let (columns, lines) = grid_size();
    repeat("ArrowUp", lines);
    repeat("ArrowLeft", columns);
    repeat("ArrowDown", y);
    repeat("ArrowRight", x);
    assert_eq!(
        element::<Element>("cursor-position")
            .text_content()
            .unwrap(),
        format!("Column {}, row {}", x + 1, y + 1)
    );
}
/// Dispatch a clipboard event on the artwork and return the text it left on the clipboard.
fn clipboard(kind: &str, text: &str) -> String {
    let data = DataTransfer::new().unwrap();
    data.set_data("text/plain", text).unwrap();
    let init = ClipboardEventInit::new();
    init.set_bubbles(true);
    init.set_cancelable(true);
    init.set_clipboard_data(Some(&data));
    let event = ClipboardEvent::new_with_event_init_dict(kind, &init).unwrap();
    element::<Element>("art").dispatch_event(&event).unwrap();
    data.get_data("text/plain").unwrap()
}

#[wasm_bindgen_test]
fn pencil_draws_by_pointer_and_keyboard() {
    blank(8, 4);
    tool("pencil");
    drag(&[(1, 0), (4, 0), (4, 2)]);
    assert_eq!(
        art(),
        rows(&[" ####   ", "    #   ", "    #   ", "        "])
    );
    set_value("brush", "*");
    cursor_to((0, 3));
    keys(&[" ", "ArrowRight", "ArrowRight", " "]);
    assert_eq!(
        art(),
        rows(&[" ####   ", "    #   ", "    #   ", "***     "])
    );
}

#[wasm_bindgen_test]
fn eraser_clears_by_pointer_and_keyboard() {
    blank(6, 3);
    tool("fill");
    drag(&[(0, 0)]);
    assert_eq!(art(), rows(&["######"; 3]));
    tool("eraser");
    drag(&[(0, 0), (3, 0)]);
    cursor_to((5, 2));
    keys(&[" ", "ArrowLeft", "ArrowUp", " "]);
    assert_eq!(art(), rows(&["    ##", "#### #", "####  "]));
}

#[wasm_bindgen_test]
fn line_previews_from_its_start_and_draws_by_keyboard() {
    blank(7, 4);
    tool("line");
    set_value("brush", "-");
    // Only the final position counts: the detour through (3, 3) leaves no trace.
    drag(&[(0, 0), (3, 3), (6, 0)]);
    assert_eq!(art(), rows(&["-------", "       ", "       ", "       "]));
    set_value("brush", "|");
    cursor_to((3, 1));
    keys(&[" ", "ArrowDown", "ArrowDown", " "]);
    assert_eq!(art(), rows(&["-------", "   |   ", "   |   ", "   |   "]));
    // Escape abandons a keyboard line without changing the frame.
    keys(&[" ", "ArrowLeft", "Escape"]);
    assert_eq!(art(), rows(&["-------", "   |   ", "   |   ", "   |   "]));
}

#[wasm_bindgen_test]
fn rectangle_draws_by_pointer_and_keyboard() {
    blank(9, 4);
    tool("rectangle");
    // Corners in either order give the same outline.
    drag(&[(3, 2), (0, 0)]);
    set_value("brush", "o");
    cursor_to((5, 1));
    keys(&[
        " ",
        "ArrowRight",
        "ArrowRight",
        "ArrowRight",
        "ArrowDown",
        "ArrowDown",
        " ",
    ]);
    assert_eq!(
        art(),
        rows(&["####     ", "#  # oooo", "#### o  o", "     oooo"])
    );
}

#[wasm_bindgen_test]
fn fill_replaces_the_touching_run_by_pointer_and_keyboard() {
    blank(7, 5);
    tool("rectangle");
    drag(&[(1, 1), (5, 3)]);
    tool("fill");
    set_value("brush", ".");
    drag(&[(3, 2)]);
    assert_eq!(
        art(),
        rows(&["       ", " ##### ", " #...# ", " ##### ", "       "])
    );
    set_value("brush", ":");
    cursor_to((0, 4));
    keys(&[" "]);
    assert_eq!(
        art(),
        rows(&[":::::::", ":#####:", ":#...#:", ":#####:", ":::::::"])
    );
}

#[wasm_bindgen_test]
fn type_writes_rows_by_pointer_keyboard_and_stamp() {
    blank(8, 3);
    tool("type");
    drag(&[(2, 0)]);
    keys(&["H", "i", "x", "Backspace", "Enter", "y", "o"]);
    assert_eq!(art(), rows(&["  Hi    ", "  yo    ", "        "]));
    // The keyboard alone chooses the cell; Stamp text needs no keyboard typing.
    cursor_to((5, 2));
    set_value("stamp", "ok!");
    click("stamp-text");
    assert_eq!(art(), rows(&["  Hi    ", "  yo    ", "     ok!"]));
}

#[wasm_bindgen_test]
fn area_selects_moves_copies_pastes_and_clears() {
    blank(10, 5);
    tool("type");
    drag(&[(1, 1)]);
    keys(&["A", "B", "Enter", "C", "D"]);
    tool("area");
    drag(&[(1, 1), (2, 2)]);
    assert!(!element::<Element>("selection").has_attribute("hidden"));
    assert_eq!(clipboard("copy", ""), "AB\nCD");
    // Dragging inside the selection moves its contents.
    drag(&[(1, 1), (5, 2)]);
    let moved = rows(&[
        "          ",
        "          ",
        "     AB   ",
        "     CD   ",
        "          ",
    ]);
    assert_eq!(art(), moved);
    // Pasting at the keyboard cursor selects the pasted area; Delete clears it.
    cursor_to((7, 0));
    clipboard("paste", "xy");
    assert_eq!(&art()[..10], "       xy ");
    keys(&["Delete"]);
    assert_eq!(art(), moved);
    // Keyboard selection, then cut, then paste with the button and clear with the button.
    cursor_to((5, 2));
    keys(&[" ", "ArrowRight", "ArrowDown", " "]);
    assert_eq!(clipboard("cut", ""), "AB\nCD");
    assert_eq!(art(), rows(&["          "; 5]));
    cursor_to((0, 0));
    click("area-paste");
    assert_eq!(&art()[..21], "AB        \nCD        ");
    click("area-clear");
    assert_eq!(art(), rows(&["          "; 5]));
    // The Copy area button fills the editor clipboard even where the system one is blocked.
    tool("type");
    drag(&[(0, 4)]);
    keys(&["z"]);
    tool("area");
    drag(&[(0, 4), (0, 4)]);
    click("area-copy");
    cursor_to((9, 0));
    click("area-paste");
    assert_eq!(
        art(),
        rows(&[
            "         z",
            "          ",
            "          ",
            "          ",
            "z         "
        ])
    );
}

#[wasm_bindgen_test]
fn diagram_boxes_and_connectors_join_with_plus() {
    blank(21, 7);
    check("diagram", true);
    tool("rectangle");
    set_value("stamp", "CLI");
    drag(&[(0, 0), (6, 2)]);
    set_value("stamp", "LIB");
    drag(&[(14, 0), (20, 2)]);
    // By keyboard, a box below the first.
    set_value("stamp", "OUT");
    cursor_to((0, 4));
    keys(&[" "]);
    repeat("ArrowRight", 6);
    keys(&["ArrowDown", "ArrowDown", " "]);
    tool("line");
    // Ending on the far box's edge keeps the edge; the arrowhead stops before it.
    drag(&[(6, 1), (14, 1)]);
    cursor_to((3, 2));
    keys(&[" ", "ArrowDown", "ArrowDown", " "]);
    assert_eq!(
        art(),
        rows(&[
            "+-----+       +-----+",
            "| CLI +------>| LIB |",
            "+--+--+       +-----+",
            "   v                 ",
            "+-----+              ",
            "| OUT |              ",
            "+-----+              ",
        ])
    );
}

#[wasm_bindgen_test]
fn onion_skin_stays_out_of_the_frame_and_selected_text() {
    blank(6, 2);
    tool("pencil");
    drag(&[(0, 0), (5, 0)]);
    click("add-frame");
    drag(&[(2, 1)]);
    check("onion-skin", true);
    let onion = element::<Element>("onion");
    assert!(!onion.has_attribute("hidden"));
    // The ghost shows only where the current frame is blank.
    assert_eq!(onion.text_content().unwrap(), rows(&["######", "      "]));
    assert_eq!(art(), rows(&["      ", "  #   "]));
    assert_eq!(
        element::<HtmlTextAreaElement>("editor").value(),
        rows(&["      ", "  #   "])
    );
    tool("select-text");
    let selection = web_sys::window().unwrap().get_selection().unwrap().unwrap();
    selection
        .select_all_children(&element::<Element>("grid"))
        .unwrap();
    let selected = String::from(js_sys::Object::to_string(&selection));
    selection.remove_all_ranges().unwrap();
    assert_eq!(selected.matches('#').count(), 1, "{selected:?}");
    // The first frame has no previous frame, so the ghost hides.
    click("previous");
    assert!(onion.has_attribute("hidden"));
}

#[wasm_bindgen_test]
fn undo_and_redo_a_stroke_by_button_and_keyboard() {
    blank(5, 1);
    tool("pencil");
    drag(&[(0, 0), (2, 0)]);
    assert_eq!(art(), "###  ");
    click("undo");
    assert_eq!(art(), "     ");
    click("redo");
    assert_eq!(art(), "###  ");
    assert!(key_with("z", true, false));
    assert_eq!(art(), "     ");
    assert!(key_with("y", true, false));
    assert_eq!(art(), "###  ");
    key_with("z", true, false);
    assert!(key_with("Z", true, true));
    assert_eq!(art(), "###  ");
}

#[wasm_bindgen_test]
fn undo_reverts_a_typing_run_as_one_step() {
    blank(6, 1);
    tool("type");
    drag(&[(0, 0)]);
    keys(&["a", "b", "c"]);
    // Moving the cursor ends the run, so this character is a step of its own.
    keys(&["ArrowRight", "d"]);
    assert_eq!(art(), "abc d ");
    key_with("z", true, false);
    assert_eq!(art(), "abc   ");
    key_with("z", true, false);
    assert_eq!(art(), "      ");
    key_with("y", true, false);
    assert_eq!(art(), "abc   ");
}

#[wasm_bindgen_test]
fn non_ascii_input_never_reaches_the_artwork() {
    blank(4, 2);
    tool("type");
    drag(&[(0, 0)]);
    keys(&["é", "\u{1F600}", "\t", "a"]);
    assert_eq!(art(), rows(&["a   ", "    "]));
    clipboard("paste", "é");
    assert!(status().contains("printable ASCII"), "{}", status());
    set_value("stamp", "café");
    click("stamp-text");
    assert!(status().contains("printable ASCII"), "{}", status());
    tool("pencil");
    set_value("brush", "é");
    drag(&[(0, 1), (3, 1)]);
    assert!(status().contains("one printable ASCII"), "{}", status());
    set_value("brush", "\t");
    drag(&[(0, 1), (3, 1)]);
    assert_eq!(art(), rows(&["a   ", "    "]));
}
