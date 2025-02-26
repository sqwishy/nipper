use crate::Document;
use crate::Selection;
use crate::dom_tree::StrTendril;

impl Document {
    /// Gets the HTML contents of the document. It includes
    /// the text and comment nodes.
    pub fn html(&self) -> StrTendril {
        self.tree.root().html()
    }

    /// Gets the text content of the document.
    pub fn text(&self) -> StrTendril {
        self.tree.root().text()
    }
}

impl<'a> Selection<'a> {
    /// Gets the specified attribute's value for the first element in the
    /// selection. To get the value for each element individually, use a looping
    /// construct such as map method.
    pub fn attr(&self, name: &str) -> Option<StrTendril> {
        self.nodes().first().and_then(|node| node.attr(name))
    }

    /// Works like `attr` but returns default value if attribute is not present.
    pub fn attr_or(&self, name: &str, default: &str) -> StrTendril {
        self.attr(name).unwrap_or_else(|| StrTendril::from(default))
    }

    /// Sets the given attribute to each element in the set of matched elements.
    pub fn set_attr(&mut self, name: &str, val: &str) {
        for node in self.nodes() {
            node.set_attr(name, val);
        }
    }

    /// Removes the named attribute from each element in the set of matched elements.
    pub fn remove_attr(&mut self, name: &str) {
        for node in self.nodes() {
            node.remove_attr(name);
        }
    }

    /// Adds the given class to each element in the set of matched elements.
    /// Multiple class names can be specified, separated by a space via multiple arguments.
    pub fn add_class(&mut self, class: &str) {
        for node in self.nodes() {
            node.add_class(class);
        }
    }

    /// Determines whether any of the matched elements are assigned the
    /// given class.
    pub fn has_class(&self, class: &str) -> bool {
        self.nodes().iter().any(|node| node.has_class(class))
    }

    /// Removes the given class from each element in the set of matched elements.
    /// Multiple class names can be specified, separated by a space via multiple arguments.
    pub fn remove_class(&mut self, class: &str) {
        for node in self.nodes() {
            node.remove_class(class);
        }
    }

    /// Returns the number of elements in the selection object.
    pub fn length(&self) -> usize {
        self.nodes().len()
    }

    /// Is an alias for `length`.
    pub fn size(&self) -> usize {
        self.length()
    }

    /// Is there any matched elements.
    pub fn exists(&self) -> bool {
        self.length() > 0
    }

    /// Gets the HTML contents of the first element in the set of matched
    /// elements. It includes the text and comment nodes.
    pub fn html(&self) -> StrTendril {
        if self.length() > 0 {
            return self.nodes().first().unwrap().html();
        }

        StrTendril::new()
    }

    /// Gets the combined text content of each element in the set of matched
    /// elements, including their descendants.
    pub fn text(&self) -> StrTendril {
        let mut s = StrTendril::new();

        for node in self.nodes() {
            s.push_tendril(&node.text());
        }

        s
    }
}
