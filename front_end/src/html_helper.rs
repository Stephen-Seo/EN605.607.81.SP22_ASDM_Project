use web_sys::{window, Document, Window};

pub fn get_window_document() -> Result<(Window, Document), String> {
    let window = window().ok_or_else(|| String::from("Failed to get window"))?;
    let document = window
        .document()
        .ok_or_else(|| String::from("Failed to get document"))?;

    Ok((window, document))
}

pub fn append_to_info_text(
    document: &Document,
    id: &str,
    msg: &str,
    limit: u32,
) -> Result<(), String> {
    let info_text = document
        .get_element_by_id(id)
        .ok_or_else(|| format!("Failed to get info_text \"{}\"", id))?;

    let height = info_text.client_height();

    // create the new text to be appended in the text
    let p = document
        .create_element("p")
        .map_err(|e| format!("{:?}", e))?;

    p.set_inner_html(msg);

    // check if scrolled to top
    let at_top: bool = info_text.scroll_top() <= height - info_text.scroll_height();

    // append text to output
    info_text
        .append_with_node_1(&p)
        .map_err(|e| format!("{:?}", e))?;

    while info_text.child_element_count() > limit {
        info_text
            .remove_child(
                &info_text.first_child().ok_or_else(|| {
                    format!("Failed to get first_child() of info_text \"{}\"", id)
                })?,
            )
            .map_err(|e| format!("{:?}", e))?;
    }

    if at_top {
        info_text.set_scroll_top(height - info_text.scroll_height());
    }

    Ok(())
}

pub fn element_append_class(document: &Document, id: &str, class: &str) -> Result<(), String> {
    let element = document
        .get_element_by_id(id)
        .ok_or_else(|| format!("Failed to get element with id \"{}\"", id))?;
    let new_class = format!("{} {}", element.class_name(), class);
    element.set_class_name(&new_class);

    Ok(())
}
