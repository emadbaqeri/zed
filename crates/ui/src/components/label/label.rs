#![allow(missing_docs)]

use gpui::{App, StyleRefinement, Window};

use crate::{prelude::*, LabelCommon, LabelLike, LabelSize, LineHeightStyle};

/// A struct representing a label element in the UI.
///
/// The `Label` struct stores the label text and common properties for a label element.
/// It provides methods for modifying these properties.
///
/// # Examples
///
/// ```
/// use ui::prelude::*;
///
/// Label::new("Hello, World!");
/// ```
///
/// **A colored label**, for example labeling a dangerous action:
///
/// ```
/// use ui::prelude::*;
///
/// let my_label = Label::new("Delete").color(Color::Error);
/// ```
///
/// **A label with a strikethrough**, for example labeling something that has been deleted:
///
/// ```
/// use ui::prelude::*;
///
/// let my_label = Label::new("Deleted").strikethrough(true);
/// ```
#[derive(IntoElement)]
pub struct Label {
    base: LabelLike,
    label: SharedString,
}

impl Label {
    /// Creates a new [`Label`] with the given text.
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::prelude::*;
    ///
    /// let my_label = Label::new("Hello, World!");
    /// ```
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            base: LabelLike::new(),
            label: label.into(),
        }
    }
}

// Style methods.
impl Label {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.base.style()
    }

    gpui::margin_style_methods!({
        visibility: pub
    });
}

impl LabelCommon for Label {
    /// Sets the size of the label using a [`LabelSize`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::prelude::*;
    ///
    /// let my_label = Label::new("Hello, World!").size(LabelSize::Small);
    /// ```
    fn size(mut self, size: LabelSize) -> Self {
        self.base = self.base.size(size);
        self
    }

    fn weight(mut self, weight: gpui::FontWeight) -> Self {
        self.base = self.base.weight(weight);
        self
    }

    /// Sets the line height style of the label using a [`LineHeightStyle`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::prelude::*;
    ///
    /// let my_label = Label::new("Hello, World!").line_height_style(LineHeightStyle::UiLabel);
    /// ```
    fn line_height_style(mut self, line_height_style: LineHeightStyle) -> Self {
        self.base = self.base.line_height_style(line_height_style);
        self
    }

    /// Sets the color of the label using a [`Color`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::prelude::*;
    ///
    /// let my_label = Label::new("Hello, World!").color(Color::Accent);
    /// ```
    fn color(mut self, color: Color) -> Self {
        self.base = self.base.color(color);
        self
    }

    /// Sets the strikethrough property of the label.
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::prelude::*;
    ///
    /// let my_label = Label::new("Hello, World!").strikethrough(true);
    /// ```
    fn strikethrough(mut self, strikethrough: bool) -> Self {
        self.base = self.base.strikethrough(strikethrough);
        self
    }

    /// Sets the italic property of the label.
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::prelude::*;
    ///
    /// let my_label = Label::new("Hello, World!").italic(true);
    /// ```
    fn italic(mut self, italic: bool) -> Self {
        self.base = self.base.italic(italic);
        self
    }

    /// Sets the alpha property of the color of label.
    ///
    /// # Examples
    ///
    /// ```
    /// use ui::prelude::*;
    ///
    /// let my_label = Label::new("Hello, World!").alpha(0.5);
    /// ```
    fn alpha(mut self, alpha: f32) -> Self {
        self.base = self.base.alpha(alpha);
        self
    }

    fn underline(mut self, underline: bool) -> Self {
        self.base = self.base.underline(underline);
        self
    }

    fn text_ellipsis(mut self) -> Self {
        self.base = self.base.text_ellipsis();
        self
    }

    fn single_line(mut self) -> Self {
        self.label = SharedString::from(self.label.replace('\n', "␤"));
        self.base = self.base.single_line();
        self
    }
}

impl RenderOnce for Label {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        self.base.child(self.label)
    }
}
