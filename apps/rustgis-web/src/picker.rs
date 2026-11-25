//! Browser file-picker extension filters, kept in parity with the extensions `ui-egui` accepts.

/// Extensions offered when opening a project (`purpose == "project"`).
pub(crate) const PROJECT_EXTS: &[&str] = &["rgis", "rgisx", "rgispkg", "rgisfield", "qgs", "qgz", "geolibre", "json"];

/// Extensions offered when opening a project item (`purpose == "item"`).
pub(crate) const ITEM_EXTS: &[&str] = &[
    "rgismap",
    "rgismpk",
    "rgispkg",
    "rgislyr",
    "rgisstyle",
    "rgisramp",
    "rgistemplate",
    "rgistask",
    "rgisbookmarks",
    "rgis",
    "rgisx",
    "qml",
    "rgisreport",
    "rgisrpttemplate",
    "rgispres",
    "rgisnb",
    "ipynb",
    "rgisdlpk",
    "rgisfield",
    "json",
    "rgistool",
    "rgislayout",
];

/// Returns the static extension filter list for a named picker `purpose`, when it uses one.
#[cfg_attr(target_arch = "wasm32", allow(dead_code))]
pub(crate) fn filter_extensions(purpose: &str) -> Option<&'static [&'static str]> {
    match purpose {
        "project" => Some(PROJECT_EXTS),
        "item" => Some(ITEM_EXTS),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every project and item extension that `ui-egui` opens via `open_async("project")` or
    /// `open_async("item")` must be selectable in the browser file picker.
    #[test]
    fn browser_picker_extensions_cover_all_project_and_item_formats() {
        let project = filter_extensions("project").unwrap();
        for ext in ["rgis", "rgisx", "rgispkg", "rgisfield", "qgs", "qgz", "geolibre", "json"] {
            assert!(project.contains(&ext), "project picker is missing .{ext}");
        }

        let item = filter_extensions("item").unwrap();
        for ext in [
            "rgismap",
            "rgismpk",
            "rgispkg",
            "rgislyr",
            "rgisstyle",
            "rgisramp",
            "rgistemplate",
            "rgistask",
            "rgisbookmarks",
            "rgis",
            "rgisx",
            "qml",
            "rgisreport",
            "rgisrpttemplate",
            "rgispres",
            "rgisnb",
            "ipynb",
            "rgisdlpk",
            "rgisfield",
            "json",
            "rgistool",
            "rgislayout",
        ] {
            assert!(item.contains(&ext), "item picker is missing .{ext}");
        }
    }
}
