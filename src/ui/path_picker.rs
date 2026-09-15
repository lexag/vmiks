// from

use std::{
    fs::{canonicalize, read_dir},
    marker::PhantomData,
    path::{Path, PathBuf},
};

use egui::{
    containers::menu::{MenuButton, MenuConfig},
    PopupCloseBehavior, Response, ScrollArea, TextBuffer, Ui, Widget,
};

/// A widget for picking files.
///
/// It boils down to a text input, next to a button that opens a widget that
/// allows the user to search the local files and directories.
pub struct PathPicker<'input, 'path, S: TextBuffer, I: IconProvider> {
    /// Buffer into which store the input
    input: &'input mut S,
    /// The path to fall back upon when the user inputs an invalid path
    default_path: &'path Path,
    provider: PhantomData<I>,
}

impl<'input, 'path, S: TextBuffer, I: IconProvider> PathPicker<'input, 'path, S, I> {
    /// Creates a new PathPicker widget
    ///
    /// ## Args
    ///
    /// - input: the buffer to use for input storage
    /// - default_path: the path to fall back onto when the user inputs an invalid path
    pub fn new<P: AsRef<Path>>(input: &'input mut S, default_path: &'path P) -> Self {
        Self {
            input,
            default_path: default_path.as_ref(),
            provider: PhantomData::default(),
        }
    }

    /// Internal method that renders the file picker widget
    fn picker_widget(self, ui: &mut Ui) {
        ui.set_max_height(200.);
        ui.set_max_width(200.);
        ScrollArea::vertical().show(ui, |ui| {
            let mut search_path = PathBuf::from(self.input.as_str());
            if !search_path.exists() {
                search_path = self.default_path.to_owned();
            } else if !search_path.is_dir() {
                search_path = search_path.parent().unwrap_or(self.default_path).to_owned();
            }

            if let Ok(search_path) = canonicalize(search_path) {
                if let Some(parent) = search_path.parent() {
                    if ui.button(I::BACK_ICON).clicked() {
                        self.input.replace_with(parent.to_string_lossy().as_ref());
                    }
                }

                if let Ok(iter) = read_dir(search_path) {
                    for ent in iter {
                        if let Ok(ent) = ent {
                            let path = ent.path();

                            let icon = if path.is_file() {
                                I::FILE_ICON
                            } else {
                                I::FOLDER_ICON
                            };

                            if ui
                                .button(format!("{} {}", icon, ent.file_name().display()))
                                .clicked()
                            {
                                self.input.replace_with(path.to_string_lossy().as_ref());
                                if path.is_file() {
                                    ui.close();
                                }
                            }
                        }
                    }
                }
            }
        });
    }
}

impl<S: TextBuffer, I: IconProvider> Widget for PathPicker<'_, '_, S, I> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.horizontal(|ui| {
            ui.text_edit_singleline(self.input);
            MenuButton::new(I::OPEN_ICON)
                .config(
                    MenuConfig::default().close_behavior(PopupCloseBehavior::CloseOnClickOutside),
                )
                .ui(ui, |ui| {
                    self.picker_widget(ui);
                });
        })
        .response
    }
}

/// A simple 'static' trait that provides the [crate::PathPicker] with fitting icons to be used in the GUI
pub trait IconProvider {
    const BACK_ICON: &'static str;
    const FOLDER_ICON: &'static str;
    const FILE_ICON: &'static str;
    const OPEN_ICON: &'static str;
}

/// My [IconProvider] of choice
pub struct DefaultIconProvider;

impl IconProvider for DefaultIconProvider {
    const BACK_ICON: &'static str = "⤴";
    const FOLDER_ICON: &'static str = "📁";
    const FILE_ICON: &'static str = "";
    const OPEN_ICON: &'static str = "📂";
}
